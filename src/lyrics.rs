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
        synced: json["syncedLyrics"].as_str().map(|s| s.to_string()),
    })
}
