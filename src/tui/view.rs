use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use super::app_model::AppModel;

pub fn view(_model: &AppModel, frame: &mut Frame) {
    let typee = Paragraph::new("Typee");

    frame.render_widget(typee, frame.area());
}
