use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
use ammonia::Builder;

fn run() -> io::Result<()> {
    let input = env::args().nth(1).unwrap_or_else(|| String::from("-"));
    let output = env::args().nth(2).unwrap_or_else(|| String::from("-"));

    let mut rdr: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut wrt: Box<dyn Write> = if output == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(output)?)
    };


     let mut tags = HashSet::new();
     let mut tag_attributes = HashMap::new();

     tags.insert("html");
     tags.insert("head");
     tags.insert("body");
     tags.insert("title");

     tags.insert("h1");
     tags.insert("h2");
     tags.insert("h3");
     tags.insert("h4");
     tags.insert("h5");
     tags.insert("h6");
     
     tags.insert("p");
     tags.insert("br");

     tags.insert("a");
     let mut a_attributes = HashSet::new();
     a_attributes.insert("href");
     a_attributes.insert("title");
     tag_attributes.insert(
         "a",
         a_attributes
     );

     tags.insert("img");
     let mut img_attributes = HashSet::new();
     img_attributes.insert("src");
     img_attributes.insert("alt");
     tag_attributes.insert(
         "img",
         img_attributes
     );

     tags.insert("strong");
     tags.insert("em");
     tags.insert("blockquote");
     tags.insert("pre");


     tags.insert("ol");
     tags.insert("ul");
     tags.insert("li");

     tags.insert("hr");

     tags.insert("table");
     tags.insert("tr");
     tags.insert("th");
     tags.insert("td");

     Builder::new().tags(tags).tag_attributes(tag_attributes)
         .clean_from_reader(&mut rdr)?
         .write_to(&mut wrt)?;

     Ok(())
}

fn main() {
    env_logger::init();
    if let Err(ref e) = run() {
        println!("error: {}", e);
        process::exit(1);
    }
}