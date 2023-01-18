use super::splits;
use crate::error::show_error;
use crate::global_state::GlobalState;
use chronos::UniqueID;

use cursive::traits::{Nameable, Resizable};
use cursive::views::{Button, Dialog, EditView, SelectView};
use cursive::Cursive;

pub fn build_comparison_button(_s: &mut Cursive) -> Button {
    Button::new("Edit Comparisons", |s| comparisons_menu(s))
}

pub fn comparisons_menu(s: &mut Cursive) {
    let mut menu = SelectView::new();
    menu.add_item("Add New Comparison", 1);
    menu.add_item("Generate Goal Comparison", 2);
    menu.add_item("Rename Comparison", 3);
    menu.add_item("Remove Comparison", 4);

    menu.set_on_submit(|s, v| {
        match v {
            1 => add_new_comparison(s),
            2 => generate_goal_comparison(s),
            3 => rename_comparison(s),
            4 => remove_comparison(s),
            _ => unreachable!(),
        }
        // refresh_layout(s);
        // s.pop_layer();
    });

    let dialog =
        Dialog::around(menu)
            .title("Edit Comparisons")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

fn add_new_comparison(s: &mut Cursive) {
    let editor_id = UniqueID::new();
    let edit_view = EditView::new()
        .with_name(editor_id.to_string())
        .fixed_width(20);

    let dialog = Dialog::new()
        .title("Add New Comparison")
        .padding_lrtb(1, 1, 1, 0)
        .content(edit_view)
        .button("Ok", move |s| {
            let edit_view =
                s.find_name::<EditView>(&editor_id.to_string()).unwrap();

            let globals = s.user_data::<GlobalState>().unwrap();

            match globals
                .splits_editor
                .add_comparison(edit_view.get_content().to_string())
            {
                Ok(_v) => {
                    splits::refresh_splits(s);
                    splits::refresh_splits_title(s);
                    s.pop_layer();
                }
                Err(error) => {
                    show_error(s, &error.to_string());
                }
            }
        });

    s.add_layer(dialog);
}

fn generate_goal_comparison(s: &mut Cursive) {
    let editor_id = UniqueID::new();
    let edit_view = EditView::new()
        .with_name(editor_id.to_string())
        .fixed_width(20);

    let dialog = Dialog::new()
        .title("Enter Goal Time")
        .padding_lrtb(1, 1, 1, 0)
        .content(edit_view)
        .button("Ok", move |s| {
            let edit_view =
                s.find_name::<EditView>(&editor_id.to_string()).unwrap();

            let globals = s.user_data::<GlobalState>().unwrap();

            let content = edit_view.get_content().to_string();

            match globals
                .splits_editor
                .parse_and_generate_goal_comparison(&content)
            {
                Ok(_v) => {
                    splits::refresh_splits(s);
                    splits::refresh_splits_title(s);
                    s.pop_layer();
                }
                Err(error) => {
                    show_error(s, &error.to_string());
                }
            }
        });

    s.add_layer(dialog);
}

fn remove_comparison(s: &mut Cursive) {
    let mut select_view = SelectView::<String>::new();

    let globals = s.user_data::<GlobalState>().unwrap();

    for (_i, c) in globals
        .splits_editor
        .state()
        .comparison_names
        .iter()
        .enumerate()
    {
        select_view.add_item_str(c.to_string());
    }

    let select_view = select_view.on_submit(|s, item| {
        let globals = s.user_data::<GlobalState>().unwrap();
        globals.splits_editor.remove_comparison(&item);
        splits::refresh_splits(s);
        splits::refresh_splits_title(s);
        s.pop_layer();
    });

    let dialog = Dialog::new()
        .title("Remove Comparison")
        .padding_lrtb(1, 1, 1, 0)
        .content(select_view)
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

fn rename_comparison(s: &mut Cursive) {
    let mut select_view = SelectView::<String>::new();
    let globals = s.user_data::<GlobalState>().unwrap();

    for (_i, c) in globals
        .splits_editor
        .state()
        .comparison_names
        .iter()
        .enumerate()
    {
        select_view.add_item_str(c.to_string());
    }

    let select_view = select_view.on_submit(|s, item: &String| {
        let editor_id = UniqueID::new();
        let edit_view = EditView::new()
            .content(item.to_string().clone())
            .with_name(editor_id.to_string());

        let item = item.clone();
        let dialog = Dialog::new()
            .title("Rename Comparison")
            .padding_lrtb(1, 1, 1, 0)
            .content(edit_view)
            .button("save", move |s| {
                let view =
                    s.find_name::<EditView>(&editor_id.to_string()).unwrap();

                let globals = s.user_data::<GlobalState>().unwrap();
                match globals.splits_editor.rename_comparison(
                    &item.to_string(),
                    &view.get_content().to_string(),
                ) {
                    Ok(_v) => {
                        splits::refresh_splits(s);
                        splits::refresh_splits_title(s);
                        s.pop_layer();
                        s.pop_layer();
                    }
                    Err(error) => {
                        show_error(s, &error.to_string());
                    }
                }
            })
            .button("cancel", |s| {
                s.pop_layer();
            });

        s.add_layer(dialog);
    });

    let dialog = Dialog::new()
        .title("Enter your name")
        .padding_lrtb(1, 1, 1, 0)
        .content(select_view)
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}
