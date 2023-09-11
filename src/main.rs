mod utils;
use regex;
use scraper::{self, Selector};

fn main() {
    let url = "https://old.reddit.com/";
    let html = utils::get_html(url).unwrap();
    // print!("{}", html)

    let postid_regex = regex::Regex::new(r"thing_t3_(\w+)").unwrap();

    let mut posts = postid_regex.find_iter(&html).map(
        |m| m.as_str().to_string()
    ).collect::<Vec<String>>();


    let document = scraper::Html::parse_document(&html);

    for postid in posts.iter(){
        let postselector = Selector::parse(&format!("#{}", postid)).unwrap();
        let posthtml = document.select(&postselector).next().unwrap();
        print!("{} \n\n", posthtml.html());
    }
}
