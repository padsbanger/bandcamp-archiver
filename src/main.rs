mod args;
mod scrapper;

fn main() -> anyhow::Result<()> {
    let url = args::Args::parse_args().url().to_string();

    let scraper = scrapper::Scrapper::new(&url);
    println!("Fetching HTML from URL: {}", &url);

    match scraper.fetch_html() {
        Ok(data) => {
            println!("Fetched data: {}", data);
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Error, no album info found."));
        }
    }





    // fs::write("dump.html", &doc.html())?;



    Ok(())
}
