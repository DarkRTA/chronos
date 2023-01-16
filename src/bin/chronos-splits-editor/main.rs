use clap::Parser;
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::reexports::enumset::EnumSet;
use cursive::theme::ColorStyle;
use cursive::traits::*;
use cursive::view::SizeConstraint::*;
use cursive::views::*;
use cursive::Cursive;
use livesplit_core::run::{
    editor::{SegmentState, SelectionState},
    parser::composite,
    saver::livesplit::{self, IoWrite},
    Editor,
};
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

use livesplit_core::timing::formatter::{
    none_wrapper::{EmptyWrapper, NoneWrapper},
    Accuracy, SegmentTime, TimeFormatter,
};

#[derive(Parser)]
struct Args {
    splits_file: String,
}

pub struct GlobalState {
    splits_editor: Editor,
    splits_file: String,
}

// this whole thing is a mess because cursive has some of the most atrocious
// state management i have ever seen
//
// this will eventually be rewritten to use either dioxus or intuitive

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = Path::new(&args.splits_file);
    let splits = match fs::read(path) {
        Ok(file) => {
            let parsed = composite::parse(&file, None, false)
                .expect("Not a valid splits file");
            parsed.run
        }
        Err(_) => {
            let default = chronos::DEFAULT_SPLITS;
            let parsed = composite::parse(&default, None, false)
                .expect("Not a valid splits file");
            parsed.run
        }
    };

    let editor = Editor::new(splits)?;

    let globals = GlobalState {
        splits_editor: editor,
        splits_file: args.splits_file,
    };

    let backend = {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered = cursive_buffered_backend::BufferedBackend::new(backend);
        Box::new(buffered)
    };

    let mut siv = cursive::CursiveRunner::new(Cursive::new(), backend);
    cursive::logger::init();
    siv.load_toml(chronos::CURSIVE_THEME).unwrap();

    siv.add_global_callback('~', cursive::Cursive::toggle_debug_console);
    siv.set_user_data(globals);
    main_splits(&mut siv);
    siv.run();
    Ok(())
}

