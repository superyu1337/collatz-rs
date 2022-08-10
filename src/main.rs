use clap::{Parser, Subcommand};

mod collatz;
mod collatz_fast;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Default collatz implementation
    Default {
        /// Smallest number in the range to check for
        #[clap(default_value_t = 0, short, long)]
        from: u64,

        /// Highest number in the range to check for
        #[clap(default_value_t = 1_000_000_000, short, long)]
        to: u64,

        /// Amount of threads to use
        #[clap(short, long)]
        threads: Option<usize>
    },
    /// Fast collatz implementation
    Fast {
        /// Smallest number in the range to check for
        #[clap(default_value_t = 0, short, long)]
        from: u64,

        /// Highest number in the range to check for
        #[clap(default_value_t = 1_000_000_000, short, long)]
        to: u64,

        /// Amount of threads to use
        #[clap(short, long)]
        threads: Option<usize>
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Default{from, to, threads} => {
            let cpu_count = std::cmp::min(threads.unwrap_or(num_cpus::get()), (to - from) as usize);

            println!("Running with {} thread(s)...", cpu_count);

            let start_time = std::time::Instant::now();
            let (highest_run, highest_number) = collatz::multithreaded(*from as usize, *to as usize, cpu_count);
            let time_taken = start_time.elapsed();

            println!("Total time: {}s", time_taken.as_nanos() as f64 / 1000.0 / 1000.0 / 1000.0);
            println!("{} took {} iterations to finish!", highest_number, highest_run);
        },
        Commands::Fast{from, to, threads} => {
            let cpu_count = threads.unwrap_or(num_cpus::get());
            println!("Running with {} thread(s)...", cpu_count);

            let start_time = std::time::Instant::now();
            let (highest_run, highest_number) = collatz_fast::multithreaded(*from as usize, *to as usize, cpu_count);
            let time_taken = start_time.elapsed();

            println!("Total time: {}s", time_taken.as_nanos() as f64 / 1000.0 / 1000.0 / 1000.0);
            println!("{} took {} iterations to finish!", highest_number, highest_run);
        }
    }
}