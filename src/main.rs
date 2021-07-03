use druid::{AppLauncher, Data, EventCtx, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use druid::widget::{Align, Button, Flex, TextBox};
use run_script::ScriptOptions;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<BrightnessState> = LocalizedString::new("Brightness");

#[derive(Clone, Data, Lens)]
struct BrightnessState {
    level: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        // TODO: adjust this, possibly make it smaller
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = BrightnessState {
        level: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<BrightnessState> {
    // a label that will determine its text based on the current app data.
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Brightness level (0 - 255)")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(BrightnessState::level);

    let button = Button::new("Adjust")
        .on_click(|_ctx, data: &mut String, _env| run_bash(data.to_string()))
        .padding(10.0)
        .lens(BrightnessState::level);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button);

    // center the two widgets in the available space
    Align::centered(layout)
}

// TODO: rename function
fn run_bash(data: String) -> () {

    println!("DATA {}", data);

    let options = ScriptOptions::new();
    let args = vec![];

    // run the script and get the script execution output
    let (code, output, error) = run_script::run(
        // TODO: sanitize input
        &format!("echo {} > /sys/class/backlight/rpi_backlight/brightness", data),
        &args,
        &options,
    )
        .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);

    // TODO: conditionally print this
    println!("Error: {}", error);
}