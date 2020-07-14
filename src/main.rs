extern crate walkdir;
extern crate md5;
use std::{env};
use rayon::prelude::*;

fn main() {
    let mut paths: Vec<String> = vec![];
    for arg in env::args().skip(1) {
        for entry in walkdir::WalkDir::new(arg)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            paths.push(String::from(entry.path().to_string_lossy()))
        }
    }
    // since it compares basing on Unicode code points, we have to
    // lowercase every name so we are fine 
    paths.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let mut s = String::new();
    s = paths
        .par_iter()
        .map(|s| format!("{:x}", md5::compute(s)))
        .collect::<Vec<String>>()
        .join("");
    //println!("{:?}", s);
    
    let digest_final = md5::compute(s);
    let output = &format!("{:x}", digest_final);
    print!("{}", output);
}
