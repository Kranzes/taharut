use std::{error::Error, mem, thread::sleep, time::Duration};
mod cli;
use clap::Parser;
use cli::Args;
use log::{info, warn};
use notify_rust::Notification;
use taharut::Exercises;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut exercises = Exercises::new();

    loop {
        let old_exercise = exercises.get().clone();

        match exercises.update() {
            Ok(_) if old_exercise.is_empty() => info!("Successfully updated initial exercises"),
            Err(e) if old_exercise.is_empty() => {
                warn!("Failed to update initial exercises with error: {}", e)
            }
            Ok(_) => {
                for d in exercises.get().difference(&old_exercise) {
                    let message = format!("Name: {} {}\nURL: '{}'", d.author, d.name, d.url);

                    info!("{}", message);

                    Notification::new()
                        .summary("Taharut: New exercise available!")
                        .body(&message)
                        .show()?;
                }
            }
            Err(e) => warn!("Failed to fetch exercises with error: {}", e),
        }

        mem::drop(old_exercise);

        sleep(Duration::from_secs(args.interval as u64 * 60));
    }
}
