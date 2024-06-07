use std::env::{self, current_dir};
use std::{io, path};
use std::fs;
use std::str::from_utf8;

use xml::reader::{EventReader, XmlEvent};

fn main() -> anyhow::Result<()>{
    let args: Vec<String> = env::args().collect();
    let mut path = path::Path::new(".");

    if args.len() > 1 {
        path = path::Path::new(&args[1]);
    }

    dbg!("Path: {}", path.display());

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension(){
                    if ext == "CLB"{
                        rename_cad_files(&path)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn rename_cad_files( path: &path::PathBuf ) -> anyhow::Result<()>{
    let file = fs::File::open(path)?;
    let file = io::BufReader::new(file);
    let contents = String::new();

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }

            Ok( XmlEvent::Characters(contents) ) => {
                println!("{}", contents);
            }

            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    println!("file: {}", path.display());

    Ok(())
}
