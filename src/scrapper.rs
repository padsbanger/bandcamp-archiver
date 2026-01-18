struct Scrapper {
    url: String,
}

impl Scrapper {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    fn fetch_html(&self) -> anyhow::Result<String> {
        let resp = ureq::get(&self.url).call()?;
        let html = resp.into_body().read_to_string()?;
        Ok(html)
    }
}
