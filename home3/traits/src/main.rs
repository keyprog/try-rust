fn main() {
    println!("Hello, world!");
    handle(&NewsArticle{author: String::from("alex"), headline: String::from("hello world"), content: String::from("bla")});
}

fn handle(summary: &impl Summary)  {
    println!("{}", summary.summary());
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("author: {}, headline {}", self.author, self.headline)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("username: {}, content {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summary(&self) -> String;
}
