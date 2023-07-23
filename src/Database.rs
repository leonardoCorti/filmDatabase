use std::{path::Path, fs::File, io::Write};
use chrono::prelude::*;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use druid::{im::{vector, Vector}, Data, Lens};
use crate::Film::Film;

#[derive(Debug, Clone, Data, Lens)]
pub struct Database{
    path: String,
    number_of_films: usize,
    films: Vector<FilmInDatabase>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Data, Lens)]
pub struct FilmInDatabase {
    pub Title: String,
    pub Year: String,
    pub Released: String,
    pub Runtime: String,
    pub Genre: String,
    pub Metascore: String,
    pub Poster: String,
    pub DateWatched: String,
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
        let films: Vector<FilmInDatabase> = films.into();
        Ok(Database{
            path: path.to_string(),
            number_of_films,
            films
        })
    }

    fn update_database(&self, a_film: FilmInDatabase) -> Result<(), Box<dyn std::error::Error>>{
        let file_database = std::fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(&self.path)?;
    
        let mut wrt = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file_database);

        let _ = wrt.serialize(a_film)?;
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
        self.films.push_back(the_film_in_database);
        Ok(())
    }

 }
