use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let depth_arg: String;

    if args.len() > 1 {
        depth_arg = args[1].clone();
    } else {
        depth_arg = "0".to_string();
    }

    let depth = match depth_arg.parse::<i32>() {
        Ok(i) => i,
        Err(_error) => 0,
    };
    traverse("./".to_string(), depth, "".to_string())
}

fn traverse<P: AsRef<Path>>(path: P, depth: i32, indentation: String) -> () {
    let paths = match fs::read_dir(&path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error reading directory{}: {}", path.as_ref().display(), e);
            return;
        }
    };
    for path in paths {
        match path {
            Ok(item) => {
                let path_buf = item.path();
                let metdata = match item.metadata() {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("Error reading metadata for {}: {}", path_buf.display(), e);
                        continue;
                    }
                };
                //
                let display_path = match path_buf.strip_prefix("./") {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!(
                            "Error stripping ./ prefix from path {}: {}",
                            path_buf.display(),
                            e
                        );
                        return;
                    }
                };

                if metdata.is_dir() {
                    println!("{} {}", indentation, display_path.display());
                    if depth > 0 {
                        traverse(path_buf, depth - 1, indentation.clone() + "  ")
                    }
                } else {
                    println!("{}{}", indentation, display_path.display());
                }
            }
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
            }
        }
    }
}
