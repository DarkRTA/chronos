use super::comparisons;
use super::details;
use super::splits;
use super::timing;
use crate::save;

use crate::global_state::GlobalState;

use cursive::direction::Orientation;
use cursive::traits::{Nameable, Resizable};
use cursive::view::SizeConstraint::Full;
use cursive::views::{Dialog, LinearLayout, Panel, ScrollView, SelectView};
use cursive::Cursive;

pub fn render_layout(s: &mut Cursive) {
    let details = Panel::new(details::update_details_view(s))
        .title("details")
        .with_name("details_panel");

    let mut splits_list = SelectView::<usize>::new()
        .on_select(splits::on_splits_change)
        .on_submit(splits::on_splits_select);

    let globals = s.user_data::<GlobalState>().unwrap();

    let segments = globals.splits_editor.state().segments;
    for (i, segment) in segments.iter().enumerate() {
        splits::add_split(s, &mut splits_list, segment, i)
    }

    let splits_list = splits_list.with_name("splits_list");
    let actions_list = LinearLayout::horizontal()
        .child(timing::build_timing_methods(s))
        .child(comparisons::build_comparison_button(s));

    let layout = LinearLayout::new(Orientation::Vertical)
        .child(details)
        .child(Panel::new(actions_list))
        .child(splits::build_splits_title(s))
        .child(ScrollView::new(splits_list));

    let main = Dialog::around(layout)
        .title("chronos splits editor")
        .button("save", save::save_data)
        .button("close", |s| s.quit())
        .resized(Full, Full);

    s.add_layer(main);
}
