pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

// shared trait, to summarize both NewsArticle and Tweet
pub trait Summary {
    // default implementation
    // if no default implementation
    // it foreces overwrite
    fn summarize(&self) -> String {
        return String::from("Read more......");
    }
}

// implement summary trait
// now has to overwrite summarize function
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.headline, self.author);
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

// take input of any type implementing struct
// trait bond
fn notify<T: Summary>(item: T) {
    println!("Breaking news: {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is falling"),
        content: String::from("The sky is not actually falling")
    };
    
    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article Summary: {}", article.summarize());
    notify(article);
}