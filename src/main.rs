use std::{
    sync::mpsc::{self},
    thread,
};

mod lib;

use lib::{scrape, write_curiosities_to_file, Result};

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
