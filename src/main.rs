use color_eyre::Result;
use typee::run;

fn main() -> Result<()> {
    color_eyre::install()?;

    run()?;

    Ok(())
}
