#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use druid::widget::{Align, Flex,TextBox, Button};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Film Database");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    api_key: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct Config{
    api_key: String,
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
        .on_click(|_, _data: &mut HelloState, _: &_| ());

    let button_long_add = Button::new("add film")
        .fix_width(100.)
        .on_click(|_, _data: &mut HelloState, _: &_| ());

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