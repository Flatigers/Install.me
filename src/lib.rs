mod ui;
mod config;

pub fn run() {

    let _ = ui::start::run_ui()
        .expect("Error loading UI");


}