// #![allow(unused)]

use std::error::Error;

use fltk_theme::{widget_schemes, color_themes, ColorTheme, WidgetScheme, SchemeType};

use fltk::{prelude::*, app, window};
use fltk::app::{Receiver, Sender};
use fltk::enums::Color;
use fltk::group::{Pack, PackType};
use fltk_evented::Listener;
use fltk_theme::colors::aqua::light::{controlAccentColor, labelColor, windowBackgroundColor};

use crate::ui::events::*;
use crate::ui::pages::page_win;


pub fn run_ui() -> Result<(), Box<dyn Error>> {

    /// Message channel init
    let (tx, rx): (Sender<UIMessage>, Receiver<UIMessage>) = app::channel();


    let app = app::App::default();
    g_styling();
    let bg = windowBackgroundColor.to_rgb();
    app::background(bg.0, bg.1, bg.2);
    let ctrl = controlAccentColor.to_rgb();
    app::background2(ctrl.0, ctrl.1, ctrl.2);
    let lbl = labelColor.to_rgb();
    app::foreground(lbl.0, lbl.1, lbl.2);
    app::set_color(Color::Selection, 255, 255, 255);


    /// App Window View
    let mut win = window::Window::default()
        .with_label("Install.me")
        .with_size(600, 360)
        .center_screen();

    page_win(tx.clone(), win.width(), win.height());

    win.end();
    win.show();


    /// Handle message in event loop
    while app.wait() {
        if let Some(msg) = rx.recv() {
            println!("msg: {:?}", msg);
            handler(msg);

            app::awake();
            app::flush();
            app::redraw();
        }


    }

    Ok(())
}

/// Styling global Theme&Scheme
fn g_styling() {
    let color_theme = ColorTheme::new(color_themes::BLACK_THEME);
    color_theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
}

/// Message Handler
fn handler(msg: UIMessage) {

    match msg {
        UIMessage::PrePage => {

        }
        UIMessage::NextPage => {

        }
        UIMessage::Page(_pages) => {

        }
    }
}