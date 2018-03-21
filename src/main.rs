#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate rss;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use std::str::FromStr;
use rss::Channel;

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
        Rss::HackerNews(url) => create_list(&url)
            .iter()
            .for_each(|item| println!("{}", serde_json::to_string(&item).unwrap())),
        Rss::Reddit(url) => create_list(&url)
            .iter()
            .for_each(|item| println!("{}", serde_json::to_string(&item).unwrap())),
        Rss::HatenaBookMark(url) => create_list(&url)
            .iter()
            .for_each(|item| println!("{}", serde_json::to_string(&item).unwrap())),
    };
}

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

#[derive(Serialize, Deserialize, Debug)]
struct FeedItem {
    title: String,
    link: String,
}

fn create_list(url: &str) -> Vec<FeedItem> {
    let channel = Channel::from_url(url).unwrap();
    let items: Vec<FeedItem> = channel
        .items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
        })
        .collect();
    items
}
