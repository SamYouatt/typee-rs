use color_eyre::Result;

mod tui;

pub fn run() -> Result<()> {
    tui::tui::run_tui()
}
