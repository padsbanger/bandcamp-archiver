mod args;
mod scrapper;
mod album;

use album::Album;

fn main() -> anyhow::Result<()> {
    let url = args::Args::parse_args().url().to_string();

    let scraper = scrapper::Scrapper::new(&url);
    println!("Fetching HTML from URL: {}", &url);

    match scraper.fetch_html() {
        Ok(data) => {
            let data: Album = serde_json::from_str(&data)?;
            println!("Fetched album data successfully:{} - {}", data.current.title, data.artist);
            println!("Will now attempt to download {} tracks into default directory", data.trackinfo.len());
            
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Error, no album info found."));
        }
    }





    // fs::write("dump.html", &doc.html())?;



    Ok(())
}
