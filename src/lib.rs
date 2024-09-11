use color_eyre::Result;

mod tui;
mod features;

pub fn run() -> Result<()> {
    tui::tui::run_tui()
}
