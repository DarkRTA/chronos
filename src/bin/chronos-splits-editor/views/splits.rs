use crate::error::show_error;
use crate::global_state::GlobalState;

use cursive::align::HAlign;
use cursive::reexports::enumset::EnumSet;
use cursive::theme::ColorStyle;
use cursive::traits::Resizable;
use cursive::view::SizeConstraint::{AtLeast, Fixed};
use cursive::views::{
    Dialog, EditView, LinearLayout, ListView, PaddedView, SelectView, TextView,
};
use cursive::Cursive;
use livesplit_core::run::editor::{SegmentState, SelectionState};
use livesplit_core::timing::formatter::{
    none_wrapper::NoneWrapper, SegmentTime, TimeFormatter,
};

pub fn add_split(
    splits_list: &mut SelectView<usize>,
    s: &SegmentState,
    i: usize,
) {
    let name = format!("{:<37}", s.name);
    let mut split_time = format!("{:>11}", s.split_time);
    let mut segment_time = format!("{:>13}", s.segment_time);
    let mut best_segment_time = format!("{:>13}", s.best_segment_time);

    if split_time.trim() == "" {
        split_time = format!("{:>11}", "—");
    }
    if segment_time.trim() == "" {
        segment_time = format!("{:>13}", "—");
    }
    if best_segment_time.trim() == "" {
        best_segment_time = format!("{:>13}", "—");
    }

    let formatted = format!(
        "{}{}{}{}{}{}{}",
        name, split_time, segment_time, best_segment_time, best_segment_time, best_segment_time, best_segment_time
    );
    splits_list.add_item(formatted, i);
}

pub fn on_splits_change(s: &mut Cursive, v: &usize) {
    let globals = s.user_data::<GlobalState>().unwrap();
    globals.splits_editor.select_only(*v);
    refresh_splits(s);
}

pub fn on_splits_select(s: &mut Cursive, _v: &usize) {
    let globals = s.user_data::<GlobalState>().unwrap();
    let mut menu = SelectView::new();

    menu.add_item("Edit Segment", 0);
    menu.add_item("Insert Above", 1);
    menu.add_item("Insert Below", 2);

    if globals.splits_editor.can_move_segments_up() {
        menu.add_item("Move Up", 3);
    }
    if globals.splits_editor.can_move_segments_down() {
        menu.add_item("Move Down", 4);
    }
    if globals.splits_editor.can_remove_segments() {
        menu.add_item("Remove", 5);
    }

    menu.set_on_submit(|s, v| {
        let globals = s.user_data::<GlobalState>().unwrap();
        match v {
            0 => {
                s.pop_layer();
                edit_split_menu(s);
                return;
            }
            1 => globals.splits_editor.insert_segment_above(),
            2 => globals.splits_editor.insert_segment_below(),
            3 => globals.splits_editor.move_segments_up(),
            4 => globals.splits_editor.move_segments_down(),
            5 => globals.splits_editor.remove_segments(),
            _ => unreachable!(),
        }
        refresh_splits(s);
        s.pop_layer();
    });

    let dialog =
        Dialog::around(menu)
            .title("split actions")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog)
}

