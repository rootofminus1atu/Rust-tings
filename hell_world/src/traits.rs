pub fn display() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("Lorem Ipsum")
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    // how can I make this work?
    let mut things: Vec<&dyn Summary> = Vec::new();
    things.push(&article);
    things.push(&tweet);

    for thing in things {
        println!("New article available! {}", thing.summarize());
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}