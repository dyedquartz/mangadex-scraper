extern crate reqwest;

use reqwest::Url;
use std::env;
use std::fs::File;
use std::io;


fn main() -> Result<(), reqwest::UrlError> {
    let args: Vec<String> = env::args().collect();

    let id: &str = &args[1];

    println!("{:?}", args);
    println!("{}", id);

    if args.len() != 3 { panic!("mangadex-splice <id> <amount>"); }

    let base_url = Url::parse("https://s5.mangadex.org/data/")?;
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

    for prefix in prefixes {
        let url = base_url.join(&format!("{}/{}1.png",id, prefix))?;
        // println!("{:?}", url);
        let resp = reqwest::get(url).unwrap();
        if resp.status() == reqwest::StatusCode::OK {
            pre = String::from(prefix);
            break;
        }
        // println!("{:?}",resp.status());
    }

    println!("File Prefix: {}", pre);
    /*
    let mut resp = reqwest::get(format!("https://s5.mangadex.org/data/{}/{}.png","re", "re"))?;
    let mut out = File::create(format!("{}.png", "1")).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy");
    */
    Ok(())
}
