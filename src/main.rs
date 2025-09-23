use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode}, Frame
};

use crate::{
    draw::draw,
    lyrics::fetch_lyrics,
    player::watch_playerctl,
    state::{AppState, Lyrics},
};

pub mod draw;
pub mod lyrics;
pub mod player;
pub mod state;

#[tokio::main]
async fn main() {
    let mut terminal = ratatui::init();
    let mut app_state = AppState {
        song_info: None,
        lyrics: None,
        loading_status: state::LoadingStatus::Idle,
        quit: false,
    };

    while !app_state.quit {
        let saved_app_state = app_state.clone();
        app_state.song_info = watch_playerctl();

        if saved_app_state.song_info != app_state.song_info {
            if let Some(song) = &app_state.song_info {
                app_state.loading_status = state::LoadingStatus::Loading;

                // assuming song has title & artist fields
                match fetch_lyrics(&song.title, &song.artist).await {
                    Ok(lyrics) => {
                        app_state.lyrics = Some(lyrics);
                        app_state.loading_status = state::LoadingStatus::Loaded;

                        // Draw lyrics
                    }
                    Err(err) => {
                        eprintln!("Failed to fetch lyrics: {}", err);
                        app_state.lyrics = None;
                        app_state.loading_status = state::LoadingStatus::Error(err.to_string());
                    }
                }
            }
            terminal
                .draw(|f| draw(f, &app_state))
                .expect("Failed to draw frame");

            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Char('q') => app_state.quit = true,
                        _ => {}
                    }
                }
            }
        }
    }

    ratatui::restore();
}
