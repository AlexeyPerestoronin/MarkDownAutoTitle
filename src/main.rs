use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version="1.0.0.0",
    about="Command Line Tool for auto-adding title to mark-down file")
]
struct Args {
    /// target mark-down file for title addition
    #[arg(short, long)]
    file_name: String,
}

fn main() {
    let arguments = Args::parse();

    // println!("CLI count is {}", arguments.count);
    println!("CLI file-name is {}", arguments.file_name);
}
