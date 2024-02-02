mod tests {
    use curiosities_scraper::{
        fetch_page, scrape, scrape_curiosity, write_curiosities_to_file, CURIOSITIES_FILE,
    };
    use std::thread;

    #[test]
    fn test_fetch_page() {
        let text = fetch_page(1).unwrap();

        assert!(text.is_empty() == false);
    }

    #[test]
    fn test_scrape_curiosity() {
        let text = r#"
            <div class="entry-title">Curiosity 1</div>
            <div class="entry-title">Curiosity 2</div>
            <div class="entry-title">Curiosity 3</div>
        "#;

        let curiosities = scrape_curiosity(text).unwrap();
        assert_eq!(
            curiosities,
            vec!["Curiosity 1", "Curiosity 2", "Curiosity 3"]
        );
    }

    #[test]
    fn test_scrape() {
        let range = 1..=1;
        let (sender, receiver) = std::sync::mpsc::channel::<Vec<String>>();

        scrape(range, sender).unwrap();

        let received = receiver.iter().collect::<Vec<_>>();
        assert_eq!(received.len(), 1);
    }

    #[test]
    fn test_write_curiosities_to_file() {
        let (sender, receiver) = std::sync::mpsc::channel::<Vec<String>>();

        let handle = thread::spawn(move || {
            sender
                .clone()
                .send(vec!["Curiosity 1".to_string(), "Curiosity 2".to_string()])
                .unwrap()
        });

        write_curiosities_to_file(receiver).unwrap();

        handle.join().unwrap();

        let text = std::fs::read_to_string(CURIOSITIES_FILE).unwrap();
        assert_eq!(text, "Curiosity 1\nCuriosity 2");
    }
}
