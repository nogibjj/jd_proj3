use watermark::process_image;
extern crate clap;
use clap::Parser;

// add cli to trigger process image 
#[derive(Parser)]
#[clap(version = "1.0", author = "Jackie Du", about = None)]
struct Args {
    #[clap(short, long)]
    file_name: String,
}

fn main() {
    let args = Args::parse();
    process_image(args.file_name);
}
