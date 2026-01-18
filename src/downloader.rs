use super::album::Album;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

pub async fn create_and_download_album(
    album: Album,
    download_directory: impl AsRef<str>,
) -> Result<()> {
    let base_dir = download_directory.as_ref();

    // Extract year safely
    let year = album
        .album_release_date
        .split_whitespace()
        .nth(2)
        .unwrap_or("????");

    // Better sanitization for folder name
    let safe_artist = sanitize_filename(&album.artist);
    let safe_title = sanitize_filename(&album.current.title);

    let album_folder = format!("{} - {} ({})", safe_artist, safe_title, year);
    let full_download_path = Path::new(base_dir).join(&album_folder);

    std::fs::create_dir_all(&full_download_path)
        .context("Failed to create album directory")?;

    println!("Saving album to: {}", full_download_path.display());

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    // Download tracks
    for track in &album.trackinfo {
        let Some(url) = track.file.get("mp3-128") else {
            println!("  ✗ Skipping {} - no mp3-128 stream available", track.title);
            continue;
        };

        let safe_track_title = sanitize_filename(&track.title);
        let filename = full_download_path.join(format!(
            "{:02} - {}.mp3",
            track.track_num, safe_track_title
        ));

        println!(
            "  [{:02}/{:02}]  {} → {}",
            track.track_num,
            album.trackinfo.len(),
            track.title,
            filename.file_name().unwrap().to_string_lossy()
        );

        if let Err(e) = download_file_with_progress(&client, url, &filename).await {
            eprintln!("    → Failed: {}", e);
        }
    }

    // Download cover art
    let cover_path = full_download_path.join("folder.jpg");
    let cover_url = format!("https://f4.bcbits.com/img/a{:010}_16.jpg", album.art_id);

    println!("Downloading cover art...");
    if let Err(e) = download_file_with_progress(&client, &cover_url, &cover_path).await {
        eprintln!("  ! Cover art download failed: {}", e);
    } else {
        println!("  ✓ Cover saved as folder.jpg");
    }

    println!("\n✓ Album download completed: {} - {}", album.current.title, album.artist);

    Ok(())
}

// Helper: remove dangerous characters from filenames
fn sanitize_filename(name: &str) -> String {
    name.replace(|c: char| "<>:\"/\\|?*".contains(c), "_")
        .trim()
        .to_string()
}

// Improved download function (takes Client to reuse connection pool)
async fn download_file_with_progress(
    client: &Client,
    url: &str,
    path: impl AsRef<Path>,
) -> Result<()> {
    let path = path.as_ref();

    let head = client.head(url).send().await?;
    let total_size = head
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let pb: ProgressBar = if total_size > 0 {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                .progress_chars("█▉▊▋▌▍▎▏  "),
        );
        pb.set_message(format!(
            "→ {}",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
        pb
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_message("Downloading... (size unknown)");
        pb.enable_steady_tick(Duration::from_millis(120));
        pb
    };

    let mut response = client.get(url).send().await?;
    let mut file = File::create(path)?;

    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_and_clear();
    Ok(())
}