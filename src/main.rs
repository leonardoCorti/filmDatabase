#![allow(non_snake_case)]
use druid::{WindowDesc, LocalizedString, AppLauncher};
use filmDatabase::*;

mod Film;
mod Database;

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Film Database");

fn main() {

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
        api_user: "".into(),
        api_key,
        text_bar: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

