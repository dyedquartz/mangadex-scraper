extern crate reqwest;

use reqwest::Url;
use std::env;
use std::fs;
use std::fs::File;
use std::io;


fn main() -> Result<(), reqwest::UrlError> {
    // command line arguments
    let args: Vec<String> = env::args().collect();

    let id: &str = &args[1];

    println!("{:?}", args);
    println!("{}", id);

    if args.len() != 4 { panic!("mangadex-splice <id> <amount> <output>"); }

    let base_url = Url::parse("https://s2.mangadex.org/data/")?;
    let prefixes = vec!["", "x", "s"];
    
    // testing id directory
    let url = base_url.join(id)?;
    let resp = reqwest::get(url).unwrap();
    match resp.status() {
        reqwest::StatusCode::FORBIDDEN => println!("Correct ID Path"),
        reqwest::StatusCode::NOT_FOUND => panic!("Incorrect ID Path"),
        _ => panic!("Unknown ID Path"),
    }

    let mut pre = String::new();
    
    // grabbing correct file prefix
    for prefix in prefixes {
        let url = base_url.join(&format!("{}/{}1.png",id, prefix))?;
        println!("{:?}", url);
        let resp = reqwest::get(url).unwrap();
        if resp.status() == reqwest::StatusCode::OK {
            pre = String::from(prefix);
            break;
        }
        println!("{:?}",resp.status());
    }

    println!("File Prefix: {}", pre);
   
    // downloading files
    for i in 1..args[2].parse::<i32>().unwrap()+1 {
        println!("{}", i);

        fs::create_dir_all(format!("{}", args[3])).unwrap();
        let url = base_url.join(&format!("{}/{}{}.png",id, pre, i))?;
        let mut resp = reqwest::get(url).unwrap();
        let mut out = File::create(format!("{}/{}.png", args[3], i)).expect("failed to create file");
        io::copy(&mut resp, &mut out).expect("failed to copy");
    }
    Ok(())
}
