mod structures;

use druid::widget::{Align, Button, Flex, Label, TextBox};
use druid::{AppLauncher, Env, LocalizedString, Widget, WindowDesc, WidgetExt};
use structures::{HelloState, p};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    let button = Button::new("Hi")
        .on_click(|_esc, data: &mut HelloState, _env | {
            println!("{:?}",(*data).name);
            (*data).name = format!("Szczurwysynu");
            p();
        })
        .padding(0.5);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button);

    // center the two widgets in the available space
    Align::centered(layout)
}

#[cfg(test)]
mod tests {
    use crate::HelloState;
    #[test]
    fn hello_state_test() {
        let data: HelloState = HelloState {
            name: "HelloStateString".to_string()
        };
        assert_eq!("HelloStateString", data.name);
    }

    #[test]
    fn hello_state_rewrite_test() {
        let mut data: HelloState = HelloState {
            name: "HelloStateString".to_string()
        };
        data.name = "Hi".to_string();
        assert_eq!(data.name,"Hi")
    }

}