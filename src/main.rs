#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use druid::widget::{Align, Flex,TextBox, Button, Label};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt, WindowConfig, Size};

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
        .on_click(|_, _data: &mut HelloState, _: &_| ());

    let button_long_add = Button::new("add film")
        .fix_width(100.)
        .on_click(|ctx, data: &mut HelloState, env| {
            match data.api_key{
                Some(_) => todo!(),
                None => {
                    let label_message = Label::new("please add the api key");
                    let tb = TextBox::new().fix_size(500.,30.).lens(HelloState::api_user);
                    let button_close = Button::new("confirm key")
                        .fix_width(100.)
                        .on_click(|_,data: &mut HelloState,_|{
                           let config = Config{
                            api_key:  data.api_user.clone(),
                           };
                           let config_content = serde_json::to_string(&config).expect("failed to serialize config.json");
                           let mut config_file = File::create("config.json").expect("failed to create config.json");
                           config_file.write_all(config_content.as_bytes()).expect("failed to write to config.json");
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