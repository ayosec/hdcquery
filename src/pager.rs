//! Run an external program as a pager (like "less")

use std::env;
use std::process::{Command, Stdio};

pub const PAGER_ENV: &str = "HDC_PAGER";

const DEFAULT_PAGER: &str = if cfg!(windows) { "more" } else { "pager -F" };

pub fn command(repository: Option<&str>) -> Option<Command> {
    if !atty::is(atty::Stream::Stdout) {
        return None;
    }

    let pager_var = env::var(PAGER_ENV);
    let mut pager_args = pager_var
        .as_deref()
        .unwrap_or(DEFAULT_PAGER)
        .split_whitespace();

    let mut cmd = Command::new(pager_args.next()?);

    for arg in pager_args {
        cmd.arg(arg);
    }

    cmd.stdin(Stdio::piped());
    cmd.env("HDCQUERY_VERSION", env!("CARGO_PKG_VERSION"));
    repository.map(|r| cmd.env("HDCQUERY_REPOSITORY", r));

    Some(cmd)
}
