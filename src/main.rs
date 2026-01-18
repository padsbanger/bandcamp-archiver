mod args;
mod scrapper;
mod album;
mod downloader;

use album::Album;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = args::Args::parse_args().url().to_string();

    let scraper = scrapper::Scrapper::new(&url);
    println!("Fetching HTML from URL: {}", &url);

    match scraper.fetch_html() {
        Ok(data) => {
            let  data: Album = serde_json::from_str(&data)?;
                println!(
                    "Will now attempt to download {} tracks into default directory",
                    &data.trackinfo.len()
                );
            downloader::create_and_download_album(data).await?;
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Error, no album info found."));
        }
    }







    // fs::write("dump.html", &doc.html())?;



    Ok(())
}
