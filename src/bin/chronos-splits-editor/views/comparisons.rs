use super::splits;
use crate::error::show_error;
use crate::global_state::GlobalState;
use chronos::UniqueID;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, PaddedView, RadioButton,
    RadioGroup, SelectView,
};
use cursive::Cursive;
use livesplit_core::util::PopulateString;

pub fn build_comparison_button(s: &mut Cursive) -> Button {
    Button::new("Edit Comparisons", |s| comparisons_menu(s))
}

pub fn comparisons_menu(s: &mut Cursive) {
    let mut menu = SelectView::new();
    menu.add_item("Add New Comparison", 1);
    menu.add_item("Generate Goal Comparison", 2);
    menu.add_item("Rename Comparison", 3);
    menu.add_item("Remove Comparison", 4);

    menu.set_on_submit(|s, v| {
        let globals = s.user_data::<GlobalState>().unwrap();
        match v {
            1 => add_new_comparison(s),
            // 1 => globals.splits_editor.remove_component(),
            // 2 => globals.splits_editor.duplicate_component(),
            // 3 => globals.splits_editor.move_component_up(),
            // 4 => globals.splits_editor.move_component_down(),
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
        .title("Enter your name")
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