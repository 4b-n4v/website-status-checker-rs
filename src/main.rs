mod checker;
mod cli;
mod status;
mod worker;

use status::WebsiteStatus;
use std::sync::{Arc, Mutex, mpsc};
use worker::{Job, run_worker};

fn main() {
    let cli = Arc::new(cli::CLI::from_args());

    let urls = cli.get_all_urls().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

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

    let mut results = Vec::new();
    for result in result_rx {
        println!("{}", result.human_readable());
        results.push(result);
    }

    if let Err(e) = write_json(&results) {
        eprintln!("Failed to write status.json: {}", e);
    }
}

use std::fs::File;
use std::io::Write;

fn write_json(results: &[WebsiteStatus]) -> std::io::Result<()> {
    let mut file = File::create("status.json")?;
    writeln!(file, "[")?;

    for (i, result) in results.iter().enumerate() {
        writeln!(file, "{}", result.to_json_object())?;
        if i != results.len() - 1 {
            writeln!(file, ",")?;
        }
    }

    writeln!(file, "]")?;
    Ok(())
}
