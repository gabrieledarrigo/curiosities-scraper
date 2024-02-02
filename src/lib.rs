use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    ops::RangeInclusive,
    sync::mpsc::{Receiver, Sender},
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

const URL: &str = "https://curiositadalmondo.it/tutte-le-curiosita/?avia-element-paging={}";
pub const CURIOSITIES_FILE: &str = "curiosities.txt";

pub fn fetch_page(page: i32) -> Result<String> {
    let response = reqwest::blocking::get(format!("{}", URL.replace("{}", &page.to_string())))?;
    let text = response.text()?;

    Ok(text)
}

pub fn scrape_curiosity<'a>(text: &'a str) -> Result<Vec<String>> {
    let document = scraper::Html::parse_document(text);
    let selector = scraper::Selector::parse(".entry-title")?;
    let v = document
        .select(&selector)
        .map(|curiosity| curiosity.text().collect::<Vec<_>>().join(""))
        .collect::<Vec<String>>();

    Ok(v)
}

pub fn scrape(range: RangeInclusive<i32>, sender: Sender<Vec<String>>) -> Result<()> {
    println!("Scraping pages {}..{}", range.start(), range.end());

    for page in range {
        let text = fetch_page(page)?;
        let curiosities = scrape_curiosity(&text)?;

        sender.send(curiosities)?;
    }

    Ok(())
}

pub fn write_curiosities_to_file(receiver: Receiver<Vec<String>>) -> Result<()> {
    let file = File::create(CURIOSITIES_FILE)?;
    let mut buffer = BufWriter::new(file);

    for received in receiver {
        println!("Writing {} curiosities to file", received.len());
        buffer.write(received.join("\n").as_bytes())?;
    }

    println!("Flushing buffer");

    buffer.flush()?;
    Ok(())
}
