mod cli;
use cli::CLI;

fn main() {
    let cli = CLI::from_args();
    println!("{:#?}", cli); // debug print
}
