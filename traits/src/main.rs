mod news;

use crate::news::news::Summary;

fn main() {
    let newsarticle = crate::news::news::NewsArticle {
        headline: String::from("Test Headline"),
        location: String::from("Cleveland, OH"),
        author: String::from("Some Author"),
        content: String::from("Test Content")
    };

    println!("{}", newsarticle.summarize());

    news::news::notify(newsarticle);
}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Summary + Clone,
          U: Clone + Summary
{
    0
}
