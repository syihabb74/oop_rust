
use oop_rust::caller_trait_object::TryConnect;
use oop_rust::trait_object::{Initialization, MySql, Postgres};

fn main() {
    // let mysql = MySql::init(String::from("Dev"), String::from("127.0.0.1:3000"));
    // let postgres = Postgres::init(String::from("Dev"), String::from("127.0.0.1:3000"));
    // let try_connect = TryConnect::new();
    // try_connect.connecting(Box::new(mysql));
    // try_connect.connecting(Box::new(postgres));
    // top is trait object


    // OOP traditional state pattern
    // let mut post = Post::new();

    let mut post = Post::new();

    // post.content();


    post.add_text("I will go to play football tomorrow");
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    // assert_eq!("I will go to play football tomorrow", post.content());
    println!("{}", post.content());


}



pub struct Post {
    state : Option<Box<dyn State>>,
    content : String,
    published_threshold : i8
}


impl Post {
    pub fn new() -> Self {
        Self { state: Some(Box::new(Draft{})), content: String::from("") , published_threshold : 2}
    }

    pub fn add_text(&mut self, text : &str) {
        if let Some(value) = self.state.take() {
            value.change_content(self, text);
            self.state = Some(value)
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(value) = self.state.take() {
            self.state = Some(value.request_review())
        }
    }

    pub fn approve(&mut self) {
        match self.published_threshold {
            0 => {},
            _ => self.published_threshold -= 1
        }
        if let Some(value) = self.state.take() {
            self.state = Some(value.approve(self.published_threshold))
        }
    }

    pub fn reject(&mut self) {
        if let Some(value) = self.state.take() {
            self.state = Some(value.reject())
        }
    }


}


trait State {
    fn request_review(self : Box<Self>) -> Box<dyn State>;
    fn approve (self : Box<Self>, threshold : i8) -> Box<dyn State>;
    fn reject(self : Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _ : &'a Post) -> &'a str {
        ""
    }
    fn change_content(&self, post : &mut Post, text : &str);
}

pub struct Draft {}
pub struct PendingReview {}

pub struct Published{}

impl State for Draft {
    fn request_review(self : Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
    fn approve (self : Box<Self>, _ : i8) -> Box<dyn State> {
        self
    }
    fn reject(self : Box<Self>) -> Box<dyn State> {
        self
    }

    fn change_content(&self, post : &mut Post, text : &str) {
        post.content.push_str(text);
    }

}

impl State for PendingReview {
    fn request_review(self : Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve (self : Box<Self>, threshold : i8) -> Box<dyn State> {
        match threshold {
            0 => {
                Box::new(Published{})
            },
            _ => {
                self
            }
        }
    }
     fn reject(self : Box<Self>) -> Box<dyn State> {
        Box::new(Draft{})
    }
    fn change_content(&self, _ : &mut Post, _ : &str) {
        
    }
}

impl State for Published {
    fn request_review(self : Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve (self : Box<Self>, _ : i8) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self : Box<Self>) -> Box<dyn State> {
        self
    }

    fn change_content(&self, _ : &mut Post, _ : &str) {
        
    }
}