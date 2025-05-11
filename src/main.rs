mod checker;
mod cli;
mod status;

use checker::check_website;
use cli::CLI;

fn main() {
    let cli = CLI::from_args();
    let urls = cli.get_all_urls().expect("Failed to load URLs");

    for url in urls {
        let result = check_website(&url, cli.timeout_secs, cli.retries);
        println!("{}", result.human_readable());
    }
}
