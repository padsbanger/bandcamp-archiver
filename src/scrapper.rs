use scraper::{Html, Selector};
use anyhow::{Context};
use std::fs::File;
use std::io::{self, Write};
use ureq;

pub(crate) struct Scrapper {
    url: String,
}

impl Scrapper {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn fetch_html(&self) -> anyhow::Result<String> {
      let resp = ureq::get(&self.url).call()?;

      let html = resp.into_body().read_to_string()?;
      let document: Html = Html::parse_document(&html);

        let selector = Selector::parse(r#"script[data-tralbum]"#)
            .expect("Invalid CSS selector — this should never happen");

        for script in document.select(&selector) {
            if let Some(data) = script.value().attr("data-tralbum") {
                // Optional: dump raw data for debugging
                Self::dump_to_file(data)
                    .context("Failed to write debug dump to dump.html")?;

                return Ok(data.to_string());
            }
        }

        anyhow::bail!(
            "No <script data-tralbum=...> element found on page.\nURL: {}",
            self.url
        );
    }

    pub fn fetch_disography_html(&self) -> anyhow::Result<String>  {
      let resp = ureq::get(&self.url).call()?;

      let html = resp.into_body().read_to_string()?;
      let document: Html = Html::parse_document(&html);

        let selector = Selector::parse(r#"ol#music-grid"#)
            .expect("Invalid CSS selector — this should never happen");

        for element in document.select(&selector) {
            if let Some(data) = element.value().attr("data-client-items"){
                // Optional: dump raw data for debugging
                Self::dump_to_file(data)
                    .context("Failed to write debug dump to dump.html")?;

                return Ok(data.to_string());
            }
        }




        anyhow::bail!(
            "No <script data-tralbum=...> element found on page.\nURL: {}",
            self.url
        );
    }

    //   let resp = ureq::get(&self.url).call()?;

    //   let html = resp.into_body().read_to_string()?;
    //   let document: Html = Html::parse_document(&html);

    //     let selector = Selector::parse(r#"script[data-tralbum]"#)
    //         .expect("Invalid CSS selector — this should never happen");

    //     for script in document.select(&selector) {
    //         if let Some(data) = script.value().attr("data-tralbum") {
    //             // Optional: dump raw data for debugging
    //             Self::dump_to_file(data)
    //                 .context("Failed to write debug dump to dump.html")?;

    //             return Ok(data.to_string());
    //         }
    //     }

    //     anyhow::bail!(
    //         "No <script data-tralbum=...> element found on page.\nURL: {}",
    //         self.url
    //     );
    // }

    fn dump_to_file(data: &str) -> io::Result<()> {
        let mut file = File::create("album-info.json")?;
        file.write_all(data.as_bytes())?;
        file.write_all(b"\n")?; // just nicer for repeated manual checks
        Ok(())
    }

}
