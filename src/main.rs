extern crate regex;
extern crate rustyline;

use std::env;
use regex::Regex;
use rustyline::Editor;

mod twitbeam;

fn main() {

    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN not set.");
    let api_server = env::var("API_SERVER").expect("API_SERVER not set.");
    let re = Regex::new(r#"<(".*?"|'.*?'|[^'"])*?>"#).unwrap();
    let mut rl = Editor::<()>::new();

    let client = twitbeam::Client::new(&token, &api_server);

    loop {
        for v in client.home().as_array().unwrap().iter().rev() {
            let account = v["account"]["acct"].as_str().unwrap();
            let context = re.replace_all(v["content"].as_str().unwrap(), "");
            println!("{: <20} : {}", account, context);
        }
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                } else {
                    rl.add_history_entry(&line);
                    client.toot(&line);
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
