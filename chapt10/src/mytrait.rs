pub struct Tweet {
    tweet: String,
}

impl Tweet {
    pub fn new(tweet: String) -> Tweet {
        Tweet { tweet }
    }
}

pub struct NewsArticle {
    content: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet: {}", self.tweet)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("News: {}", self.content)
    }
}

fn print_summary(item: impl Summary) {
    println!("{}", item.summarize());
}
fn get_summary() -> impl Summary {
    Tweet::new(String::from("qwe"))
}

pub fn run() {
    let tweet = Tweet {
        tweet: String::from("Hello, twitter!"),
    };

    let news = NewsArticle {
        content: String::from("Hello, news!"),
    };

    println!("Tweet: {}", tweet.summarize());
    println!("News: {}", news.summarize());
}
