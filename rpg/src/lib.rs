use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc, UnitPoint};

use wasm_bindgen::prelude::*;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens)]
struct HelloState {
    repo_label: String,
    name: String,
}

// This wrapper function is the primary modification we're making to the vanilla
// hello.rs example.
#[wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main()
}

pub fn main() {
    // describe the main window
    //
    // Window title is set in index.html and window size is ignored on the web,
    // so can we leave those off.
    let main_window = WindowDesc::new(build_root_widget());

    // create the initial app state
    let initial_state = HelloState {
        repo_label: "Repositories".into(),
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

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    let repo_label = Label::new(|data: &HelloState, _env: &Env| format!("{}", data.repo_label));
    
    let leftbar = Flex::column()
        .with_child(repo_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Align::centered(Label::new("Repo 1")));
    let app = Flex::row()
        .with_child(leftbar)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(layout)
        .align_vertical(UnitPoint::TOP_LEFT)
        .padding(12.0);
    // center the two widgets in the available space
    app
}
