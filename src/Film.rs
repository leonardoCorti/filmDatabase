use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Film{
    Title: String,
    Year: String,
    Released: String,
    Runtime: String,
    Genre: String,
    Metascore: String,
    Poster: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilmFromListOmdb {
    Title: String,
    Year: String,
    imdbID: String,
    Poster: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilmList {
    Search: Vec<FilmFromListOmdb>,
    totalResults: String,
}

pub fn search_film(name: &str, api: &str) -> FilmList {
    let base_url = "http://www.omdbapi.com/";
    let url = format!("{}/?apikey={}&s={}",base_url,api,name.replace(" ", "%20") );
    println!("{}", url);
    let response = reqwest::blocking::get(url).unwrap();
    let response_string = response.text().unwrap();

    let results: FilmList = serde_json::from_str(&response_string).unwrap();
    results
}

pub fn film_info(id: &str, api: &str) -> Film {
    let base_url = "http://www.omdbapi.com/";
    let url = format!("{}/?apikey={}&i={}",base_url,api,id );
    println!("{}", url);
    let response = reqwest::blocking::get(url).unwrap();
    let response_string = response.text().unwrap();

    let result: Film = serde_json::from_str(&response_string).unwrap();
    result 
}
