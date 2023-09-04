use std::fs;
use regex::Regex;

pub fn find(paths: &Vec<String>, reg: &Regex, verbose: bool, recursive: bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matched: Vec<String> = Vec::new();
    for path in paths {
        let path = std::path::Path::new(path);
        if path.is_dir() {
            tracing::debug!("Searching directory: {}", path.to_str().unwrap());
            if verbose {
                eprintln!("\x1b[37;1mSearching directory: \x1b[0m {}", path.to_str().unwrap());
            }
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if recursive {
                        let mut sub_matched = find(&vec![path.to_str().unwrap().to_string()], reg, verbose, recursive)?;
                        matched.append(&mut sub_matched);
                    }
                } else {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if verbose {
                        eprint!("Checking file: {} ...... ", file_name);
                    }
                    if reg.is_match(file_name) {
                        matched.push(path.to_str().unwrap().to_string());
                        tracing::debug!("Checking file: {} ...... Matched!", file_name);
                        if verbose {
                            eprintln!("\x1b[32mMatched! \x1b[0m");
                        }
                    } else {
                        tracing::debug!("Checking file: {} ...... Not matched!", file_name);
                        if verbose {
                            eprintln!("\x1b[31mNot matched! \x1b[0m");
                        }
                    }
                }
            }
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if verbose {
                eprint!("Checking file: {} ...... ", file_name);
            }
            if reg.is_match(file_name) {
                matched.push(path.to_str().unwrap().to_string());
                tracing::debug!("Checking file: {} ...... Matched!", file_name);
                if verbose {
                    eprintln!("\x1b[32mMatched! \x1b[0m");
                }
            } else {
                tracing::debug!("Checking file: {} ...... Not matched!", file_name);
                if verbose {
                    eprintln!("\x1b[31mNot matched! \x1b[0m");
                }
            }
        }
    }
    Ok(matched)
}