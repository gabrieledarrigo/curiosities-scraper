# Curiosity scraper 🔎

A very naif, multi thread Rust application to scrape data from [curiositadalmondo.it](https://curiositadalmondo.it/).

## Usage

To run the application, you need to have Rust installed. 
Then, run the following command:

```bash
$ cargo run
```
The application will download the data and save it in a file called `curiosities.txt`,   
where each curiosity is separated by a newline.

## Why?

Because I can use curiosities to render my terminal a little funnier:

```bash
$ sort -R curiosities.txt | head -n 1 | cowsay | lolcat
```

<img width="335" alt="cowsay" src="https://github.com/gabrieledarrigo/curiosities-scraper/assets/1985555/5f691328-ab53-4482-beea-84462ddbede6">
