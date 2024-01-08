pub trait Summary {
    // fn summarize(&self) -> String;
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

// impl Summary for Tweet {}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for String {
    fn summarize(&self) -> String {
        format!("{} (Read more from String...)", self)
    }
}

// pub fn notify<T: Summary>(item: &T) {
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// // for multiple parameters we can use this to allow different types between parameters
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// or this, to force the same ytype for parameters
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
// // for multiple traits we can use this
// pub fn notify(item: &(impl Summary + Display)) {
// or this
// pub fn notify<T: Summary + Display>(item: &T) {

// this
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// is the same as this
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

// returning traits that implement traits
// fn returns_summarizable() -> impl Summary {