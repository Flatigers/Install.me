use std::ops::Mul;

use fltk_theme::widget_schemes::aqua::frames;
use fltk_theme::colors::aqua::{
    light,
    dark,
};

use fltk::app::Sender;
use fltk::button;
use fltk::frame::Frame;
use fltk::group::{Flex, Pack, PackType};
use fltk::image::SharedImage;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use fltk_grid::Grid;

use crate::ui::events::{PagesRoll, UIMessage};

/// Support page status
static mut page_roller: PagesRoll = PagesRoll::HomePage;

/// Page which fills Main Window
pub fn page_win(tx: Sender<UIMessage>, width: i32, height: i32) {

    /// Calculate height of each page
    let per = height / 10;
    let (a_height, b_height) = (per.mul(7), per.mul(3));

    let mut layout = Pack::default_fill()
        .with_size(width, height)
        .with_type(PackType::Vertical);


    /// Above
    let mut above = Pack::default_fill()
        .with_size(width, a_height)
        .with_type(PackType::Vertical);

    page_switcher(a_height);

    above.end();
    above.show();

    /// Below
    let mut below = Pack::default_fill()
        .with_size(width, b_height)
        .with_type(PackType::Vertical);

    button_switcher(tx.clone(), b_height);

    below.end();
    below.show();


    layout.end();
    layout.show();
}


/// Page the above part
fn page_switcher(height: i32) {
    unsafe {
        match page_roller {
            PagesRoll::HomePage => {
                home_view(height);
            }
            PagesRoll::InfoPage => {}
            PagesRoll::LicensePage => {}
            PagesRoll::InstallPage => {}
            PagesRoll::FinishPage => {}
        }
    }
}

fn home_view(height: i32) {

    let per = height /10;

    let mut layout = Flex::default_fill();


    let img_r = per * 4;
    let mut image_frame = Frame::default()
        .with_size(img_r, img_r)
        .center_of_parent();


    layout.end();
    layout.show();
}

fn info_view() {

}

fn license_view() {

}

fn install_view() {

}

fn finish_view() {

}



/// Page the below part
fn button_switcher(tx: Sender<UIMessage>, height: i32) {

    let mut layout = Grid::default_fill();
    layout.debug(false);
    layout.set_layout(3, 8);

    let mut n_btn = button::Button::default()
        .with_label("Back");
    n_btn.set_frame(frames::OS_DEFAULT_BUTTON_UP_BOX);
    n_btn.set_color(*light::controlAccentColor);
    n_btn.set_selection_color(*light::controlColor);
    n_btn.set_label_color(*light::textBackgroundColor);


    let mut p_btn = button::Button::default()
        .with_label("Go");
    p_btn.set_frame(frames::OS_DEFAULT_BUTTON_UP_BOX);
    p_btn.set_color(*light::controlAccentColor);
    p_btn.set_selection_color(*light::controlColor);
    p_btn.set_label_color(*light::textBackgroundColor);

    layout.insert(&mut p_btn, 1, 6);

    unsafe {
        match page_roller {
            PagesRoll::HomePage => {
                p_btn.set_callback(move |b| {
                    page_roller = PagesRoll::InfoPage;
                    b.emit(tx, UIMessage::NextPage);
                    b.set_label("Hello");
                    println!("{:?}", page_roller);
                });
                println!("{:?}", page_roller);
            }
            PagesRoll::InfoPage => {
                layout.insert(&mut p_btn, 1, 1);
                n_btn.set_callback(move |b| {
                    page_roller = PagesRoll::HomePage;
                    b.emit(tx, UIMessage::PrePage);
                    println!("{:?}", page_roller);
                });
                p_btn.set_callback(move |b| {
                    page_roller = PagesRoll::LicensePage;
                    b.emit(tx, UIMessage::NextPage);
                    println!("{:?}", page_roller);
                });
                println!("{:?}", page_roller);
            }
            PagesRoll::LicensePage => {}
            PagesRoll::InstallPage => {}
            PagesRoll::FinishPage => {}
        }
    }



}
