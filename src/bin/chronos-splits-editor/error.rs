use cursive::Cursive;
use cursive::views::{Dialog, TextView};

pub fn show_error(s: &mut Cursive, error: &str) {
    let dialog = Dialog::around(TextView::new(error)).title("Error").button(
        "close",
        |s| {
            s.pop_layer();
        },
    );

    s.add_layer(dialog);
}
