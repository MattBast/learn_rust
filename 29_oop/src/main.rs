// This code implements a blog post programme.
// It does the following:
//
// 1. A blog post starts as an empty draft.
// 2. When the draft is done, a review of the post is requested.
// 3. When the post is approved, it gets published.
// 4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

use oop::Post;

// a good test for the blog post is if the content of the blog changes
// before its state changes to approved.
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a sandwich for lunch today");
    println!("Draft state: {}", post.content());

    post.request_review();
    println!("Pending review state: {}", post.content());

    post.approve();
    println!("Approved state: {}", post.content());
}
