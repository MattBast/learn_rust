// import the trait and the data type (struct) we want to make use of.
// as this is a library we need to have its name in the import as well
// as the `Cargo.toml` file.
use aggregator::{Summary, NewsArticle, Tweet, notify};

fn main() {
    // declare the structs as usual
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    // and then call their Summary trait method as if it was any other method
    println!("New article available! {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());

    // call a function that takes a type including the Summary trait
    notify(&article);
}
