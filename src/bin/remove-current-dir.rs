use std::{io::Result, path::{PathBuf, Path}};

fn main() -> Result<()> {
    simplelog::TermLogger::init(simplelog::LevelFilter::Trace, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto);
    remove_dir_all::remove_dir_containing_current_executable()?;
    Ok(())
}
