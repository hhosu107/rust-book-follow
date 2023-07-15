pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        // take() method takes the Some value out of the state field and leaves a None in its place
        // This ensures that the Post can’t use the old state value after we’ve transformed it into a new state.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        // take() method takes the Some value out of the state field and leaves a None in its place
        // This ensures that the Post can’t use the old state value after we’ve transformed it into a new state.
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(self: &self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Box::new(PendingReview {}) creates a new instance of PendingReview and puts it in a Box
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Box::new(PendingReview {}) creates a new instance of PendingReview and puts it in a Box
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Box::new(PendingReview {}) creates a new instance of PendingReview and puts it in a Box
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Box::new(PendingReview {}) creates a new instance of PendingReview and puts it in a Box
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Box::new(PendingReview {}) creates a new instance of PendingReview and puts it in a Box
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Box::new(PendingReview {}) creates a new instance of PendingReview and puts it in a Box
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
