use crate::error::show_error;
use crate::global_state::GlobalState;
use chronos::UniqueID;

use cursive::align::HAlign;
use cursive::reexports::enumset::EnumSet;
use cursive::theme::ColorStyle;
use cursive::traits::{Nameable, Resizable};
use cursive::view::SizeConstraint::{AtLeast, Fixed};
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, ListView, NamedView, PaddedView,
    SelectView, TextView,
};
use cursive::Cursive;
use livesplit_core::run::editor::{SegmentState, SelectionState};
use livesplit_core::timing::formatter::{
    none_wrapper::NoneWrapper, SegmentTime, TimeFormatter,
};

pub fn add_split(
    _s: &mut Cursive,
    splits_list: &mut SelectView<usize>,
    segment: &SegmentState,
    i: usize,
) {
    let name = format!("{:<37}", segment.name);
    let mut split_time = format!("{:>11}", segment.split_time);
    let mut segment_time = format!("{:>13}", segment.segment_time);
    let mut best_segment_time = format!("{:>13}", segment.best_segment_time);

    if split_time.trim() == "" {
        split_time = format!("{:>11}", "—");
    }
    if segment_time.trim() == "" {
        segment_time = format!("{:>13}", "—");
    }
    if best_segment_time.trim() == "" {
        best_segment_time = format!("{:>13}", "—");
    }

    let mut formatted = format!(
        "{}{}{}{}",
        name, split_time, segment_time, best_segment_time
    );

    // reduce comparisons into formatted text
    segment.comparison_times.clone().iter_mut().for_each(|c| {
        let mut comparison = format!("{:>13}", c);
        if comparison.trim() == "" {
            comparison = format!("{:>13}", "—");
        }
        formatted = format!("{}{}", formatted, comparison);
    });

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

    let state = globals.splits_editor.state();
    let formatter = NoneWrapper::new(SegmentTime::new(), "");
    let active_segment = globals.splits_editor.active_segment();
    let name = active_segment.name();
    let split_time = formatter.format(active_segment.split_time()).to_string();
    let segment_time =
        formatter.format(active_segment.segment_time()).to_string();
    let best_segment_time = formatter
        .format(active_segment.best_segment_time())
        .to_string();

    let mut save_details_list_view = ListView::new()
        .child(
            "Name",
            LinearLayout::horizontal().child(
                Button::new(name, move |s| edit_split_name_view(s))
                    .with_name("edit_split_name_button"),
            ),
        )
        .child(
            "Split Time",
            LinearLayout::horizontal().child(
                Button::new(split_time, move |s| edit_split_time_view(s))
                    .with_name("edit_split_time_button"),
            ),
        )
        .child(
            "Segment Time",
            LinearLayout::horizontal().child(
                Button::new(segment_time, move |s| edit_segment_time_view(s))
                    .with_name("edit_segment_time_button"),
            ),
        )
        .child(
            "Best Segment",
            LinearLayout::horizontal().child(
                Button::new(best_segment_time, move |s| {
                    edit_best_segment_time_view(s)
                })
                .with_name("edit_best_segment_time_button"),
            ),
        );

    // reduce comparisons into formatted text
    for c in state.comparison_names {
        let button_name =
            format!("{}_edit_comparison_time_button", c.to_string());
        let value = format!(
            "{}",
            formatter
                .format(active_segment.comparison_time(&c))
                .to_string()
        );
        save_details_list_view.add_child(
            &c.clone(),
            LinearLayout::horizontal().child(
                Button::new(value.to_string(), move |s| {
                    edit_comparison_time_view(s, &c, &value)
                })
                .with_name(button_name),
            ),
        );
    }

    // save_details_list_view.resized(AtLeast(40), AtLeast(10));

    let view = PaddedView::lrtb(0, 0, 1, 0, save_details_list_view);

    let dialog =
        Dialog::around(view)
            .title("edit split")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

pub fn edit_comparison_time_view(s: &mut Cursive, name: &str, value: &str) {
    let editor_id = UniqueID::new();
    let n = name.to_string();
    let split_name_edit_view = EditView::new()
        .content(value.to_string())
        .with_name(editor_id.to_string());

    let split_name_list_view = ListView::new()
        .child("", PaddedView::lrtb(0, 0, 0, 1, split_name_edit_view));

    let view = PaddedView::lrtb(0, 0, 1, 0, split_name_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog = Dialog::around(view)
        .title("edit split name")
        .button("save", move |s| {
            let edit_view =
                s.find_name::<EditView>(&editor_id.to_string()).unwrap();

            let value = edit_view.get_content();
            let globals = s.user_data::<GlobalState>().unwrap();

            match globals
                .splits_editor
                .active_segment()
                .parse_and_set_comparison_time(&n, &value)
            {
                Ok(_) => {
                    refresh_edit_split_times(s);
                    s.pop_layer();
                }
                Err(error) => show_error(s, &error.to_string()),
            }
        })
        .button("close", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

pub fn edit_split_name_view(s: &mut Cursive) {
    let editor_id = UniqueID::new();
    let globals = s.user_data::<GlobalState>().unwrap();
    let name = globals.splits_editor.active_segment().name().to_string();
    let split_name_edit_view = EditView::new()
        .content(name.to_string())
        .with_name(editor_id.to_string());

    let split_name_list_view = ListView::new()
        .child("", PaddedView::lrtb(0, 0, 0, 1, split_name_edit_view));

    let view = PaddedView::lrtb(0, 0, 1, 0, split_name_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog = Dialog::around(view)
        .title("edit split name")
        .button("save", move |s| {
            let edit_view =
                s.find_name::<EditView>(&editor_id.to_string()).unwrap();

            let value = edit_view.get_content();
            let globals = s.user_data::<GlobalState>().unwrap();

            globals
                .splits_editor
                .active_segment()
                .set_name(value.to_string());

            refresh_splits(s);
            match s.pop_layer() {
                _ => (),
            }
        })
        .button("close", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

pub fn edit_split_time_view(s: &mut Cursive) {
    let editor_id = UniqueID::new();
    let globals = s.user_data::<GlobalState>().unwrap();
    let formatter = NoneWrapper::new(SegmentTime::new(), "");
    let active_segment = globals.splits_editor.active_segment();
    let split_time = formatter.format(active_segment.split_time()).to_string();
    let split_time_edit_view = EditView::new()
        .content(split_time.to_string())
        .with_name(editor_id.to_string());

    let split_time_list_view = ListView::new()
        .child("", PaddedView::lrtb(0, 0, 0, 1, split_time_edit_view));

    let view = PaddedView::lrtb(0, 0, 1, 0, split_time_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog = Dialog::around(view)
        .title("edit split time")
        .button("save", move |s| {
            let edit_view =
                s.find_name::<EditView>(&editor_id.to_string()).unwrap();

            let value = edit_view.get_content();
            let globals = s.user_data::<GlobalState>().unwrap();

            match globals
                .splits_editor
                .active_segment()
                .parse_and_set_split_time(&value)
            {
                Ok(_timespan) => (),
                Err(error) => show_error(s, &error.to_string()),
            };

            refresh_edit_split_times(s);
            match s.pop_layer() {
                _ => (),
            }
        })
        .button("close", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

pub fn edit_best_segment_time_view(s: &mut Cursive) {
    let editor_id = UniqueID::new();
    let globals = s.user_data::<GlobalState>().unwrap();
    let formatter = NoneWrapper::new(SegmentTime::new(), "");
    let active_segment = globals.splits_editor.active_segment();
    let best_segment_time = formatter
        .format(active_segment.best_segment_time())
        .to_string();
    let best_segment_time_edit_view = EditView::new()
        .content(best_segment_time.to_string())
        .with_name(editor_id.to_string());

    let best_segment_time_list_view = ListView::new().child(
        "",
        PaddedView::lrtb(0, 0, 0, 1, best_segment_time_edit_view),
    );

    let view = PaddedView::lrtb(0, 0, 1, 0, best_segment_time_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog = Dialog::around(view)
        .title("edit split time")
        .button("save", move |s| {
            let edit_view =
                s.find_name::<EditView>(&editor_id.to_string()).unwrap();

            let value = edit_view.get_content();
            let globals = s.user_data::<GlobalState>().unwrap();

            match globals
                .splits_editor
                .active_segment()
                .parse_and_set_best_segment_time(&value)
            {
                Ok(_timespan) => (),
                Err(error) => show_error(s, &error.to_string()),
            };

            refresh_edit_split_times(s);
            match s.pop_layer() {
                _ => (),
            }
        })
        .button("close", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

pub fn edit_segment_time_view(s: &mut Cursive) {
    let editor_id = UniqueID::new();
    let globals = s.user_data::<GlobalState>().unwrap();
    let formatter = NoneWrapper::new(SegmentTime::new(), "");
    let active_segment = globals.splits_editor.active_segment();
    let segment_time =
        formatter.format(active_segment.segment_time()).to_string();
    let segment_time_edit_view = EditView::new()
        .content(segment_time.to_string())
        .with_name(editor_id.to_string());

    let segment_time_list_view = ListView::new()
        .child("", PaddedView::lrtb(0, 0, 0, 1, segment_time_edit_view));

    let view = PaddedView::lrtb(0, 0, 1, 0, segment_time_list_view)
        .resized(AtLeast(40), AtLeast(10));

    let dialog = Dialog::around(view)
        .title("edit split time")
        .button("save", move |s| {
            let edit_view =
                s.find_name::<EditView>(&editor_id.to_string()).unwrap();

            let value = edit_view.get_content();
            let globals = s.user_data::<GlobalState>().unwrap();

            match globals
                .splits_editor
                .active_segment()
                .parse_and_set_segment_time(&value)
            {
                Ok(_timespan) => (),
                Err(error) => show_error(s, &error.to_string()),
            };

            refresh_edit_split_times(s);
            match s.pop_layer() {
                _ => (),
            }
        })
        .button("close", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
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
    for (i, segment) in segments.iter().enumerate() {
        match segment.selected {
            SelectionState::Active | SelectionState::Selected => {
                selected_index = i
            }
            _ => (),
        }
        add_split(s, &mut splits_list, segment, i)
    }
    splits_list.set_selection(selected_index as usize);
}

pub fn refresh_splits_title(s: &mut Cursive) {
    /* splits list */
    let mut splits_title = s.find_name::<ListView>("splits_title").unwrap();

    let wrapped_layout = build_splits_title_wrapped_layout(s);
    splits_title.clear();
    splits_title.add_child("", wrapped_layout);
}

pub fn refresh_edit_split_times(s: &mut Cursive) {
    {
        let globals = s.user_data::<GlobalState>().unwrap();
        let active_segment = globals.splits_editor.active_segment();
        let formatter = NoneWrapper::new(SegmentTime::new(), "");
        let split_time =
            formatter.format(active_segment.split_time()).to_string();

        s.find_name::<Button>("edit_split_time_button")
            .unwrap()
            .set_label(split_time.to_string());
    }

    {
        let globals = s.user_data::<GlobalState>().unwrap();
        let active_segment = globals.splits_editor.active_segment();
        let formatter = NoneWrapper::new(SegmentTime::new(), "");
        let segment_time =
            formatter.format(active_segment.segment_time()).to_string();
        s.find_name::<Button>("edit_segment_time_button")
            .unwrap()
            .set_label(segment_time.to_string());
    }

    {
        let globals = s.user_data::<GlobalState>().unwrap();
        let active_segment = globals.splits_editor.active_segment();
        let formatter = NoneWrapper::new(SegmentTime::new(), "");
        let best_segment_time = formatter
            .format(active_segment.best_segment_time())
            .to_string();
        let mut button = s
            .find_name::<Button>("edit_best_segment_time_button")
            .unwrap();

        button.set_label(best_segment_time.to_string());
    }
    {
        // comparison times
        let globals = s.user_data::<GlobalState>().unwrap();
        for c in globals.splits_editor.state().comparison_names {
            let globals = s.user_data::<GlobalState>().unwrap();
            let active_segment = globals.splits_editor.active_segment();
            let formatter = NoneWrapper::new(SegmentTime::new(), "");
            let comparison_time = formatter
                .format(active_segment.comparison_time(&c))
                .to_string();

            let button_name =
                format!("{}_edit_comparison_time_button", c.to_string());
            let mut button = s.find_name::<Button>(&button_name).unwrap();

            button.set_label(comparison_time.to_string());
        }
    }

    refresh_splits(s);
}

pub fn build_splits_title(s: &mut Cursive) -> NamedView<ListView> {
    let wrapped_layout = build_splits_title_wrapped_layout(s);

    ListView::new()
        .child("", wrapped_layout)
        .with_name("splits_title")
}

pub fn build_splits_title_wrapped_layout(
    s: &mut Cursive,
) -> PaddedView<LinearLayout> {
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

    PaddedView::lrtb(0, 0, 0, 1, splits_title_layout)
}
