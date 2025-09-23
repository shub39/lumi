use std::{process::Command};

use crate::state::SongInfo;

pub fn watch_playerctl() -> (SongInfo, u32) {
    let cmd = Command::new("playerctl")
        .arg("metadata")
        .arg("--format")
        .arg("{{title}}|{{artist}}|{{album}}|{{position}}")
        .output()
        .expect("failed to spawn playerctl");

    let (title, artist, album, position) = {
        let buffer = String::from_utf8_lossy(&cmd.stdout);
        let parts: Vec<&str> = buffer.split('|').collect();
        (
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].trim_matches(char::is_control).to_string(),
            parts[3].trim_matches(char::is_control).parse::<u32>().unwrap_or(0) / 1000
        )
    };
    
    (
        SongInfo {
            title,
            artist,
            album,
        },
        position,
    )
}

