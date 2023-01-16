use crate::global_state::GlobalState;
use crate::error::show_error;

use cursive::Cursive;
use cursive::views::{
    Button, 
    Dialog, 
    EditView, 
    LinearLayout, 
    ListView,
    PaddedView, 
    TextView, 
};
use cursive::view::SizeConstraint::{AtLeast, Full, Fixed};
use cursive::traits::{Resizable, Nameable};
use cursive::align::HAlign;

use livesplit_core::timing::formatter::{
    none_wrapper::{NoneWrapper, EmptyWrapper}, SegmentTime, TimeFormatter, Accuracy
};

pub fn edit_game_name_view(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();

    let name = globals.splits_editor.game_name();

    let menu = PaddedView::lrtb(0, 0, 1, 0,
        ListView::new()
            .child(
                "Name",
                PaddedView::lrtb(0, 0, 0, 1,
                    EditView::new().content(name).on_submit(save_game_name),
                ),
            )
            .resized(AtLeast(40), AtLeast(10)),
    );

    let dialog =
        Dialog::around(menu)
            .title("edit game name")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

pub fn save_game_name(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    globals.splits_editor.set_game_name(value);

    let mut button = s
        .find_name::<Button>("details_game_name_edit_view")
        .unwrap();

    button.set_label(value);
    match s.pop_layer() {
        _ => (),
    }
}

pub fn edit_run_category_view(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();

    let name = globals.splits_editor.category_name();

    let menu = PaddedView::lrtb(0, 0, 1, 0,
        ListView::new()
            .child(
                "Name",
                PaddedView::lrtb(0, 0, 0, 1,
                    EditView::new().content(name).on_submit(save_run_category),
                ),
            )
            .resized(AtLeast(40), AtLeast(10)),
    );

    let dialog =
        Dialog::around(menu)
            .title("edit game name")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

pub fn save_run_category(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    globals.splits_editor.set_category_name(value);

    let mut button = s
        .find_name::<Button>("details_run_category_edit_view")
        .unwrap();

    button.set_label(value);
    match s.pop_layer() {
        _ => (),
    }
}

pub fn edit_start_timer_at_view(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();

    let formatter = NoneWrapper::new(SegmentTime::new(), "");
    let name = formatter.format(globals.splits_editor.offset()).to_string();

    let menu = PaddedView::lrtb(0, 0, 1, 0,
        ListView::new()
            .child(
                "Name",
                PaddedView::lrtb(0, 0, 0, 1,
                    EditView::new()
                        .content(name)
                        .on_submit(save_start_timer_at),
                ),
            )
            .resized(AtLeast(40), AtLeast(10)),
    );

    let dialog =
        Dialog::around(menu)
            .title("edit game name")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

pub fn save_start_timer_at(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    match globals.splits_editor.parse_and_set_offset(value) {
        Ok(_timespan) => (),
        Err(error) => return show_error(s, &error.to_string()),
    }

    let formatter = SegmentTime::new();
    let value = formatter.format(globals.splits_editor.offset()).to_string();

    let mut button = s
        .find_name::<Button>("details_start_timer_at_edit_view")
        .unwrap();

    button.set_label(value);
    match s.pop_layer() {
        _ => (),
    }
}

pub fn edit_attempts_view(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();

    let name = globals.splits_editor.attempt_count();

    let menu = PaddedView::lrtb(0, 0, 1, 0,
        ListView::new()
            .child(
                "Name",
                PaddedView::lrtb(0, 0, 0, 1,
                    EditView::new()
                        .content(name.to_string())
                        .on_submit(save_attempts),
                ),
            )
            .resized(AtLeast(40), AtLeast(10)),
    );

    let dialog =
        Dialog::around(menu)
            .title("edit game name")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

pub fn save_attempts(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    match globals.splits_editor.parse_and_set_attempt_count(value) {
        Ok(_timespan) => (),
        Err(error) => return show_error(s, &error.to_string()),
    }

    let mut button =
        s.find_name::<Button>("details_attempts_edit_view").unwrap();

    button.set_label(value);
    match s.pop_layer() {
        _ => (),
    }
}

pub fn update_details_view(s: &mut Cursive) -> ListView {
    let globals = s.user_data::<GlobalState>().unwrap();
    let formatter =
        EmptyWrapper::new(SegmentTime::with_accuracy(Accuracy::Hundredths));

    let offset = formatter.format(globals.splits_editor.offset()).to_string();
    let name = globals.splits_editor.game_name();
    let category = globals.splits_editor.category_name();
    let attempts = globals.splits_editor.attempt_count().to_string();

    ListView::new()
        .child(
            "",
            LinearLayout::horizontal()
                .child(
                    TextView::new("Game Name:")
                        .h_align(HAlign::Left)
                        .resized(Full, Fixed(1)),
                )
                .child(
                    Button::new(name, |s| edit_game_name_view(s))
                        .with_name("details_game_name_edit_view"),
                ),
        )
        .child(
            "",
            LinearLayout::horizontal()
                .child(
                    TextView::new("Run Category:")
                    .h_align(HAlign::Left)
                    .resized(Full, Fixed(1)),
                )
                .child(
                    Button::new(category, |s| edit_run_category_view(s))
                        .with_name("details_run_category_edit_view"),
                ),
        )
        .child(
            "",
            LinearLayout::horizontal()
                .child(
                    TextView::new("Start Timer At:")
                    .h_align(HAlign::Left)
                    .resized(Full, Fixed(1)),
                )
                .child(
                    Button::new(offset, |s| edit_start_timer_at_view(s))
                        .with_name("details_start_timer_at_edit_view"),
                ),
        )
        .child(
            "",
            LinearLayout::horizontal()
                .child(
                    TextView::new("Attempts:")
                        .h_align(HAlign::Left)
                        .resized(Full, Fixed(1)),
                )
                .child(
                    Button::new(attempts, |s| edit_attempts_view(s))
                        .with_name("details_attempts_edit_view"),
                ),
        )
}
