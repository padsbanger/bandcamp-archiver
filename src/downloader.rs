use super::album::Album;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::fs::File;
use std::io::{self, Write};
use std::time::Duration;

pub async fn create_and_download_album(album: Album) -> Result<()> {
    let tracks = album.trackinfo;

    for track in &tracks {
        let url = track.file.get("mp3-128").unwrap();
        let filename = format!(
            "downloads/{}. {}.mp3",
            track.track_num,
            track.title.replace("/", "_")
        );

        println!(
            "Downloading {} {}  Total: {}/{}...",
            track.track_num,
            track.title,
            track.track_num,
            tracks.len()
        );
        download_file_with_progress(url, &filename.to_string()).await?;
    }

    println!("Dowladoing cover art...");
    let art_url = format!("https://f4.bcbits.com/img/a{}.jpg", album.art_id);
    download_file_with_progress(&art_url, "downloads/folder.jpg").await?;

    println!("✓ Download completed!");

    println!(
        "Fetched album data successfully: {} - {}",
        album.current.title, album.artist
    );

    // Placeholder for future album download logic
    Ok(())
}

pub async fn download_file_with_progress(url: &str, path: &str) -> Result<()> {
    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    // Make HEAD request to get content length
    let head = client.head(url).send().await?;
    let total_size = head
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|len| len.to_str().ok())
        .and_then(|len| len.parse::<u64>().ok())
        .unwrap_or(0);

    // Create progress bar
    let pb = if total_size > 0 {
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
            .progress_chars("#>-"));
        pb.set_message(format!(
            "Downloading {}",
            url.split('/').last().unwrap_or("file")
        ));
        pb
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_message("Downloading... (size unknown)");
        pb.enable_steady_tick(Duration::from_millis(120));
        pb
    };

    // Actual download
    let mut response = client.get(url).send().await.context("Failed to get file")?;

    let mut file = File::create(path).context("Failed to create output file")?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        let new = min(downloaded + chunk.len() as u64, total_size);
        downloaded = new;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("✓ Download completed");
    Ok(())
}
