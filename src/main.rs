mod checker;
mod cli;
mod status;
mod worker;

// use checker::check_website;
// use cli::CLI;
use status::WebsiteStatus;
use std::sync::{Arc, Mutex, mpsc};
use worker::{Job, run_worker};

fn main() {
    let cli = Arc::new(cli::CLI::from_args());

    let urls = cli.get_all_urls().expect("Failed to load URLs");

    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (result_tx, result_rx) = mpsc::channel::<WebsiteStatus>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    for id in 0..cli.workers {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();
        run_worker(id, job_rx, result_tx);
    }
    drop(result_tx);

    for url in urls {
        let job = Job {
            url,
            cli_args: Arc::clone(&cli),
        };
        job_tx.send(job).unwrap();
    }
    drop(job_tx);

    // bro i m so tired
    let mut results = Vec::new();
    for result in result_rx {
        println!("{}", result.human_readable());
        results.push(result);
    }
}
