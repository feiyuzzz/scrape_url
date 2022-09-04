use std::{fs, env};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: url output");
        return Ok(());
    }

    args.iter().for_each(|arg| {
        println!("{}", arg);
    });

    let url = &args[1];
    let output= &args[2];

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    println!("Writing to file: {}", output);

    fs::write(output, md.as_bytes())?;
    println!("Done!");

    Ok(())
}