pub fn edit_split_menu(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();

    let formatter = NoneWrapper::new(SegmentTime::new(), "");
    let active_segment = globals.splits_editor.active_segment();
    let name = active_segment.name();
    let split_time = formatter.format(active_segment.split_time()).to_string();
    let segment_time =
        formatter.format(active_segment.segment_time()).to_string();
    let best_segment_time = formatter
        .format(active_segment.best_segment_time())
        .to_string();

    let save_split_name_edit_view =
        EditView::new().content(name).on_submit(save_split_name);
    let save_split_time_edit_view = EditView::new()
        .content(split_time)
        .on_submit(save_split_time);
    let save_segment_time_edit_view = EditView::new()
        .content(segment_time)
        .on_submit(save_segment_time);
    let best_segment_time_edit_view = EditView::new()
        .content(best_segment_time)
        .on_submit(save_best_segment_time);

    let save_details_list_view = ListView::new()
        .child(
            "Name",
            PaddedView::lrtb(0, 0, 0, 1, save_split_name_edit_view),
        )
        .child(
            "Split Time",
            PaddedView::lrtb(0, 0, 0, 1, save_split_time_edit_view),
        )
        .child(
            "Segment Time",
            PaddedView::lrtb(0, 0, 0, 1, save_segment_time_edit_view),
        )
        .child(
            "Best Segment",
            PaddedView::lrtb(0, 0, 0, 1, best_segment_time_edit_view),
        )
        .resized(AtLeast(40), AtLeast(10));

    let view = PaddedView::lrtb(0, 0, 1, 0, save_details_list_view);

    let dialog =
        Dialog::around(view)
            .title("edit split")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

pub fn save_split_name(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    globals.splits_editor.active_segment().set_name(value);
    refresh_splits(s)
}

pub fn save_split_time(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    match globals
        .splits_editor
        .active_segment()
        .parse_and_set_split_time(value)
    {
        Ok(_timespan) => (),
        Err(error) => show_error(s, &error.to_string()),
    };
    refresh_splits(s)
}

pub fn save_segment_time(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    match globals
        .splits_editor
        .active_segment()
        .parse_and_set_segment_time(value)
    {
        Ok(_timespan) => (),
        Err(error) => show_error(s, &error.to_string()),
    };
    refresh_splits(s)
}

pub fn save_best_segment_time(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    match globals
        .splits_editor
        .active_segment()
        .parse_and_set_best_segment_time(value)
    {
        Ok(_timespan) => (),
        Err(error) => show_error(s, &error.to_string()),
    };
    refresh_splits(s)
}

pub fn refresh_splits(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();
    let state = globals.splits_editor.state();

    /* splits list */
    let mut splits_list =
        s.find_name::<SelectView<usize>>("splits_list").unwrap();

    splits_list.clear();

    let segments = state.segments;
    let mut selected_index = 0;
    for (i, s) in segments.iter().enumerate() {
        match s.selected {
            SelectionState::Active | SelectionState::Selected => {
                selected_index = i
            }
            _ => (),
        }
        add_split(&mut splits_list, s, i)
    }
    splits_list.set_selection(selected_index as usize);
}

pub fn build_splits_title(s: &mut Cursive) -> ListView {
    let split_name_text_view = TextView::new("Split Name")
        .style(cursive::theme::Style {
            effects: EnumSet::only(cursive::theme::Effect::Bold),
            color: ColorStyle::default(),
        })
        .h_align(HAlign::Left)
        .resized(Fixed(36), Fixed(1));

    let split_time_text_view = TextView::new("Split Time")
        .style(cursive::theme::Style {
            effects: EnumSet::only(cursive::theme::Effect::Bold),
            color: ColorStyle::default(),
        })
        .h_align(HAlign::Right)
        .resized(Fixed(10), Fixed(1));

    let segment_time_text_view = TextView::new("Segment Time")
        .style(cursive::theme::Style {
            effects: EnumSet::only(cursive::theme::Effect::Bold),
            color: ColorStyle::default(),
        })
        .h_align(HAlign::Right)
        .resized(Fixed(12), Fixed(1));

    let best_segment_text_view = TextView::new("Best Segment")
        .style(cursive::theme::Style {
            effects: EnumSet::only(cursive::theme::Effect::Bold),
            color: ColorStyle::default(),
        })
        .h_align(HAlign::Right)
        .resized(Fixed(12), Fixed(1));

    let mut splits_title_layout = LinearLayout::horizontal()
        .child(split_name_text_view)
        .child(PaddedView::lrtb(1, 0, 0, 0, split_time_text_view))
        .child(PaddedView::lrtb(1, 0, 0, 0, segment_time_text_view))
        .child(PaddedView::lrtb(1, 0, 0, 0, best_segment_text_view));

    let globals = s.user_data::<GlobalState>().unwrap();
    let comparisons = globals.splits_editor.state().comparison_names;
    for c in comparisons {
        let comparison_view = TextView::new(c)
            .style(cursive::theme::Style {
                effects: EnumSet::only(cursive::theme::Effect::Bold),
                color: ColorStyle::default(),
            })
        .h_align(HAlign::Right)
        .resized(Fixed(12), Fixed(1));

        let wrapped_comparison = PaddedView::lrtb(1, 0, 0, 0, comparison_view);

        splits_title_layout.add_child(wrapped_comparison)
    }

    let wrapped_layout = PaddedView::lrtb(0, 0, 0, 1, splits_title_layout);

    ListView::new().child("", wrapped_layout)
}
