extern crate reqwest;

use reqwest::Url;
use std::env;
use std::fs::File;
use std::io;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut resp = reqwest::get(format!("https://s5.mangadex.org/data/{}/{}.png","re", "re")).expect("reeeee");
    let mut out = File::create(format!("{}.png", "1")).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy");
}
