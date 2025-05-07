mod cli;
use cli::CLI;

fn main() {
    let cli = CLI::from_args();

    match cli.get_all_urls() {
        Ok(urls) => {
            println!("Loaded {} URLs:", urls.len());
            for url in urls {
                println!("- {}", url);
            }
        }
        Err(e) => {
            eprintln!("Failed to load URLs: {}", e);
            std::process::exit(1);
        }
    }
}
