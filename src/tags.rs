//! Implementation of the 'tags' command

use crate::hubapi::Tag;
use crate::langext::DurationExt;
use crate::options::TagsOptions;

const DEFAULT_OS: Option<&str> = if cfg!(target_os = "linux") {
    Some("linux")
} else if cfg!(target_os = "windows") {
    Some("windows")
} else {
    None
};

const DEFAULT_ARCH: Option<&str> = if cfg!(target_arch = "x86_64") {
    Some("amd64")
} else if cfg!(target_arch = "x86") {
    Some("386")
} else if cfg!(target_arch = "arm") {
    Some("arm")
} else if cfg!(target_arch = "aarch64") {
    Some("arm64")
} else {
    None
};

#[derive(serde::Deserialize, Debug)]
struct Response {
    count: usize,
    results: Vec<Tag>,
}

pub async fn run(options: TagsOptions) -> anyhow::Result<()> {
    macro_rules! row {
        ($($values:tt)*) => {
            println!("{:10} {:8.8} {:6.6} {:15} {}", $($values)*)
        }
    }

    let filter_os;
    let filter_arch;

    if options.current_machine {
        filter_os = DEFAULT_OS;
        filter_arch = DEFAULT_ARCH;
    } else {
        filter_os = options.operating_system.as_deref();
        filter_arch = options.architecture.as_deref();
    }

    let http_client = crate::hubapi::http_client()?;

    let mut pending = options.limit;

    for repository in &options.repositories {
        let slug_prefix = if repository.contains('/') {
            ""
        } else {
            "library/"
        };

        'repository: for page in 1.. {
            let url = format!(
                "https://hub.docker.com/v2/repositories/{}{}/tags/?page={}&page_size={}",
                slug_prefix,
                repository,
                page,
                options.limit.min(50)
            );

            let response: Response = http_client
                .get(&url)
                .header("Accept", "application/json")
                .send()
                .await?
                .json()
                .await?;

            if page == 1 {
                println!("- {} results for {}", response.count, repository);

                row!("SIZE", "OS", "ARCH", "LAST PUSHED", "NAME");
            }

            if response.results.is_empty() {
                break;
            }

            for result in response.results {
                let last_updated = result
                    .last_updated
                    .map(|lp| lp.to_human())
                    .unwrap_or_default();

                for image in &result.images {
                    if pending == 0 {
                        break 'repository;
                    }

                    if filter_os.is_some() && filter_os != Some(image.os.as_str()) {
                        continue;
                    }

                    if filter_arch.is_some() && filter_arch != Some(image.architecture.as_str()) {
                        continue;
                    }

                    pending -= 1;

                    row!(
                        bytesize::to_string(image.size, true),
                        image.os,
                        image.architecture,
                        last_updated,
                        result.name
                    );
                }
            }
        }
    }

    Ok(())
}
