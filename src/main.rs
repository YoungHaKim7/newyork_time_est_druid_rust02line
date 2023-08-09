use druid::{AppLauncher, WindowDesc};

use data::TimeState;
use ui::build_root_widget;

mod data;
mod ui;

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("NewYork_Time: EST!")
        .window_size((160.0, 90.0))
        .resizable(false);

    // create the initial app state
    let initial_state: TimeState = TimeState {
        time: "".to_string(),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
