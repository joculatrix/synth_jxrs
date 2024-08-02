use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, Sparkline},
    Frame
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let osc_data = &app.osc_data(0);

    let oscilloscope = Sparkline::default()
        .block(Block::bordered())
        .data(osc_data)
        .max(100)
        .style(Style::default().blue().on_black());

    f.render_widget(oscilloscope, f.size());
}