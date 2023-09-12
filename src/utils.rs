use curl;
use scraper;

pub fn get_html(url: &str) -> Result<String, curl::Error> {
    let mut easy = curl::easy::Easy::new();
    easy.url(url)?;
    easy.useragent("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)")?;
    let mut html = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            html.push_str(std::str::from_utf8(data).unwrap());
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    Ok(html)
}

struct Comment {
    op: String,
    content: String,
}

struct Post {
    url: String,
    title: String,
    content: String,
    comments: Comment,
}
impl Post {
    fn get_comments(&self) {
        !todo!("get comments")
    }

    fn get_title(&self) {
        !todo!("get title")
    }

    fn get_content(&self) {
        !todo!("get content")
    }
}
