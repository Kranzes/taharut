#[derive(clap::Parser)]
pub struct Args {
    /// How often (in minutes) to check for new exercises
    #[arg(long, default_value_t = 180)]
    pub interval: u16,
}
