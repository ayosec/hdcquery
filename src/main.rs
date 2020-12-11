use gumdrop::Options;

mod hubapi;
mod langext;
mod options;
mod pager;
mod search;
mod show;
mod tags;

use options::Command as C;

fn main() -> anyhow::Result<()> {
    let mut rt = tokio::runtime::Runtime::new()?;
    let options = options::Options::parse_args_default_or_exit();

    match options.command {
        Some(C::Search(opts)) => rt.block_on(search::run(opts))?,
        Some(C::Show(opts)) => rt.block_on(show::run(opts))?,
        Some(C::Tags(opts)) => rt.block_on(tags::run(opts))?,
        None => eprintln!("Missing command. Use --help for more info."),
    }

    Ok(())
}
