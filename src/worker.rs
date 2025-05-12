use crate::cli::CLI;
use crate::status::WebsiteStatus;
use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};

pub struct Job {
    pub url: String,
    pub cli_args: Arc<CLI>,
}

pub fn run_worker(id: usize, job_rx: Arc<Mutex<Receiver<Job>>>, result_tx: Sender<WebsiteStatus>) {
    std::thread::spawn(move || {
        loop {
            let job = {
                let lock = job_rx.lock().unwrap();
                lock.recv()
            };

            match job {
                Ok(job) => {
                    let result = crate::checker::check_website(
                        &job.url,
                        job.cli_args.timeout_secs,
                        job.cli_args.retries,
                    );
                    result_tx.send(result).unwrap();
                }
                Err(_) => break, // Channel closed
            }
        }

        println!("Workre {id} exiting");
    });
}
