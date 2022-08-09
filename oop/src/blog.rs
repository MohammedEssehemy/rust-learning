// struct DraftState;
// struct WaitingForReviewState;
// struct PublishedState;

// enum State {
//     Draft(DraftState),
//     WaitingForReview(WaitingForReviewState),
//     Published(PublishedState)
// }


// struct Post {
//     state: State,
//     content: String
// }

// impl Post {
//     pub fn new() -> Self {
//         Self { state: State::Draft(DraftState {}), content: String::new()}
//     }

//     pub fn content(&self) -> &str {
//         match self.state {
//             State::Published(_) => self.content().as_ref(),
//             _ => "",
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     pub fn request_review(&mut self) {
//         match self.state {
//             State::Draft(_) => {
//                 // do it here
//                 self.state = State::WaitingForReview(WaitingForReviewState {})
//             },
//             _ => {}
//         }
//     }

//     pub fn approve(&mut self) {
//         match self.state {
//             State::WaitingForReview(_) => {
//                 // do it here
//                 self.state = State::Published(PublishedState {})
//             },
//             _ => {}
//         }
//     }
// }


// pub fn try_blog() {
//     let mut post = Post::new();

//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());

//     post.request_review();
//     assert_eq!("", post.content());

//     post.approve();
//     assert_eq!("I ate a salad for lunch today", post.content());
// }
// ----------------------------


trait State {
    fn request_review(&self) -> Option<Box<dyn State>> {
        None
    }
    fn approve(&self) -> Option<Box<dyn State>> {
        None
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(&self) -> Option<Box<dyn State>> {
        Some(Box::new(PendingReview {}))
    }
}


struct PendingReview {}

impl State for PendingReview {
    fn approve(&self) -> Option<Box<dyn State>> {
        Some(Box::new(Published {}))
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct Post {
    state: Box<dyn State>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Box::new(Draft {}),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        self.state.content(&self)
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(new_state) = self.state.request_review() {
            self.state = new_state;
        }
    }

    pub fn approve(&mut self) {
        if let Some(new_state) = self.state.approve() {
            self.state = new_state;
        }
    }
}



pub fn try_blog() {
    println!("blog flow - State Pattern");
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Done");
}