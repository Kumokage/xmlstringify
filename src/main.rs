use clap::Parser;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, BufWriter, Write};

#[derive(Parser)]
#[command(version = "0.1.0", about = "Some short", long_about = "Some long")]
struct Cli {
    path: std::path::PathBuf,

    #[arg(short, long)]
    output: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::parse();
    let reader = match File::open(&args.path) {
        Ok(f) => BufReader::new(f),
        Err(e) => {
            panic!("Can't open file due to {}", e)
        }
    };

    let mut writer: Box<dyn Write> = match &args.output {
        Some(output_path) => Box::new(BufWriter::new(File::create(output_path).unwrap())),
        None => Box::new(BufWriter::new(stdout())),
    };

    for line in reader.lines() {
        writer
            .write_all(
                line.unwrap()
                    .replace("<", "&lt;")
                    .replace(">", "&gt;")
                    .as_bytes(),
            )
            .unwrap();
        writer.write_all(b"\n").unwrap();
    }
}
