use crate::error::show_error;
use crate::global_state::GlobalState;

use cursive::align::HAlign;
use cursive::traits::{Nameable, Resizable};
use cursive::view::SizeConstraint::{AtLeast, Fixed, Full};
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, ListView, PaddedView, TextView,
};
use cursive::Cursive;

use livesplit_core::timing::formatter::{
    none_wrapper::{EmptyWrapper, NoneWrapper},
    Accuracy, SegmentTime, TimeFormatter,
};

pub fn edit_game_name_view(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();

    let name = globals.splits_editor.game_name();

    let game_name_edit_view =
        EditView::new().content(name).on_submit(save_game_name);

    let game_name_list_view = ListView::new()
        .child("", PaddedView::lrtb(0, 0, 0, 1, game_name_edit_view));

    let view = PaddedView::lrtb(0, 0, 1, 0, game_name_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog =
        Dialog::around(view)
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

    let run_category_edit_view =
        EditView::new().content(name).on_submit(save_run_category);

    let run_category_list_view = ListView::new()
        .child("", PaddedView::lrtb(0, 0, 0, 1, run_category_edit_view));

    let view = PaddedView::lrtb(0, 0, 1, 0, run_category_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog =
        Dialog::around(view)
            .title("edit run category")
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

    let start_timer_at_edit_view =
        EditView::new().content(name).on_submit(save_start_timer_at);

    let start_timer_at_list_view = ListView::new()
        .child("", PaddedView::lrtb(0, 0, 0, 1, start_timer_at_edit_view));

    let view = PaddedView::lrtb(0, 0, 1, 0, start_timer_at_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog =
        Dialog::around(view)
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

    let attempts_edit_view = EditView::new()
        .content(name.to_string())
        .on_submit(save_attempts);

    let attempts_list_view = ListView::new()
        .child("", PaddedView::lrtb(0, 0, 0, 1, attempts_edit_view));

    let view = PaddedView::lrtb(0, 0, 1, 0, attempts_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog =
        Dialog::around(view)
            .title("edit attempt count")
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

    let game_name_layout = build_layout(
        "Game Name:",
        name,
        "details_game_name_edit_view",
        edit_game_name_view,
    );

    let run_category_layout = build_layout(
        "Run Category:",
        category,
        "details_run_category_edit_view",
        edit_run_category_view,
    );

    let start_timer_at_layout = build_layout(
        "Start Timer At:",
        &offset,
        "details_start_timer_at_edit_view",
        edit_start_timer_at_view,
    );

    let attempts_layout = build_layout(
        "Attempts:",
        &attempts.to_string(),
        "details_attempts_edit_view",
        edit_attempts_view,
    );

    ListView::new()
        .child("", game_name_layout)
        .child("", run_category_layout)
        .child("", start_timer_at_layout)
        .child("", attempts_layout)
}

fn build_layout(
    label: &str,
    value: &str,
    name: &str,
    f: fn(&mut Cursive),
) -> LinearLayout {
    let label = TextView::new(label)
        .h_align(HAlign::Left)
        .resized(Full, Fixed(1));

    let button = Button::new(value, move |s| f(s)).with_name(name);

    LinearLayout::horizontal().child(label).child(button)
}
