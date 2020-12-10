//! Implementation of the 'show' command

use crate::hubapi::Repository;
use crate::langext::DurationExt;
use crate::options::ShowOptions;

const REPOSITORY_URL: &str = "https://hub.docker.com/v2/repositories/";

pub async fn run(options: ShowOptions) -> anyhow::Result<()> {
    if options.repositories.is_empty() {
        eprintln!("No repositories");
        return Ok(());
    }

    for repository in &options.repositories {
        let repository = get_repository(&repository).await?;
        if options.only_description {
            if let Some(full_description) = repository.full_description {
                println!("{}", full_description);
            }
        } else {
            show_repository(&repository).await?
        }
    }

    Ok(())
}

pub async fn show_repository_by_slug(slug: &str) -> anyhow::Result<()> {
    show_repository(&get_repository(slug).await?).await
}

pub async fn show_repository(repository: &Repository) -> anyhow::Result<()> {
    macro_rules! option_field {
        ($field:ident, $label:literal) => {
            option_field!($field, $label, $field)
        };

        ($field:ident, $label:literal, $render:expr) => {
            if let Some($field) = &repository.$field {
                println!(concat!($label, ": {}"), $render)
            }
        };
    }

    option_field!(namespace, "Namespace");
    option_field!(name, "Name");
    option_field!(description, "Description");
    option_field!(star_count, "Starts");
    option_field!(pull_count, "Pulls");

    option_field!(
        is_automated,
        "Automated",
        if *is_automated { "yes" } else { "no" }
    );

    if let Some(last_updated) = &repository.last_updated {
        println!(
            "Last updated: {} ({})",
            last_updated.format("%F %R %Z"),
            last_updated.to_human()
        );
    }

    if let Some(full_description) = &repository.full_description {
        println!("\n----\n\n{}\n\n----", full_description);
    }

    Ok(())
}

/// Download the repository data.
///
/// If `slug` does not contain a '/', it will be prepended with "library/".
/// This is the expected format by hub.docker.com
async fn get_repository(slug: &str) -> Result<Repository, reqwest::Error> {
    let full_url = if slug.contains('/') {
        format!("{}{}/", REPOSITORY_URL, slug)
    } else {
        format!("{}library/{}/", REPOSITORY_URL, slug)
    };

    let http_client = crate::hubapi::http_client()?;

    http_client
        .get(&full_url)
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await
}

#[test]
fn show_rust_repository() {
    use assert_cmd::prelude::*;
    use std::process::Command;

    let process = {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.args(&["show", "rustlang/rust"]);
        cmd.unwrap()
    };

    let stdout = std::str::from_utf8(&process.stdout).unwrap();

    assert!(stdout.contains("Namespace: rustlang"));
    assert!(stdout.contains("Name: rust"));
    assert!(stdout.contains("---"));
    assert!(process.status.code() == Some(0));
}
