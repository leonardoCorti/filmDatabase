use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Film {
    title: String,
    year: u32,
    image: String,
    runtime: String,
    plot: String,
    director: String,
    stars: String,
    geners: String,
    rating: u16,
}
