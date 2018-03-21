#[macro_use]
extern crate clap;
extern crate rss;

use clap::{App, Arg};
use std::str::FromStr;
use rss::Channel;

enum Rss {
    HackerNews(String),
    Reddit(String),
    HatenaBookMark(String),
}

impl FromStr for Rss {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hacker" => Ok(Rss::HackerNews("https://hnrss.org/newest".to_string())),
            "reddit" => Ok(Rss::Reddit(
                "https://www.reddit.com/r/programming/.rss".to_string(),
            )),
            "hatena" => Ok(Rss::HatenaBookMark(
                "http://b.hatena.ne.jp/hotentry/it.rss".to_string(),
            )),
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
        Rss::HackerNews(url) => {
            let channel = Channel::from_url(&url).unwrap();
            println!("{}", channel.to_string());
        }
        Rss::Reddit(url) => {
            let channel = Channel::from_url(&url).unwrap();
            println!("{}", channel.to_string());
        }
        Rss::HatenaBookMark(url) => {
            let channel = Channel::from_url(&url).unwrap();
            println!("{}", channel.to_string());
        }
    }
}
