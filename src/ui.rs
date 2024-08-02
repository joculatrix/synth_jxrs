use std::sync::{Arc, Mutex};

use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, Sparkline},
    Frame
};

use crate::app::App;

pub fn ui(f: &mut Frame<'_>, data: &[u64]) {
    let oscilloscope = Sparkline::default()
        .block(Block::bordered())
        .data(data)
        .max(100)
        .style(Style::default().blue().on_black());

    f.render_widget(oscilloscope, f.size());
}