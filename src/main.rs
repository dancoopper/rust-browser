//TODO: I have the UI down I think, now I need to do the rust code
// I am going to take a brake from this project for a little because the whole point was to have fun and learn rust

// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::{LogicalSize, ToSharedString};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    // ui.on_request_update_name({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         let s: String = format!("{:?}", ui.window().size()); // remeber this shit
    //         ui.set_name(s.to_shared_string());
    //     }
    // });

    // ui.on_close({
    //     move || {
    //         slint::quit_event_loop().unwrap();
    //     }
    // });

    ui.window().set_size(LogicalSize::new(1138.0, 688.0));
    ui.run()?;

    Ok(())
}
