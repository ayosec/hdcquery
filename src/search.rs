//! Implementation of the 'search' command

use std::io::Write;
use std::str::FromStr;

use crate::hubapi::Summary;
use crate::langext::DurationExt;
use crate::options::SearchOptions;

use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

const DEFAULT_SEARCH_URL: &str = "https://hub.docker.com/api/content/v1/products/search";

const PAGES_QUEUE_SIZE: usize = 2;

#[derive(serde::Deserialize, Debug)]
struct SearchResult {
    count: usize,
    summaries: Option<Vec<Summary>>,
}

pub async fn run(options: SearchOptions) -> anyhow::Result<()> {
    let (term_width, term_height) = match terminal_size::terminal_size() {
        Some((terminal_size::Width(w), terminal_size::Height(h))) => (w as usize, h as usize),
        None => (80, 25),
    };

    let show_prompt = atty::is(atty::Stream::Stdout) && atty::is(atty::Stream::Stdin);

    let description_width = term_width - /* column widths */ 5 - 31 - 18 - 7 - 7;

    let limit = options.limit.unwrap_or(usize::MAX);
    let terms = options.terms.join(" ");
    let search_url = options.search_url;

    macro_rules! row {
        ($($values:tt)*) => {
            println!(
                "{:4} {:30.30} {:dw$.dw$} {:>17.17} {:>6.6} {:>6.6}",
                $($values)*,
                dw = description_width
            )
        }
    }

    let (pages_tx, mut pages_rx) = mpsc::channel(PAGES_QUEUE_SIZE);
    tokio::spawn(async move {
        pages_queue(pages_tx, search_url, terms, term_height - 2)
            .await
            .unwrap()
    });

    let mut repositories_found = vec![];

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    'main: while let Some(results) = pages_rx.recv().await {
        let total_rows = results.count;
        let summaries = match results.summaries {
            Some(s) if !s.is_empty() => s,
            _ => break,
        };

        // Print current page.

        if show_prompt || repositories_found.is_empty() {
            row!("", "IMAGE", "DESCRIPTION", "LAST UPDATE", "PULLS", "STARS");
        }

        for summary in summaries {
            row!(
                repositories_found.len() + 1,
                summary.slug,
                first_line(summary.short_description.as_ref(), Some(description_width)),
                summary.updated_at.to_human(),
                first_line(summary.pull_count.as_ref(), None),
                summary.star_count.unwrap_or(0)
            );

            repositories_found.push(summary);

            if repositories_found.len() >= limit {
                break;
            }
        }

        // Get input from user.

        loop {
            if !show_prompt {
                break;
            }

            print!(
                "[Found {} results] <ENTER>: more results | Number and <ENTER>: image details > ",
                total_rows
            );

            std::io::stdout().flush()?;

            let line = stdin.next_line().await?.unwrap_or_default();
            let number: usize = match line.trim() {
                "" => break,

                l => match usize::from_str(l) {
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("{:?}: {}", l, e);
                        continue;
                    }
                },
            };

            let repository = match repositories_found.get(number.wrapping_sub(1)) {
                Some(i) => i,

                None => {
                    eprintln!("Image not found");
                    continue;
                }
            };

            crate::show::show_repository_by_slug(&repository.slug).await?;
            break 'main;
        }

        if repositories_found.len() >= limit {
            break 'main;
        }
    }

    Ok(())
}

/// Extract the first line of `value`.
///
/// If `value` is `None`, returns an empty string.
///
/// If `width` is defined, the text will be wrapped to it.
fn first_line<T>(value: Option<T>, width: Option<usize>) -> String
where
    T: std::fmt::Display,
{
    let value = match value {
        Some(v) => v.to_string(),
        None => return String::new(),
    };

    match (width, value.split('\n').next()) {
        (_, None) => String::new(),

        (None, Some(v)) => v.into(),

        (Some(width), Some(value)) => {
            let mut lines = textwrap::wrap(value, width - 2).into_iter();
            let first_line = lines.next().unwrap_or_default();

            if lines.next().is_some() {
                // Append '…' to the result if there are more than one line.
                format!("{} …", first_line)
            } else {
                first_line.to_string()
            }
        }
    }
}

/// Background task to download pages when the program is waiting for user
/// input.
///
/// The queue is bounde to PAGES_QUEUE_SIZE, so we will not download a lot of
/// unneeded pages.
async fn pages_queue(
    mut pages_tx: mpsc::Sender<SearchResult>,
    search_url: Option<String>,
    terms: String,
    per_page: usize,
) -> anyhow::Result<()> {
    let per_page = format!("{}", per_page);

    let search_url = search_url
        .as_ref()
        .map(|s| &s[..])
        .unwrap_or(DEFAULT_SEARCH_URL);

    let mut num_page: usize = 1;

    let http_client = crate::hubapi::http_client()?;

    loop {
        let np_str = num_page.to_string();

        let result = http_client
            .get(search_url)
            .header("Accept", "application/json")
            .header("Search-Version", "v3")
            .query(&[
                ("type", "image"),
                ("q", terms.as_str()),
                ("page_size", per_page.as_str()),
                ("page", np_str.as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        if pages_tx.send(result).await.is_err() {
            // Receiver is closed
            return Ok(());
        }

        num_page += 1;
    }
}

#[cfg(target_os = "linux")]
#[test]
fn search_apache_repository() {
    use assert_cmd::cargo::CommandCargoExt;
    use rexpect::process::wait::WaitStatus::Exited;
    use rexpect::session::spawn_command;
    use std::process::Command;

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(&["search", "apache"]);
    let mut cmd = spawn_command(cmd, Some(60_000)).unwrap();

    // Find "httpd" repository

    let (_, line) = cmd.exp_regex("\n +[0-9]+ +httpd ").unwrap();
    let repostiory_number = line.split_whitespace().next().unwrap();
    cmd.send_line(repostiory_number).unwrap();

    // Show repository details

    cmd.exp_string("Namespace: library").unwrap();
    cmd.exp_string("Name: httpd").unwrap();
    cmd.exp_string("# Quick reference").unwrap();

    assert!(matches!(cmd.process.wait(), Ok(Exited(_, 0))));
}
