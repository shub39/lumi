use crate::{lyrics::fetch_lyrics, player::watch_playerctl, state::AppState};

pub mod lyrics;
pub mod player;
pub mod state;

#[tokio::main]
async fn main() {
    // let mut terminal = ratatui::init();
    let mut app_state = AppState {
        song_info: None,
        lyrics: None,
        loading_status: state::LoadingStatus::Idle,
        quit: false,
    };

    loop {
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
                        dbg!(&app_state);
                    }
                    Err(err) => {
                        eprintln!("Failed to fetch lyrics: {}", err);
                        app_state.lyrics = None;
                        app_state.loading_status = state::LoadingStatus::Error(err.to_string());
                    }
                }
            }
        }

        // terminal.draw(draw).expect("Failed to draw frame");
        // if matches!(event::read().expect("Failed to read event"), Event::Key(_)) {
        // break;
        // }
        //
    }

    // ratatui::restore();
}
