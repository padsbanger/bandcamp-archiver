mod args;

use scraper::{Html, Selector};
use std::fs::OpenOptions;
use std::io::Write;
use ureq;

use std::fs;

fn main() -> anyhow::Result<()> {
    let url = args::Args::parse_args().url().to_string();

    dbg!(&url);
    let resp = ureq::get(&url).call()?;

    let html = resp.into_body().read_to_string()?;
    let doc = Html::parse_document(&html);

    let scripts = Selector::parse("head script").unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("dump.html")?;

    // writeln!(file, "{}", doc.html())?;

    for script in doc.select(&scripts) {
        if script.attr("data-tralbum").is_some() {
            writeln!(file, "{}", script.attr("data-tralbum").unwrap())?;
            dbg!(script.text());
        }
    }
    // dbg!(&scripts);

    // fs::write("dump.html", &doc.html())?;

    Ok(())
}
