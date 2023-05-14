use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Data, Lens};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data = String::new();
    let data = AppState { name: "hello world".to_string() };
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}
#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
}

fn ui_builder() -> impl Widget<AppState> {
    let text_box = TextBox::new()
        .with_placeholder("Enter text")
        .lens(AppState::name);
    let button = Button::new("Submit").on_click(|_,_,_|{

    });
    Flex::row()
        .with_child(text_box)
        .with_spacer(8.0)
        .with_child(button)
        .center()
}
