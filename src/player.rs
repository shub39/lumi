use std::process::Command;
use crate::state::SongInfo;

pub fn watch_playerctl() -> Option<(SongInfo, u32)> {
    let cmd = Command::new("playerctl")
        .arg("metadata")
        .arg("--format")
        .arg("{{title}}|{{artist}}|{{album}}|{{position}}")
        .output()
        .ok()?; // kalau playerctl gagal → None

    let buffer = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
    if buffer.is_empty() {
        return None; // player mati / tidak aktif
    }

    let parts: Vec<&str> = buffer.split('|').collect();
    if parts.len() < 4 {
        return None; // data tidak lengkap
    }

    let title = parts.get(0).unwrap_or(&"").to_string();
    let artist = parts.get(1).unwrap_or(&"").to_string();
    let album = parts.get(2).unwrap_or(&"").to_string();
    let position = parts
        .get(3)
        .unwrap_or(&"0")
        .trim_matches(char::is_control)
        .parse::<u32>()
        .unwrap_or(0)
        / 1000;

    Some((
        SongInfo { title, artist, album },
        position,
    ))
}
