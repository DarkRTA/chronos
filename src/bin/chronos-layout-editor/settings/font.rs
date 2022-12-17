use super::set_setting;
use crate::refresh_layout;
use crate::GlobalState;
use chronos::UniqueID;
use cursive::direction::Orientation;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::Checkbox;
use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::RadioGroup;
use cursive::views::TextView;
use cursive::Cursive;
use livesplit_core::settings::Font;
use livesplit_core::settings::FontStretch;
use livesplit_core::settings::FontStyle;
use livesplit_core::settings::FontWeight;
use livesplit_core::settings::Value;

pub fn edit_font(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: &Option<Font>,
) {
    let tmp = "".to_string();
    let (enabled, family, style, weight, stretch) = match value {
        Some(x) => {
            let font = &x.family;

            let style = match x.style {
                FontStyle::Normal => 0,
                FontStyle::Italic => 1,
            };

            let weight = match x.weight {
                FontWeight::Thin => 0,
                FontWeight::ExtraLight => 1,
                FontWeight::Light => 2,
                FontWeight::SemiLight => 3,
                FontWeight::Normal => 4,
                FontWeight::Medium => 5,
                FontWeight::SemiBold => 6,
                FontWeight::Bold => 7,
                FontWeight::ExtraBold => 8,
                FontWeight::Black => 9,
                FontWeight::ExtraBlack => 10,
            };

            let stretch = match x.stretch {
                FontStretch::UltraCondensed => 0,
                FontStretch::ExtraCondensed => 1,
                FontStretch::Condensed => 2,
                FontStretch::SemiCondensed => 3,
                FontStretch::Normal => 4,
                FontStretch::SemiExpanded => 5,
                FontStretch::Expanded => 6,
                FontStretch::ExtraExpanded => 7,
                FontStretch::UltraExpanded => 8,
            };

            (true, font, style, weight, stretch)
        }
        None => (false, &tmp, 0, 5, 4),
    };

    let mut style_group = RadioGroup::new();
    let mut style_buttons = [
        style_group.button(FontStyle::Normal, "Normal"),
        style_group.button(FontStyle::Italic, "Italic"),
    ];
    style_buttons[style].select();

    let mut weight_group = RadioGroup::new();
    let mut weight_buttons = [
        weight_group.button(FontWeight::Thin, "Thin"),
        weight_group.button(FontWeight::ExtraLight, "Extra Light"),
        weight_group.button(FontWeight::Light, "Light"),
        weight_group.button(FontWeight::SemiLight, "Semi-Light"),
        weight_group.button(FontWeight::Normal, "Normal"),
        weight_group.button(FontWeight::Medium, "Medium"),
        weight_group.button(FontWeight::SemiBold, "Semi-Bold"),
        weight_group.button(FontWeight::Bold, "Bold"),
        weight_group.button(FontWeight::ExtraBold, "Extra Bold"),
        weight_group.button(FontWeight::Black, "Black"),
        weight_group.button(FontWeight::ExtraBlack, "Extra Black"),
    ];
    weight_buttons[weight].select();

    let mut stretch_group = RadioGroup::new();
    let mut stretch_buttons = [
        stretch_group.button(FontStretch::UltraCondensed, "Ultra Condensed"),
        stretch_group.button(FontStretch::ExtraCondensed, "Extra Condensed"),
        stretch_group.button(FontStretch::Condensed, "Condensed"),
        stretch_group.button(FontStretch::SemiCondensed, "Semi-Condensed"),
        stretch_group.button(FontStretch::Normal, "Normal"),
        stretch_group.button(FontStretch::SemiExpanded, "Semi-Expanded"),
        stretch_group.button(FontStretch::Expanded, "Expanded"),
        stretch_group.button(FontStretch::ExtraExpanded, "Extra Expanded"),
        stretch_group.button(FontStretch::UltraExpanded, "Ultra Expanded"),
    ];
    stretch_buttons[stretch].select();

    let checkbox_id = UniqueID::new();
    let family_id = UniqueID::new();

    let apply = move |s: &mut Cursive| {
        let checkbox =
            s.find_name::<Checkbox>(&checkbox_id.to_string()).unwrap();
        let family_field =
            s.find_name::<EditView>(&family_id.to_string()).unwrap();

        let value = Value::from(if checkbox.is_checked() {
            Some(Font {
                family: (*family_field.get_content()).clone(),
                stretch: *stretch_group.selection(),
                style: *style_group.selection(),
                weight: *weight_group.selection(),
            })
        } else {
            None
        });
        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let checkbox = Checkbox::new()
        .with_checked(enabled)
        .with_name(checkbox_id.to_string())
        .min_width(4);

    let checkbox_layout = LinearLayout::new(Orientation::Horizontal)
        .child(checkbox)
        .child(TextView::new("enabled"));

    let family_editor = EditView::new()
        .content(family)
        .with_name(family_id.to_string())
        .min_width(32);

    let family_layout = LinearLayout::new(Orientation::Horizontal)
        .child(TextView::new("family: "))
        .child(family_editor);

    let mut style_list = LinearLayout::new(Orientation::Vertical);
    for i in style_buttons {
        style_list.add_child(i);
    }
    let style_list = Panel::new(style_list).title("Style");

    let mut weight_list = LinearLayout::new(Orientation::Vertical);
    for i in weight_buttons {
        weight_list.add_child(i);
    }
    let weight_list = Panel::new(weight_list).title("Weight");

    let mut stretch_list = LinearLayout::new(Orientation::Vertical);
    for i in stretch_buttons {
        stretch_list.add_child(i);
    }
    let stretch_list = Panel::new(stretch_list).title("Stretch");

    let layout_top = Panel::new(
        LinearLayout::new(Orientation::Vertical)
            .child(checkbox_layout)
            .child(family_layout),
    );

    let layout_bottom = LinearLayout::new(Orientation::Horizontal)
        .child(style_list)
        .child(weight_list)
        .child(stretch_list);

    let layout_full = LinearLayout::new(Orientation::Vertical)
        .child(layout_top)
        .child(layout_bottom);

    let dialog = Dialog::around(layout_full)
        .title(name)
        .button("ok", apply)
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog)
}
