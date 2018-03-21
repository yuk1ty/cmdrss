#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::str::FromStr;

enum Rss {
    HackerNews(String),
    Reddit(String),
    HatenaBookMark(String),
}

impl FromStr for Rss {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hacker" => Ok(Rss::HackerNews("hackernews".to_string())),
            "reddit" => Ok(Rss::Reddit("reddit".to_string())),
            "hatena" => Ok(Rss::HatenaBookMark("hatena bookmark".to_string())),
            _ => Err("Sorry, not matched in this app."),
        }
    }
}

fn main() {
    let arg_matches = App::new("cmdrss")
        .version("1.0")
        .about("A RSS reader runs on command line.")
        .author("Yuki Toyoda")
        .arg(
            Arg::from_usage("<feeds> 'みたいサイト'")
                .possible_values(&["hacker", "reddit", "hatena"]),
        )
        .get_matches();

    let ty = value_t!(arg_matches, "feeds", Rss).unwrap_or_else(|e| e.exit());

    match ty {
        Rss::HackerNews(url) => println!("{}", url),
        Rss::Reddit(url) => println!("{}", url),
        Rss::HatenaBookMark(url) => println!("{}", url),
    }
}
