use anyhow::Result;
use json_rust_parser::*;
use json_rust_parser::JSONValue;
use clap::Parser as ParserClap;
use std::fs;

#[derive(ParserClap, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    json_input: String,

    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() {
    if let Err(err) = start() {
        eprintln!("Error: {}", err);
    }
}

pub fn start() -> Result<()> {
    let args = Args::parse();

    let json_input = &args.json_input;

    let json: JSONValue = parse_json_file(json_input).expect("unsuccessful parse"); 

    if let Some(output_file) = args.output_file {
        fs::write(output_file, serialize_jsonvalue(&json).as_bytes())
            .expect("unsuccessful parse");
    } else {
        println!("{}", serialize_jsonvalue(&json)); 
    }

    Ok(())
}
