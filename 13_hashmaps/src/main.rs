// hashmaps come from the standard collection crate so need to 
// be imported
use std::collections::HashMap;

fn main() {
    // they can be declared empty with the new method
    let mut scores = HashMap::new();

    // use the insert method to create a new key value pair
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 20);

    let blue_team_name = String::from("Blue");
    let blue_score = scores
        // use the get method to get a value using a key as a pointer
        // this returns an Option<&V> which equals None if the key doesn't
        // exist in the hashmap
        .get(&blue_team_name)
        // this parses Option<&i32> into Option<i32>, i.e. a varaible rather
        // than a reference
        .copied()
        // finally this method extracts the value from Option<i32>. If Option
        // is None though it will return 0 instead.
        .unwrap_or(0);

    println!("Team {} has a score of {}", blue_team_name, blue_score);

    // to overwrite a keys value call insert again on the keys name
    scores.insert(String::from("Blue"), 25);

    // the or_insert function can be used to insert a new value only if the 
    // key does not already exist in the hashmap
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // for loops can be used to get all keys and values from a hashmap
    println!("Starting loop over hashmap keys and values");
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    // another common use case is to update a keys value based on the value it
    // currently is. Word counts is a good example
    let sentence = "hello world wonderful world";

    let mut word_count = HashMap::new();

    // split_whitespace returns an iterator over slices representing each word
    for word in sentence.split_whitespace() {
        // we start by initialising a key of the word has not yet been added to the hashmap.
        // the or_insert method returns a reference to the value of the key
        let count = word_count.entry(word).or_insert(0);
        
        // we use the asterisk to dereference count into a variable so we can increment it.
        // as count is owned by the hashmap we are really updating the value in the hashmap here
        // even if it looks like the count variable is independent of hashmap.
        *count += 1;
    }

    println!("{:?}", word_count);

    // as we exit the functions scope ownership means that hashmap and all its
    // keys and values are dropped
} 
