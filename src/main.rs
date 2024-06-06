use std::env::{self, current_dir};
use std::path;
use std::fs;
use std::str::from_utf8;

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
    let data: Vec<u8> = fs::read(path)?;

    println!("file: {}\n {}", path.display(), from_utf8(&data)? );

    Ok(())
}
