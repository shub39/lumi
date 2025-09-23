use crate::{
    lyrics::fetch_lyrics,
    player::watch_playerctl,
    state::{AppState, LoadingStatus, Lyrics},
};

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, app_state: &AppState) {
    let size = frame.area();

    let text = if let Some(lyrics) = &app_state.lyrics {
        lyrics
            .plain
            .clone()
            .unwrap_or_else(|| "No lyrics found".to_string())
    } else {
        match &app_state.loading_status {
            LoadingStatus::Loading => "Loading lyrics...".to_string(),
            LoadingStatus::Idle => "Idle".to_string(),
            LoadingStatus::Loaded => "Lyrics loaded".to_string(), // Shouldn’t really hit here if lyrics is None
            LoadingStatus::Error(err) => format!("Error: {}", err),
        }
    };

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Lyrics").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, size);
}
