#![allow(non_snake_case)]
use druid::{WindowDesc, LocalizedString, AppLauncher};
use filmDatabase::*;

use crate::Film::{search_film, film_info};

mod Film;

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Film Database");

fn main() {

    println!("{:#?}", search_film("blade runner", "6f1e323f"));
    for _ in 0..50 {
        println!(" ");
    }
    println!("{:#?}", film_info("tt0083658","6f1e323f"));

    // describe the main window
    let main_window = WindowDesc::new(homepage())
        .title(WINDOW_TITLE)
        .window_size((800.0, 800.0));

    let api_key = match read_api_key() {
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


