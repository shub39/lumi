use std::{process::{Command, Stdio}};


use crate::state::{AppState, SongInfo};

pub fn watch_playerctl(app_state: &mut AppState) {
    let cmd = Command::new("playerctl")
        .arg("metadata")
        .arg("--format")
        .arg("{{title}}|{{artist}}|{{album}}")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to spawn playerctl");
    
    let (title, artist, album) = {
        let buffer = String::from_utf8_lossy(&cmd.stdout);
        let parts: Vec<&str> = buffer.split('|').collect();
        (parts[0].to_string(), parts[1].to_string(), parts[2].trim_matches(char::is_control).to_string())
    };
    
    let new_song_info = SongInfo {
        title,
        artist,
        album,
    };
    
    if app_state.song_info != Some(new_song_info.clone()) {
        app_state.song_info = Some(new_song_info);
        dbg!(app_state);
    }
}
