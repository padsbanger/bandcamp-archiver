mod args;
mod scrapper;
mod album;
mod downloader;

use album::Album;
use album::Discography as DiscographyItem;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::Args::parse_args();
    let url = args.url().to_string();
    let download_directory: String = args.destination().to_string();


    println!("Fetching HTML from URL: {}", &url);

    if(url.contains("/album/")) {
         println!("Single album");
        handle_single_album(url, download_directory).await?;
    } else {
        println!("Whole discography");
        handle_discography(url, download_directory).await?;
    }

    Ok(())
}


async fn handle_single_album(url: String, download_directory: String) -> anyhow::Result<()> {
    let scraper = scrapper::Scrapper::new(&url);
    match scraper.fetch_html() {
        Ok(data) => {
            let data: Album = serde_json::from_str(&data)?;
            println!(
                "Will now attempt to download {} tracks into default directory",
                &data.trackinfo.len()
            );
            downloader::create_and_download_album(data, download_directory).await?;
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Error, no album info found : {}", e));
        }
    }
    Ok(())
}

async fn handle_discography(url: String, download_directory: String) -> anyhow::Result<()> {
    let scraper = scrapper::Scrapper::new(&url);
    match scraper.fetch_disography_html() {
        Ok(data) => {
            let discogrphy = serde_json::from_str::<Vec<DiscographyItem>>(&data)?;

            for (i, item) in discogrphy.iter().enumerate() {
                println!("Processing album {}/{}: {} - {}", i, discogrphy.len(), item.title, item.artist);
                handle_single_album(format!("{}{}", url, item.page_url), download_directory.clone()).await?;

                // Here you would typically fetch each album's data and download it
                // For simplicity, this example just prints the album info
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Error, no album info found : {}", e));
        }
    }
    Ok(())
}