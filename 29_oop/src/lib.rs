// this is the "object" which will contain a state
pub struct Post {
	// we use a box pointer here that takes a State trait type.
	// this means we can define new types of states but they must
	// contain the common state trait if they are to be used here.
	state: Option<Box<dyn State>>,
	content: String,
}

// as with structs in other chapters it will be instantiated with
// a "new" method
impl Post {
	pub fn new() -> Post {
		return Post {
			// when a post is created we enforce the rule that it must
			// be created in a Draft satte
			state: Some(Box::new(Draft {})),
			content: String::new(),
		};
	}

	// enable the user to add text to a blog post.
	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text);
	}

	// enable the user to read the text from a blog post
	pub fn content(&self) -> &str {
		// what content is returned depends on the state of the Post.
		// so we ask what state the Post is in and then use the `content()`
		// method of its state to decide what content is returned.
		return self.state.as_ref().unwrap().content(self);
	}

	// enable the user to change a blogs state to "PendingReview".
	pub fn request_review(&mut self) {
		// call `take()` on the posts state to remove its current state.
		if let Some(s) = self.state.take() {
			// then call Drafts internal version of the method `request_review`
			// to get a new state in the form of the struct "PendingReview"
			self.state = Some(s.request_review());
		}
	}

	pub fn approve(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.approve());
		}
	}
}

// Posts can be in one of three states; "Draft", "PendingReview"
// and "Published". We'll create three structs to represent the states
// and give them all a shared trait so they can be used with the Post object.
trait State {
	// these are the default methods for each struct that contains this trait. 
	// If they don't define their own versions of these methods, these defaults
	// will be used.
	fn request_review(self: Box<Self>) -> Box<dyn State>;
	fn approve(self: Box<Self>) -> Box<dyn State>;
	fn content<'a>(&self, _post: &'a Post) -> &'a str {
		return "";
	}
}

struct Draft {}

impl State for Draft {
	// here we implement Drafts own version of the `request_review` method
	// which switches a Posts state to "PendingReview". This makes the "Draft"
	// wholly responsible for switching to a new state.
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		return Box::new(PendingReview {});
	}

	fn approve(self: Box<Self>) -> Box<dyn State> {
		return self;
	}
}

struct PendingReview {}

impl State for PendingReview {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		return self;
	}

	fn approve(self: Box<Self>) -> Box<dyn State> {
		return Box::new(Published {});
	}
}

struct Published {}

impl State for Published {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		return self;
	}

	fn approve(self: Box<Self>) -> Box<dyn State> {
		return self;
	}

	fn content<'a>(&self, post: &'a Post) -> &'a str {
		return &post.content;
	}
}