#[rustfmt::skip]
fn edit_game_name_view(s: &mut Cursive) {
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

fn save_game_name(s: &mut Cursive, value: &str) {
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

#[rustfmt::skip]
fn edit_run_category_view(s: &mut Cursive) {
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

fn save_run_category(s: &mut Cursive, value: &str) {
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

#[rustfmt::skip]
fn edit_start_timer_at_view(s: &mut Cursive) {
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

fn save_start_timer_at(s: &mut Cursive, value: &str) {
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

#[rustfmt::skip]
fn edit_attempts_view(s: &mut Cursive) {
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

fn save_attempts(s: &mut Cursive, value: &str) {
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

#[rustfmt::skip]
fn main_splits(s: &mut Cursive) {
    let details = Panel::new(update_details_view(s))
        .title("details")
        .with_name("details_panel");

    let splits_title = PaddedView::lrtb(0, 0, 0, 1,
        LinearLayout::horizontal()
            .child(
                TextView::new_with_content(TextContent::new("Split Name"))
                    .style(cursive::theme::Style {
                        effects: EnumSet::only(cursive::theme::Effect::Bold),
                        color: ColorStyle::default(),
                    })
                    .h_align(HAlign::Left)
                    .resized(Fixed(36), Fixed(1)),
            )
            .child(PaddedView::lrtb(1, 0, 0, 0,
                TextView::new_with_content(TextContent::new("Split Time"))
                    .style(cursive::theme::Style {
                        effects: EnumSet::only(cursive::theme::Effect::Bold),
                        color: ColorStyle::default(),
                    })
                    .h_align(HAlign::Right)
                    .resized(Fixed(10), Fixed(1)),
            ))
            .child(PaddedView::lrtb(1, 0, 0, 0,
                TextView::new_with_content(TextContent::new("Segment Time"))
                    .style(cursive::theme::Style {
                        effects: EnumSet::only(cursive::theme::Effect::Bold),
                        color: ColorStyle::default(),
                    })
                    .h_align(HAlign::Right)
                    .resized(Fixed(12), Fixed(1)),
            ))
            .child(PaddedView::lrtb(1, 0, 0, 0,
                TextView::new_with_content(TextContent::new("Best Segment"))
                    .style(cursive::theme::Style {
                        effects: EnumSet::only(cursive::theme::Effect::Bold),
                        color: ColorStyle::default(),
                    })
                    .h_align(HAlign::Right)
                    .resized(Fixed(12), Fixed(1)),
            )),
    );

    let splits_title = ListView::new().child("", splits_title);

    let mut splits_list = SelectView::<usize>::new()
        .on_select(on_component_change)
        .on_submit(on_component_select);

    let globals = s.user_data::<GlobalState>().unwrap();

    let segments = globals.splits_editor.state().segments;
    for (i, s) in segments.iter().enumerate() {
        add_split(&mut splits_list, s, i)
    }

    let splits_list = splits_list.with_name("splits_list");

    let layout = LinearLayout::new(Orientation::Vertical)
        .child(details)
        .child(splits_title)
        .child(ScrollView::new(splits_list));

    let main = Dialog::around(layout)
        .title("chronos splits editor")
        .button("save", save_data)
        .button("close", |s| s.quit())
        .resized(Fixed(80), Full);

    s.add_layer(main);
}

fn update_details_view(s: &mut Cursive) -> ListView {
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
                    TextView::new_with_content(TextContent::new("Game Name:"))
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
                    TextView::new_with_content(TextContent::new(
                        "Run Category:",
                    ))
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
                    TextView::new_with_content(TextContent::new(
                        "Start Timer At:",
                    ))
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
                    TextView::new_with_content(TextContent::new("Attempts:"))
                        .h_align(HAlign::Left)
                        .resized(Full, Fixed(1)),
                )
                .child(
                    Button::new(attempts, |s| edit_attempts_view(s))
                        .with_name("details_attempts_edit_view"),
                ),
        )
}

fn add_split(splits_list: &mut SelectView<usize>, s: &SegmentState, i: usize) {
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
        "{}{}{}{}",
        name, split_time, segment_time, best_segment_time
    );
    splits_list.add_item(formatted, i);
}

fn on_component_change(s: &mut Cursive, v: &usize) {
    let globals = s.user_data::<GlobalState>().unwrap();
    globals.splits_editor.select_only(*v);
    refresh_splits(s);
}

fn on_component_select(s: &mut Cursive, _v: &usize) {
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

#[rustfmt::skip]
fn edit_split_menu(s: &mut Cursive) {
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

    let menu = PaddedView::lrtb(0, 0, 1, 0,
        ListView::new()
            .child(
                "Name",
                PaddedView::lrtb(0, 0, 0, 1,
                    EditView::new().content(name).on_submit(save_split_name),
                ),
            )
            .child(
                "Split Time",
                PaddedView::lrtb(0, 0, 0, 1,
                    EditView::new()
                        .content(split_time)
                        .on_submit(save_split_time),
                ),
            )
            .child(
                "Segment Time",
                PaddedView::lrtb(0, 0, 0, 1,
                    EditView::new()
                        .content(segment_time)
                        .on_submit(save_segment_time),
                ),
            )
            .child(
                "Best Segment",
                PaddedView::lrtb(0, 0, 0, 1,
                    EditView::new()
                        .content(best_segment_time)
                        .on_submit(save_best_segment_time),
                ),
            )
            .resized(AtLeast(40), AtLeast(10)),
    );

    let dialog =
        Dialog::around(menu)
            .title("edit split")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

fn save_split_name(s: &mut Cursive, value: &str) {
    let globals = s.user_data::<GlobalState>().unwrap();

    globals.splits_editor.active_segment().set_name(value);
    refresh_splits(s)
}

fn save_split_time(s: &mut Cursive, value: &str) {
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

fn save_segment_time(s: &mut Cursive, value: &str) {
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

fn save_best_segment_time(s: &mut Cursive, value: &str) {
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

fn show_error(s: &mut Cursive, error: &str) {
    let dialog = Dialog::around(TextView::new(error)).title("Error").button(
        "close",
        |s| {
            s.pop_layer();
        },
    );

    s.add_layer(dialog);
}

fn refresh_splits(s: &mut Cursive) {
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

fn save_data(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();
    let run = globals.splits_editor.run();
    let file = File::create(&globals.splits_file)
        .expect("Failed creating save file buffer");

    match livesplit::save_run(&run, IoWrite(file)) {
        Ok(_) => {
            s.add_layer(Dialog::text("Save Successful").button("quit", |s| {
                s.quit();
            }));
        }
        Err(x) => {
            s.add_layer(Dialog::text(format!("Save Failed: {}", x)).button(
                "quit",
                |s| {
                    s.quit();
                },
            ));
        }
    }
}
