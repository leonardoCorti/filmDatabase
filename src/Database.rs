use std::{path::Path, fs::File, io::Write, result};
use chrono::prelude::*;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

use crate::Film::Film;

use druid::Data;

#[derive(Debug, Clone, Data)]
pub struct Database{
    path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

       Ok(Database{
            path: path.to_string(),
        })
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
        //let films: Vec<FilmInDatabase> = self.get_films();
        self.write_new_film(the_film_in_database)?;
        Ok(())
    }

    fn write_new_film(&self, new_film: FilmInDatabase) -> Result<(), Box<dyn std::error::Error>> {
        let file_database = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)?;
        let mut wrt = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file_database);
        wrt.serialize(new_film)?;
        wrt.flush()?;
        Ok(())
    }

    pub fn get_films(&self) -> Vec<FilmInDatabase> {
        let mut films = Vec::new();
        let file = File::open(&self.path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);
        for result in rdr.deserialize() {
            let record: FilmInDatabase = result.unwrap();
            films.push(record);
        }
        films
    }

 }
