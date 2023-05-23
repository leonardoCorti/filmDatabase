#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
use druid::widget::{Align, Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Film Database");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}
fn main() {
    // describe the main window
    let main_window = WindowDesc::new(root_widget())
        .title(WINDOW_TITLE)
        .window_size((800.0, 800.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn root_widget() -> impl Widget<HelloState> {

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

    let button = Button::new("add film")
        .fix_width(100.)
        .on_click(|_, data: &mut HelloState, _: &_| ());

    Flex::row()
        .with_flex_child(textbox, 1.0)
        .with_child(button)
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Fill)
        .fix_height(30.)
        .padding(10.)
}