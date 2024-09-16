mod curl;

use anyhow::Ok;
use clap::Parser;
use curl::aplookup;
use nu_ansi_term::Color::Red;
use spinners::{Spinner, Spinners};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(required = true, index = 1, help = "The URL to lookup")]
    url: String,
}

fn main() {
    let args = Args::parse();
    let url = &args.url.as_str();

    // Start seraching and showing the spinner
    let mut spinner = Spinner::new(Spinners::Dots2, "Lookup in progress...".into());

    let result = aplookup(url);
    if let Err(err) = result {
        spinner.stop_with_newline();
        eprintln!("{}", Red.paint(err.to_string()));
        std::process::exit(1);
    }

    spinner.stop_with_message("done!".into());

    Ok(()).expect("Failed to exit");
}
