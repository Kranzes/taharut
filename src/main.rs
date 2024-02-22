use std::{error::Error, mem, thread::sleep, time::Duration};
mod cli;
use clap::Parser;
use cli::Args;
use log::{info, warn};
use notify_rust::Notification;
use taharut::Documents;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut docs = Documents::new();

    loop {
        let old_docs = docs.get().clone();

        match docs.update() {
            Ok(_) if old_docs.is_empty() => info!("Successfully updated initial documents"),
            Err(e) if old_docs.is_empty() => {
                warn!("Failed to update initial documents with error: {}", e)
            }
            Ok(_) => {
                for d in docs.get().difference(&old_docs) {
                    let message = format!("Name: {} {}\nURL: '{}'", d.author, d.name, d.url);

                    info!("{}", message);

                    Notification::new()
                        .summary("Taharut: New document available!")
                        .body(&message)
                        .show()?;
                }
            }
            Err(e) => warn!("Failed to fetch documents with error: {}", e),
        }

        mem::drop(old_docs);

        sleep(Duration::from_secs(args.interval as u64 * 60));
    }
}
