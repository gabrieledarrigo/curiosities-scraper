use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    ops::RangeInclusive,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const URL: &str = "https://curiositadalmondo.it/tutte-le-curiosita/?avia-element-paging={}";
const CURIOSITIES_FILE: &str = "curiosities.txt";

fn fetch_page(page: i32) -> Result<String> {
    let response = reqwest::blocking::get(format!("{}", URL.replace("{}", &page.to_string())))?;
    let text = response.text()?;

    Ok(text)
}

fn scrape_curiosity<'a>(text: &'a str) -> Result<Vec<String>> {
    let document = scraper::Html::parse_document(text);
    let selector = scraper::Selector::parse(".entry-title")?;
    let v = document
        .select(&selector)
        .map(|curiosity| curiosity.text().collect::<Vec<_>>().join(""))
        .collect::<Vec<String>>();

    Ok(v)
}

fn scrape(range: RangeInclusive<i32>, sender: Sender<Vec<String>>) -> Result<()> {
    println!("Scraping pages {}..{}", range.start(), range.end());

    for page in range {
        let text = fetch_page(page)?;
        let curiosities = scrape_curiosity(&text)?;

        sender.send(curiosities)?;
    }

    Ok(())
}

fn write_curiosities_to_file(receiver: Receiver<Vec<String>>) -> Result<()> {
    let file = File::create(CURIOSITIES_FILE)?;
    let mut buffer = BufWriter::new(file);

    for received in receiver {
        println!("Writing {} curiosities to file", received.len());

        buffer.write(received.join("\n").as_bytes())?;
    }

    buffer.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let cpus = num_cpus::get();

    let total_pages = 205;
    let pages_per_cpu = total_pages / cpus;

    let mut handlers = vec![];
    let (sender, receiver) = mpsc::channel::<Vec<String>>();

    for cpu in 0..cpus {
        let start_page = cpu * pages_per_cpu + 1;
        let end_page = if cpu == cpus - 1 {
            total_pages
        } else {
            (cpu + 1) * pages_per_cpu
        };

        let range = start_page as i32..=end_page as i32;
        let sender = sender.clone();

        let handler = thread::spawn(move || {
            scrape(range, sender).unwrap();
        });

        handlers.push(handler);
    }

    drop(sender);

    write_curiosities_to_file(receiver)?;

    for handler in handlers {
        handler.join().unwrap();
    }
    Ok(())
}
