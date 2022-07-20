use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more..{}", self.summarize_author())
    }
    fn summarize_author(&self) -> String;

    fn notify<T: Summary>(item: T) {
        println!("Breaking News {}", item.summarize());
    }

    fn notify_1(item: &impl Summary, item2: &impl Summary) {
        println!("Breaking News {}", item.summarize());
        println!("Breaking News {}", item2.summarize());
    }

    fn notify_3(item: &(impl Summary + Display)) {
        println!("{}", item.summarize());
    }

    fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug;

    fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        todo!()
    }

    fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
        todo!()
    }

    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Tweet {
    pub fn returns_summarizable(&self) -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        todo!()
    }

    fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
        todo!()
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet {}", tweet.summarize());
    // println!("{:#?}",tweet.returns_summarizable());

    let article = NewsArticle {
        headline: String::from("Penguins win the stanley  cup championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("{:?}", article);
}
