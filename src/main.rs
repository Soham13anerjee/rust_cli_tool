// Convert Mardown to HTML -> a Markdown parser using rustc CLAP
use clap::Parser;
use maud::{html, Markup, DOCTYPE};
use pulldown_cmark::{Parser as MdParser, Options, html};
use std::{fs,path::PathBuf};
#[derive(Parser,Debug)]

struct Args {
    /// Input markdown file
    #[arg(short, long)]
    input: PathBuf,

    /// Output HTML file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() { 
    let args = Args::parse();
    let markdown_input = fs::read_to_string(&args.input)
        .expect("Failed to read input file");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = MdParser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let final_html:Markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Converted Markdown" }
            }
            body {
                (maud::PreEscaped(&html_output))
            }
        }
    };
    match &args.output {
        Some(output_path) => fs::write(output_path, final_html.into_string())
            .expect("Failed to write output file"),
        None => {
            println!("Path not provided. Please specify an output file using the -o or --output flag.");
        }
    }
}
