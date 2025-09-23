use std::collections::{BTreeMap};

use crate::state::Lyrics;

pub async fn fetch_lyrics(title: &String, artist: &String) -> Result<Lyrics, Box<dyn std::error::Error>> {
    let url = format!(
        "https://lrclib.net/api/get?track_name={}&artist_name={}",
        urlencoding::encode(&title),
        urlencoding::encode(&artist),
    );
    let client = reqwest::Client::new();

    let response = client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;

    Ok(Lyrics {
        plain: json["plainLyrics"].as_str().map(|s| s.to_string()),
        synced: Some(parse_lyrics_to_map(json["syncedLyrics"].as_str().unwrap_or(""))),
    })
}

pub fn parse_lyrics_to_map(lyrics_string: &str) -> BTreeMap<u32, String> {
    let mut lyrics_map = BTreeMap::new();

    for line in lyrics_string.lines() {
        if let Some(end_of_timestamp) = line.find(']') {
            let timestamp_str = &line[1..end_of_timestamp];
            let lyric_str = line[end_of_timestamp + 1..].trim();

            if let Ok(timestamp_ms) = parse_timestamp_to_ms(timestamp_str) {
                lyrics_map.insert(timestamp_ms, lyric_str.to_string());
            }
        }
    }
    
    lyrics_map
}

fn parse_timestamp_to_ms(timestamp_str: &str) -> Result<u32, String> {
    let parts: Vec<&str> = timestamp_str.split(&[':', '.']).collect();
    
    if parts.len() != 3 {
        return Err("Invalid timestamp format".to_string())
    }

    let minutes = parts[0].parse::<u32>().unwrap();
    let seconds = parts[1].parse::<u32>().unwrap();
    let centiseconds = parts[2].parse::<u32>().unwrap();

    let total_ms = (minutes * 60_000) + (seconds * 1_000) + (centiseconds * 10);
    
    Ok(total_ms)
}