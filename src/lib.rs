use color_eyre::Result;

mod features;
mod tui;

pub fn run() -> Result<()> {
    tui::tui::run_tui()
}
