use std::{fs::File, thread::sleep, time::Duration};
mod cli;
use clap::Parser;
use cli::Args;
use directories::ProjectDirs;
use log::{error, info, warn};
use notify_rust::Notification;
use taharut::Exercises;

fn main() {
    let args = Args::parse();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let project_dirs = ProjectDirs::from("com", clap::crate_authors!(), clap::crate_name!())
        .unwrap_or_else(|| {
            error!("Failed to determine project directory");
            std::process::exit(1);
        });

    if let Err(e) = std::fs::create_dir_all(project_dirs.cache_dir()) {
        error!("Failed to create cache directory: {}", e);
        std::process::exit(1);
    }

    let mut exercises = Exercises::new(
        File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(project_dirs.cache_dir().join("data.json"))
            .unwrap_or_else(|e| {
                error!("Failed to open data file: {}", e);
                std::process::exit(1);
            }),
    );

    loop {
        let old_exercise = exercises.get().clone();

        match exercises.update() {
            Ok(_) if old_exercise.is_empty() => info!("Successfully updated initial exercises"),
            Err(e) if old_exercise.is_empty() => {
                warn!("Failed to update initial exercises with: {}", e)
            }
            Ok(_) => {
                for d in exercises.get().difference(&old_exercise) {
                    let message = format!("Name: {} {}\nURL: '{}'", d.author, d.name, d.url);

                    info!("{}", message);

                    if let Err(e) = Notification::new()
                        .summary("Taharut: New exercise available!")
                        .body(&message)
                        .timeout(0)
                        .show()
                    {
                        warn!("Failed to show notification: {}", e);
                    }
                }
            }
            Err(e) => warn!("Failed to fetch exercises: {}", e),
        }

        drop(old_exercise);

        sleep(Duration::from_secs(args.interval as u64 * 60));
    }
}
