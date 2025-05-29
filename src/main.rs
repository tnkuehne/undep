use clap::Parser;

/// A simple CLI tool example
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name to greet
    #[arg(short, long, default_value = "World")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    /// Make the greeting uppercase
    #[arg(short, long)]
    uppercase: bool,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Greeting '{}' {} time(s)", args.name, args.count);
        if args.uppercase {
            println!("Using uppercase formatting");
        }
    }

    for i in 1..=args.count {
        let greeting = format!("Hello, {}!", args.name);
        let output = if args.uppercase {
            greeting.to_uppercase()
        } else {
            greeting
        };
        
        if args.verbose {
            println!("[{}] {}", i, output);
        } else {
            println!("{}", output);
        }
    }
}