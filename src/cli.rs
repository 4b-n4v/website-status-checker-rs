use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::thread::available_parallelism;

#[derive(Debug)]
pub struct CLI {
    pub file: Option<PathBuf>,
    pub urls: Vec<String>,
    pub workers: usize,
    pub timeout_secs: u64,
    pub retries: u32,
}

impl CLI {
    pub fn from_args() -> Self {
        let mut args = env::args().skip(1);
        let mut file = None;
        let mut urls = Vec::new();
        // https://stackoverflow.com/a/73599198
        let mut workers = available_parallelism().unwrap().get();
        let mut timeout_secs = 5;
        let mut retries = 0;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--file" => {
                    file = args.next().map(PathBuf::from).or_else(|| {
                        eprintln!("Expected path after --file");
                        exit(2);
                    });
                }
                "--workers" => {
                    workers = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
                        eprintln!("Expected number after --workers");
                        exit(2);
                    });
                }
                "--timeout" => {
                    timeout_secs = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
                        eprintln!("Expected seconds after --timeout");
                        exit(2);
                    });
                }
                "--retries" => {
                    retries = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
                        eprintln!("Expected number after --retries");
                        exit(2);
                    });
                }
                _ if arg.starts_with("--") => {
                    eprintln!("Unknown option: {}", arg);
                    exit(2);
                }
                _ => {
                    urls.push(arg);
                }
            }
        }

        if file.is_none() && urls.is_empty() {
            eprintln!(
                "Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]"
            );
            exit(2);
        }

        CLI {
            file,
            urls,
            workers,
            timeout_secs,
            retries,
        }
    }
}
