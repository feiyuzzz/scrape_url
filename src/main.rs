use std::{fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut url = String::new();
    let mut output =  String::new();

    println!("Pls input the url. example: http://www.rust-lang.org/");
    io::stdin()
    .read_line(&mut url)?;

    println!("Pls input the output name. example: rust.md");
    io::stdin()
    .read_line(&mut output)?;

    let url = url.trim_end();
    let output = output.trim_end();

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    println!("Writing to file: {}", output);

    fs::write(output, md.as_bytes())?;
    println!("Done!");

    Ok(())
}
