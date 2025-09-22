use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

use crate::{player::watch_playerctl, state::AppState};

pub mod state;
pub mod player;

fn main() {
    // let mut terminal = ratatui::init();
    let mut app_state = AppState {
        song_info: None,
        lyrics: None,
        loading_status: state::LoadingStatus::Idle,
        quit: false
    };
    
    loop {
        watch_playerctl(&mut app_state);
        // terminal.draw(draw).expect("Failed to draw frame");
        // if matches!(event::read().expect("Failed to read event"), Event::Key(_)) {
            // break;
        // }
    }
    
    // ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World");
    frame.render_widget(text, frame.area());
}
