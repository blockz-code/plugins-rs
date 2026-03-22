use crate::Output;

use deno_core::{ extension, op2, OpState };

use reqwest::{blocking::Client, blocking::ClientBuilder, header::HeaderMap};
use scraper::{Html, Selector};
use url::Url;



fn http_client() -> Client {
    let headers = HeaderMap::new();
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}


struct Scraper {
    client: Client,
    data: Option<Html>,
    selector: Option<Selector>,
}

impl Scraper {
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            client: http_client(),
            data: None,
            selector: None,
        })
    }

    pub fn get_html(&mut self, url: &str, path: &str) -> crate::Result<()> {
        let uri = Url::parse(url)?.join(path)?;
        let data = self.client.get(uri.as_str()).send()?.text()?;
        self.data = Some(Html::parse_document(&data));
        Ok(())
    }

    pub fn get_element(&mut self, selectors: &str) -> crate::Result<String> {
        self.selector = Some(Selector::parse(selectors).unwrap());
        let html = self.data.as_ref().unwrap();
        let data = html.select(&self.selector.as_ref().unwrap()).next().unwrap().inner_html();
        Ok(data)
    }

    pub fn get_element_with_attr(&mut self, selectors: &str, attr: &str) -> crate::Result<String> {
        self.selector = Some(Selector::parse(selectors).unwrap());
        let html = self.data.as_ref().unwrap();
        let data = html.select(&self.selector.as_ref().unwrap()).next().unwrap().attr(attr).unwrap();
        Ok(data.to_string())
    }
}


#[op2(fast)]
fn create_scraper(
    state: &mut OpState,
) -> std::io::Result<()> {
    state.put(Scraper::new().unwrap());
    Ok(())
}


#[op2(fast)]
fn load_document(
    state: &mut OpState,
    #[string] url: String,
    #[string] path: String
) -> std::io::Result<()> {
    let mut scraper: Scraper = state.take();
    scraper.get_html(&url, &path).unwrap();
    Ok(())
}


#[op2]
#[serde]
fn get_element(
    state: &mut OpState,
    #[string] selectors: String
) -> std::io::Result<Output> {
    let mut scraper: Scraper = state.take();
    let data = scraper.get_element(&selectors).unwrap();
    Ok(Output { data })
}


#[op2]
#[serde]
fn get_element_with_attr(
    state: &mut OpState,
    #[string] selectors: String,
    #[string] attr: String
) -> std::io::Result<Output> {
    let mut scraper: Scraper = state.take();
    let data = scraper.get_element_with_attr(&selectors, &attr).unwrap();
    Ok(Output { data })
}




extension!(
    html_scraper,
    ops = [
        create_scraper,
        load_document,
        get_element,
        get_element_with_attr,
    ],
    esm_entry_point = "plugins-rs:scrape", 
    esm = [
        dir "src/extensions/scrape",
        "plugins-rs:scrape" = "index.js"
    ],
    docs = "Rust Based HTML Scraper", "scraper html from rust"
);





pub fn init() -> deno_core::Extension {
    html_scraper::init()
}









