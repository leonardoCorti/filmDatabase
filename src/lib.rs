#![allow(non_snake_case)]

use std::fs::{File, self};
use std::io::{Read, Write};
use std::path::Path;
use Database::FilmInDatabase;
use serde::{Deserialize, Serialize};
use druid::widget::{Align, Flex,TextBox, Button, Label};
use druid::{Data, Lens, Widget, WidgetExt, WindowConfig, Size, Command, Target, EventCtx, Env, ImageBuf};
// use image;
mod Film;
// mod Database;
pub mod Database;
const FILE_SIZE: usize = 100000;
const PLACEHOLDER_POSTER: &[u8] = include_bytes!("graphics/poster_placeholder.jpg");

#[derive(Clone, Data, Lens)]
pub struct HelloState {
    pub api_user: String,
    pub api_key: Option<String>,
    pub text_bar: String,
    pub database: Database::Database,
    pub status_bar: String,
}

#[derive(Deserialize, Serialize)]
struct Config{
    api_key: String,
}

pub fn homepage(state: &HelloState) -> impl Widget<HelloState> {

    let status: Label<HelloState> = Label::new(
        |data: &HelloState, _env: &_| data.status_bar.clone()
    );

    let layout = Flex::column()
        .with_child(status)
        .with_child(top_bar())
        .with_flex_child(movie_display(state), 1.0);

    Align::centered(layout)
}

fn movie_display(data: &HelloState) -> impl Widget<HelloState> + 'static {


    let mut column_of_films = Flex::column();
    for film in data.database.get_films(){
        column_of_films = column_of_films.with_child(film_row(&film));
    }
   let scrollable = column_of_films.scroll();

    scrollable
}

fn film_row(film: &FilmInDatabase) -> impl Widget<HelloState> + 'static {

    let correct_title = remove_unsupported_characters(&film.Title);
    if !fs::metadata("media").is_ok() {
        let _ = fs::create_dir("media");
    }
    let mut path = format!("media/{}.jpg",correct_title);
    if !Path::new(&path).exists() {
        let poster_downloaded = download_poster(&film, &path);
        if poster_downloaded.is_err() {
            path = "media/placeholder.jpg".into();
        }
    }
    let img_data = match load_image(&path){
        Ok(data) => data,
        Err(..) => get_placeholder(),
    };
    let jpg_data = ImageBuf::from_data(&img_data).unwrap();
    let poster = druid::widget::Image::new(jpg_data)
        .boxed()
        .fix_width(200.);

    let title = Label::new(film.Title.clone());
    let genre = Label::new(film.Genre.clone());
    let date_watched: Label<HelloState> = Label::new(format!("watched in date: {}", film.DateWatched.clone()));
    
    let second_part = Flex::column()
        .with_child(title)
        .with_child(genre)
        .with_child(date_watched);

    let row = Flex::row()
        .with_child(poster)
        .with_child(second_part.padding(10.))
        .fix_width(800.);

    row
}

fn remove_unsupported_characters(name: &str) -> String {
    let invalid_chars = r#"\/:*?"<>|"#; // List of invalid characters

    let correct_name = name.replace(|c| invalid_chars.contains(c), "");

    correct_name
}

fn get_placeholder() -> [u8; FILE_SIZE] {
    let path = "media/placeholder.jpg";
    if !Path::new(path).exists() {
        let mut file = File::create(path).unwrap();
        file.write_all(PLACEHOLDER_POSTER).unwrap();
    }

    load_image(path).unwrap()
}

fn download_poster(film: &FilmInDatabase, path: &str) -> Result<(), Box< dyn std::error::Error>> {

    let response = reqwest::blocking::get(&film.Poster)?;

    if !response.status().is_success(){
        return Err("failed to downlaod image".into());
    }

    let image_data = response.bytes()?;

    let mut file = File::create(path)?;
    file.write_all(&image_data)?;
    Ok(())
}

fn load_image(path: &str) -> Result<[u8; FILE_SIZE], Box<dyn std::error::Error>> {

    let mut jpg_file = std::fs::File::open(path)?;
    let mut buffer: [u8; FILE_SIZE] = [0; FILE_SIZE];
    let _ = jpg_file.read(&mut buffer)?;

    Ok(buffer)
}

pub fn top_bar() -> impl Widget<HelloState> + 'static {
    
    let textbox = TextBox::new()
        .with_placeholder("What film did you watch?")
        .expand_width()
        .lens(HelloState::text_bar); 

    let button_quick_add = Button::new("search")
        .fix_width(100.)
        .on_click(|ctx, data: &mut HelloState, env| {
            match &data.api_key{
                Some(key) => {
                    data.status_bar = "searching film".into();
                    let a_film_list = Film::search_film(&data.text_bar.trim(), key);
                    match a_film_list {
                        Ok(list) => {
                            let a_film = list.get_list()[0].clone();
                            let a_film_info = Film::film_info(&a_film.imdbID, key);
                            let _ = data.database.add_a_film(a_film_info);
                            data.status_bar= "film added".into();
                        }
                        Err(_) => {
                            data.status_bar= "film not found".into();
                        }
                    }
                     
                },
                None => request_api(ctx, data, env),
            }
       });

    let button_long_add = Button::new("change api key")
        .fix_width(130.)
        .on_click(request_api);

    Flex::row()
        .with_flex_child(textbox, 1.0)
        .with_child(button_quick_add)
        .with_child(button_long_add)
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Fill)
        .fix_height(30.)
        .padding(10.)
}

pub fn request_api(ctx: &mut EventCtx, data: &mut HelloState, env: &Env) {
    let label_message = Label::new("please add the api key");

    let tb = TextBox::new()
        .fix_size(500.,30.)
        .lens(HelloState::api_user);

    let button_close = Button::new("confirm key")
        .fix_width(100.)
        .on_click(|ctx,data: &mut HelloState,_|{
            let config = Config{
                api_key:  data.api_user.clone(),
            };

            let config_content = serde_json::to_string(&config)
                .expect("failed to serialize config.json");
            let mut config_file = File::create("config.json")
                .expect("failed to create config.json");
            config_file.write_all(config_content.as_bytes())
                .expect("failed to write to config.json");

            data.api_key= Some(data.text_bar.clone());

            ctx.submit_command(Command::new(druid::commands::CLOSE_WINDOW, (), Target::Auto));
        } );

    let column = Flex::column()
        .with_child(label_message)
        .with_child(tb)
        .with_child(button_close);

    ctx.new_sub_window(
        WindowConfig::default()
            .window_size(Size::new(700.,200.))
            .set_level(druid::WindowLevel::AppWindow), 
        column, data.clone(), env.clone());
}

pub fn read_api_key() -> Result<String,std::io::Error> {
    let mut config_file = File::open("config.json")?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;

    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config.api_key)
}
