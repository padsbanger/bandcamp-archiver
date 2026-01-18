use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use super::album::Album;  // your Album struct

pub async fn create_and_download_album(
    album: Album,
    download_directory: impl AsRef<str>,
) -> Result<()> {
    let base_dir = download_directory.as_ref();

    let year = album
        .album_release_date
        .split_whitespace()
        .nth(2)
        .unwrap_or("????");

    let safe_artist = sanitize_filename(&album.artist);
    let safe_title = sanitize_filename(&album.current.title);

    let album_folder = format!("{} - {} ({})", safe_artist, safe_title, year);
    let full_path = Path::new(base_dir).join(&album_folder);

    tokio::fs::create_dir_all(&full_path)
        .await
        .context("Failed to create album directory")?;

    println!("Saving to: {}", full_path.display());

    let client = Arc::new(
        Client::builder()
            .timeout(Duration::from_secs(35))
            .build()?,
    );

    let mp = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("█▉▊▋▌▍▎▏  ");

    // Usually optimal range: 4–8 concurrent downloads
    let max_concurrent = 6;
    let semaphore = Arc::new(Semaphore::new(max_concurrent));

    let mut tasks = JoinSet::new();

    for track in album.trackinfo {
        let Some(url) = track.file.get("mp3-128").cloned() else {
            println!("  ✗ Skipping  {:02} - {}  (no mp3-128)", track.track_num, track.title);
            continue;
        };

        let safe_track = sanitize_filename(&track.title);
        let filename = format!("{:02} - {}.mp3", track.track_num, safe_track);
        let track_path = full_path.join(&filename);

        let pb = mp.add(ProgressBar::new(0));
        pb.set_style(sty.clone());
        pb.set_message(format!("{:02} - {}", track.track_num, track.title));

        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);

        tasks.spawn(async move {
            let _permit = semaphore.acquire().await.context("Semaphore acquire failed")?;

            // Try to get content length
            let total = match client.head(&url).send().await {
                Ok(head) => head.content_length().unwrap_or(0),
                Err(_) => 0,
            };
            pb.set_length(total);

            let mut resp = client
                .get(&url)
                .send()
                .await
                .context("Failed to start download")?;

            let mut file = TokioFile::create(&track_path)
                .await
                .context("Cannot create track file")?;

            let mut downloaded: u64 = 0;

            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk).await?;
                downloaded += chunk.len() as u64;
                pb.set_position(downloaded);
            }

            pb.finish_with_message(format!("{:02} - {} ✓", track.track_num, track.title));
            Ok::<(), anyhow::Error>(())
        });
    }

    // Wait for all downloads + collect errors
    let mut errors = Vec::new();
    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            errors.push(format!("Task panicked: {}", e));
        } else if let Err(e) = res.unwrap() {
            errors.push(format!("Download failed: {}", e));
        }
    }

    mp.clear()?;

    // Cover art (single sequential download)
    let cover_path = full_path.join("folder.jpg");
    let cover_url = format!("https://f4.bcbits.com/img/a{:010}_16.jpg", album.art_id);

    println!("Downloading cover art...");
    if let Err(e) = download_cover(&client, &cover_url, &cover_path).await {
        eprintln!("  ! Cover download failed: {}", e);
    } else {
        println!("  ✓ Cover saved");
    }

    if !errors.is_empty() {
        eprintln!("\nSome downloads failed:");
        for err in errors {
            eprintln!("  • {}", err);
        }
    }

    println!("\n✓ Album processing finished: {} - {}", album.current.title, album.artist);

    Ok(())
}

async fn download_cover(client: &Client, url: &str, path: &Path) -> Result<()> {
    let mut resp = client.get(url).send().await?;
    let mut file = TokioFile::create(path).await?;

    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk).await?;
    }

    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if "<>:\"/\\|?*".contains(c) {
                '_'
            } else {
                c
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}