use std::sync::{Arc, Mutex};

use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Style, Stylize}, widgets::{Block, Sparkline}, Frame
};

use crate::{app::App, synth};

pub fn ui(f: &mut Frame<'_>, app: &mut App) {
    // DIVIDE FRAME
    let constraints: [Constraint; synth::NUM_OSCS] = core::array::from_fn(|_| {
        Constraint::Percentage(100 / synth::NUM_OSCS as u16)
    });

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(f.size());

    // OSCILLATOR BLOCKS

    for i in 0..synth::NUM_OSCS {
        let oscilloscope = Sparkline::default()
            .block(Block::bordered())
            .data(app.osc_data(i))
            .max(100)
            .style(Style::default().blue().on_black());
        f.render_widget(oscilloscope, chunks[i]);
    }
}