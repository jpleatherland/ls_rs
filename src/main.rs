use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let depth_arg = &args[1];
    let depth = depth_arg.parse::<i32>().unwrap();
    traverse("./".to_string(), depth)
}

fn traverse(path: String, depth: i32) -> () {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let metdata = path.as_ref().unwrap().metadata().unwrap(); 
        if metdata.is_dir() && depth > 0{
            traverse(path.unwrap().path().display().to_string(), depth-1)
        } else {
            println!("{}", path.unwrap().path().display())
        }
    }
}
