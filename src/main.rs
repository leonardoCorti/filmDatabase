#![allow(non_snake_case)]
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use druid::widget::{Align, Flex,TextBox, Button, Label};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt, WindowConfig, Size, Command, Target};

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Film Database");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    api_key: Option<String>,
    api_user: String,
}
#[derive(Deserialize, Serialize)]
struct Config{
    api_key: String,
}

#[derive(Deserialize, Serialize)]
struct SearchResults{
    id: String,
    resultType: String,
    image: String,
    title: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
struct FilmList{
    searchType: String,
    expression: String,
    results: Vec<SearchResults>,
    errorMessage: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(homepage())
        .title(WINDOW_TITLE)
        .window_size((800.0, 800.0));

    let api_key = match read_api_key(){
        Ok(key) => Some(key),
        Err(_) => None
    };

    // create the initial app state
    let initial_state = HelloState {
        name: "".into(),
        api_key,
        api_user: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn homepage() -> impl Widget<HelloState> {

   let text_placeHolder = TextBox::new()
        .with_placeholder("placeholder")
        .lens(HelloState::name);

    let layout = Flex::column()
        .with_child(top_bar())
        .with_flex_child(text_placeHolder, 1.0);

    Align::centered(layout)
}

fn top_bar() -> impl Widget<HelloState> + 'static {
    
    let textbox = TextBox::new()
        .with_placeholder("What film did you watch?")
        .expand_width()
        .lens(HelloState::name);

    let button_quick_add = Button::new("quick add")
        .fix_width(100.)
        .on_click(|_, data: &mut HelloState, _: &_| {
            let list = list_of_films(data.api_key.as_ref().unwrap(), &data.name);
            let id = list.unwrap().results.get(0).unwrap().id.clone();
            println!("{}", id);
            println!("{:#?}", film_information(data.api_key.as_ref().unwrap(), &id));         
        });

    let button_long_add = Button::new("add film")
        .fix_width(100.)
        .on_click(|ctx, data: &mut HelloState, env| {
            match data.api_key{
                Some(_) => {},
                None => {
                    let label_message = Label::new("please add the api key");
                    let tb = TextBox::new().fix_size(500.,30.).lens(HelloState::api_user);
                    let button_close = Button::new("confirm key")
                        .fix_width(100.)
                        .on_click(|ctx,data: &mut HelloState,_|{
                           let config = Config{
                            api_key:  data.api_user.clone(),
                           };
                           let config_content = serde_json::to_string(&config).expect("failed to serialize config.json");
                           let mut config_file = File::create("config.json").expect("failed to create config.json");
                           config_file.write_all(config_content.as_bytes()).expect("failed to write to config.json");
                           data.api_key= Some(data.api_user.clone());
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
                },
            }
        });
    Flex::row()
        .with_flex_child(textbox, 1.0)
        .with_child(button_quick_add)
        .with_child(button_long_add)
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Fill)
        .fix_height(30.)
        .padding(10.)
}

fn read_api_key() -> Result<String,std::io::Error> {
    let mut config_file = File::open("config.json")?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;

    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config.api_key)
}

fn list_of_films(api_key:&str, name: &str) -> Result<FilmList, reqwest::Error> {
    let request = format!("https://imdb-api.com/it/API/Search/{}/{}",api_key,name);
    let body = reqwest::blocking::get(request)?.text()?;
    let list: FilmList = serde_json::from_str(&body).expect("failed to deserialize");
    Ok(list)
}

#[derive(Debug, Serialize, Deserialize)]
struct Film{
    id: String,
    title: String,
    year: String,
    image: String,
    releaseDate: String,
    runtimeMins: String,
    plot: String,
    plotLocal: Option<String>,
    directors: String,
    writers: String,
    stars: String,
    genres: String,
    companies: String,
    imDbRating: String,
    metacriticRating: String,
    wikipedia: Wikipedia,
}
#[derive(Debug, Serialize, Deserialize)]
struct Wikipedia{
    url: String,
}
impl Film{
    fn get_stars_list(&self)-> Vec<String>{
        self.stars.split(",")
            .map(|name| name.trim().to_string())
            .collect()
    }

    fn get_imdbRating(&self) -> u32{
       let raw_rating: i32 = match self.imDbRating.trim().parse() {
            Ok(num) => num,
            Err(_) => 0, 
        };
        let correct_rating: u32 = (raw_rating*10).try_into().unwrap();
        correct_rating
    }
    fn get_metacriticRating(&self) -> u32{
       match self.metacriticRating.trim().parse() {
            Ok(num) => num,
            Err(_) => 0, 
        }
    }
    fn get_wikipedia_url(&self) -> String{
        self.wikipedia.url.to_string()
    }
}

fn film_information(api_key: &str, id: &str) -> Result<Film, reqwest::Error> {
    let request = format!("https://imdb-api.com/it/API/Title/{}/{}/Wikipedia", api_key, id);
    let body = reqwest::blocking::get(request)?.text()?;
    let film_info: Film = serde_json::from_str(&body).expect("failed to deserialize film info");
    Ok(film_info)
}