use std::process::Command;

use crate::state::SongInfo;

pub fn watch_playerctl() -> Option<SongInfo> {
    let cmd = Command::new("playerctl")
        .arg("metadata")
        .arg("--format")
        .arg("{{title}}|{{artist}}|{{album}}")
        .output()
        .expect("failed to spawn playerctl");

    let (title, artist, album) = {
        let buffer = String::from_utf8_lossy(&cmd.stdout);
        let parts: Vec<&str> = buffer.split('|').collect();
        (
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].trim_matches(char::is_control).to_string(),
        )
    };

    Some(SongInfo {
        title,
        artist,
        album,
    })
}
