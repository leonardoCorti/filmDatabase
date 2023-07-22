#![allow(non_snake_case)]

use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use druid::widget::{Align, Flex,TextBox, Button, Label};
use druid::{Data, Lens, Widget, WidgetExt, WindowConfig, Size, Command, Target, EventCtx, Env, ImageBuf};
use image;
const FILE_SIZE: usize = 100000;

#[derive(Clone, Data, Lens)]
pub struct HelloState {
    pub api_user: String,
    pub api_key: Option<String>,
    pub text_bar: String,
}

#[derive(Deserialize, Serialize)]
struct Config{
    api_key: String,
}

pub fn homepage() -> impl Widget<HelloState> {

   let text_placeHolder = TextBox::new()
        .with_placeholder("placeholder")
        .lens(HelloState::text_bar);

    let layout = Flex::column()
        .with_child(top_bar())
        .with_child(test_image())
        .with_flex_child(text_placeHolder, 1.0);

    Align::centered(layout)
}

fn test_image() -> impl Widget<HelloState> + 'static {

    let buffer = load_image("media/Blade Runner.jpg");
    let jpg_data = ImageBuf::from_data(&buffer).unwrap();
    let img = druid::widget::Image::new(jpg_data)
        .fill_mode(druid::widget::FillStrat::Fill)
        .interpolation_mode(druid::piet::InterpolationMode::Bilinear); 

    img
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
