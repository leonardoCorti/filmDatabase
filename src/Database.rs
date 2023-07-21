use std::{path::Path, fs::File, io::Write};
use chrono::prelude::*;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

use crate::Film::Film;

#[derive(Debug)]
pub struct Database{
    path: String,
    number_of_films: usize,
    films: Vec<FilmInDatabase>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilmInDatabase {
    Title: String,
    Year: String,
    Released: String,
    Runtime: String,
    Genre: String,
    Metascore: String,
    Poster: String,
    DateWatched: String,
}

impl Database {

    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Path::new(path);
        if !file_path.exists() {
            let mut file = File::create(file_path).unwrap();
            let header = format!("Title,Year,Released,Runtime,Genre,Metascore,Poster,DateWatched\n");
            file.write(header.as_bytes())?;
        }

        let mut films = Vec::new();
        let file = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);
        for result in rdr.deserialize() {
            let record: FilmInDatabase = result?;
            films.push(record);
        }
        let number_of_films = films.len();

        Ok(Database{
            path: path.to_string(),
            number_of_films,
            films
        })
    }

    pub fn update_database(&self) -> Result<(), Box<dyn std::error::Error>>{
        let diff = self.films.len() - self.number_of_films;
        if diff<1 { return Ok(())};
        let new_films: Vec<FilmInDatabase> = self.films[self.films.len()-diff..].to_vec();
        let file_database = std::fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(&self.path)?;
    
        let mut wrt = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file_database);

        for film in new_films {
            let _ = wrt.serialize(film)?;
        } 
        wrt.flush()?;

        Ok(())
    } 

    pub fn add_a_film(&mut self, the_film: Film) -> Result<(), Box<dyn std::error::Error>>{
        let local: Date<Local> = Local::today();
        let DateWatched = local.format("%Y-%m-%d");
        let the_film_in_database: FilmInDatabase = FilmInDatabase {
            Title: the_film.Title,
            Year: the_film.Year,
            Released: the_film.Released,
            Runtime: the_film.Runtime,
            Genre: the_film.Genre,
            Metascore: the_film.Metascore,
            Poster: the_film.Poster,
            DateWatched: DateWatched.to_string(),
        };
        self.films.push(the_film_in_database);
        Ok(())
    }

 }
