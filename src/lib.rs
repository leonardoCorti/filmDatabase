#![allow(non_snake_case)]

use std::fs::File;
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

#[derive(Clone, Data, Lens)]
pub struct HelloState {
    pub api_user: String,
    pub api_key: Option<String>,
    pub text_bar: String,
    pub database: Database::Database,
}

#[derive(Deserialize, Serialize)]
struct Config{
    api_key: String,
}

pub fn homepage(state: &HelloState) -> impl Widget<HelloState> {

   let text_placeHolder = TextBox::new()
        .with_placeholder("placeholder")
        .lens(HelloState::text_bar);

    let layout = Flex::column()
        .with_child(top_bar())
        .with_child(test_image(state))
        .with_flex_child(text_placeHolder, 1.0);

    Align::centered(layout)
}

fn test_image(data: &HelloState) -> impl Widget<HelloState> + 'static {

    // let buffer = load_image("media/Blade Runner.jpg");
    // let jpg_data = ImageBuf::from_data(&buffer).unwrap();
    // let img = druid::widget::Image::new(jpg_data)
    //     .fill_mode(druid::widget::FillStrat::Fill)
    //     .interpolation_mode(druid::piet::InterpolationMode::Bilinear); 
    // img


    // film_row(FilmInDatabase{
    //     Title: "The Whale".to_string(),
    //     Year: "".to_string(),
    //     Released: "".to_string(),
    //     Runtime: "a lot of time".to_string(),
    //     Genre: "noir, cyberpunk".to_string(),
    //     Metascore: "".to_string(),
    //     Poster: "https://m.media-amazon.com/images/M/MV5BZDQ4Njg4YTctNGZkYi00NWU1LWI4OTYtNmNjOWMyMjI1NWYzXkEyXkFqcGdeQXVyMTA3MDk2NDg2._V1_SX300.jpg".to_string(),
    //     DateWatched: "".to_string(),
    // })

    let mut column_of_films = Flex::column();
    for film in data.database.get_films(){
        column_of_films = column_of_films.with_child(film_row(&film));
    }
    column_of_films
}

fn film_row(film: &FilmInDatabase) -> impl Widget<HelloState> + 'static {

    let path = format!("media/{}.jpg",film.Title);
    if !Path::new(&path).exists() {
        let _ = download_poster(&film, &path);
        //TODO: should add standard poster if this fails
    }
    let img_data = load_image(&path);
    let jpg_data = ImageBuf::from_data(&img_data).unwrap();
    let poster = druid::widget::Image::new(jpg_data)
        .boxed()
        .fix_width(200.)
        .fix_height(600.);

    let title = Label::new(film.Title.clone());
    let genre = Label::new(film.Genre.clone());
    
    let second_part = Flex::column()
        .with_child(title)
        .with_child(genre);

    Flex::row()
        .with_child(poster)
        .with_child(second_part)
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

fn load_image(path: &str) -> [u8; FILE_SIZE] {

    let mut jpg_file = std::fs::File::open(path).unwrap();
    let mut buffer: [u8; FILE_SIZE] = [0; FILE_SIZE];
    let _ = jpg_file.read(&mut buffer).unwrap();

    buffer
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
                Some(_key) => {},
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
                api_key:  data.text_bar.clone(),
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
