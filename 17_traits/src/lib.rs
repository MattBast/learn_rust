// in this example we're creating a trait called Summary. This trait can be 
// used to summarize various structs representing articles. We might have a
// struct representing a tweet for instance and another representing a
// newspaper article. But with a summary trait they can all uniformely
// produce a summary of their content.
pub trait Summary {
    // inside the curly brackets we define the methods that define the trait.
    // In this case we define a single method that returns a string. These are
    // known as method signatures.
    fn summarize(&self) -> String {
        // this is the default behaviour of summarize when a custom type doesn't
        // define its own behaviour.
        String::from("(Read more...)")
    }

    // more methods could be included here.
}

// this is our first struct that will use the SUmmary trait.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// we add the trait to NewsArticle using the `impl` and `for` keywords
impl Summary for NewsArticle {
    // we then need some custom code to define how the trait signatures will
    // be fulfilled by NewsArticle
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// this is another example of a type using the Summary trait.
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// we can add the Summary trait without defining a custom summarize signature.
// this means Tweet will use the default signature for summarize.
impl Summary for Tweet {}

// we can also define functions that can accept any type that includes the Summary trait
pub fn notify(item: &impl Summary) {
    // these functions can then make use of Summarys signatures with the certainty that
    // the function will definetly be available to use since a type with the SUmmary 
    // trait must contain this method.
    println!("Breaking news! {}", item.summarize());
}

// notify could also be written in a more verbose way.
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }