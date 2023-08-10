use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Film{
    pub Title: String,
    pub Year: String,
    pub Released: String,
    pub Runtime: String,
    pub Genre: String,
    pub Metascore: String,
    pub Poster: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilmFromListOmdb {
    pub Title: String,
    pub Year: String,
    pub imdbID: String,
    pub Poster: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilmList {
    Search: Vec<FilmFromListOmdb>,
    totalResults: String,
}

impl FilmList {
    pub fn get_list(&self) -> &Vec<FilmFromListOmdb> {
        &self.Search
    }
}

//TODO: add error handling
pub fn search_film(name: &str, api: &str) -> Result<FilmList, Box<dyn std::error::Error>> {
    let base_url = "http://www.omdbapi.com/";
    let url = format!("{}/?apikey={}&s={}",base_url,api,name.replace(" ", "%20") );
    println!("{}", url);
    let response = reqwest::blocking::get(url)?;
    let response_string = response.text()?;

    let results: FilmList = serde_json::from_str(&response_string)?;
    Ok(results)
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
