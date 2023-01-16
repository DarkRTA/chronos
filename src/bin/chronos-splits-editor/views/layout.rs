use super::details;
use super::splits;
use crate::save;

use crate::global_state::GlobalState;

use cursive::Cursive;
use cursive::direction::Orientation;
use cursive::traits::{Resizable, Nameable};
use cursive::align::HAlign;
use cursive::theme::ColorStyle;
use cursive::reexports::enumset::EnumSet;
use cursive::view::SizeConstraint::{Full, Fixed};
use cursive::views::{Panel, SelectView, PaddedView, ListView, LinearLayout, TextView, ScrollView, Dialog}; 

pub fn render_layout(s: &mut Cursive) {
    let details = Panel::new(details::update_details_view(s))
        .title("details")
        .with_name("details_panel");

    let splits_title = PaddedView::lrtb(0, 0, 0, 1,
        LinearLayout::horizontal()
            .child(
                TextView::new("Split Name")
                    .style(cursive::theme::Style {
                        effects: EnumSet::only(cursive::theme::Effect::Bold),
                        color: ColorStyle::default(),
                    })
                    .h_align(HAlign::Left)
                    .resized(Fixed(36), Fixed(1)),
            )
            .child(PaddedView::lrtb(1, 0, 0, 0,
                TextView::new("Split Time")
                    .style(cursive::theme::Style {
                        effects: EnumSet::only(cursive::theme::Effect::Bold),
                        color: ColorStyle::default(),
                    })
                    .h_align(HAlign::Right)
                    .resized(Fixed(10), Fixed(1)),
            ))
            .child(PaddedView::lrtb(1, 0, 0, 0,
                TextView::new("Segment Time")
                    .style(cursive::theme::Style {
                        effects: EnumSet::only(cursive::theme::Effect::Bold),
                        color: ColorStyle::default(),
                    })
                    .h_align(HAlign::Right)
                    .resized(Fixed(12), Fixed(1)),
            ))
            .child(PaddedView::lrtb(1, 0, 0, 0,
                TextView::new("Best Segment")
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
        .on_select(splits::on_splits_change)
        .on_submit(splits::on_splits_select);

    let globals = s.user_data::<GlobalState>().unwrap();

    let segments = globals.splits_editor.state().segments;
    for (i, s) in segments.iter().enumerate() {
        splits::add_split(&mut splits_list, s, i)
    }

    let splits_list = splits_list.with_name("splits_list");

    let layout = LinearLayout::new(Orientation::Vertical)
        .child(details)
        .child(splits_title)
        .child(ScrollView::new(splits_list));

    let main = Dialog::around(layout)
        .title("chronos splits editor")
        .button("save", save::save_data)
        .button("close", |s| s.quit())
        .resized(Fixed(80), Full);

    s.add_layer(main);
}
