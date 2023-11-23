use super::set_setting;
use super::show_error;
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
use livesplit_core::component::timer::DeltaGradient;
use livesplit_core::settings::Color;
use livesplit_core::settings::Gradient;
use livesplit_core::settings::ListGradient;
use livesplit_core::settings::Value;
use std::error::Error;

#[allow(clippy::identity_op)] // readability
fn parse_color(hex_color: &str) -> Result<Color, Box<dyn Error>> {
    let stripped = hex_color.trim_start_matches('#');
    match stripped.chars().count() {
        6 => {
            let parsed = u32::from_str_radix(stripped, 16)?;
            let r = ((parsed & 0x00ff0000) >> 16) as u8;
            let g = ((parsed & 0x0000ff00) >> 8) as u8;
            let b = ((parsed & 0x000000ff) >> 0) as u8;
            Ok(Color::rgba8(r, g, b, 255))
        }
        8 => {
            let parsed = u32::from_str_radix(stripped, 16)?;
            let r = ((parsed & 0xff000000) >> 24) as u8;
            let g = ((parsed & 0x00ff0000) >> 16) as u8;
            let b = ((parsed & 0x0000ff00) >> 8) as u8;
            let a = ((parsed & 0x000000ff) >> 0) as u8;
            Ok(Color::rgba8(r, g, b, a))
        }
        _ => Err(Box::from("invalid length")),
    }
}

fn color_to_string(color: Color) -> String {
    let rgba = color.to_rgba8();
    format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        rgba[0], rgba[1], rgba[2], rgba[3]
    )
}

pub fn edit_color(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: Color,
) {
    let field_id = UniqueID::new();

    let apply = move |s: &mut Cursive| {
        let field = s.find_name::<EditView>(&field_id.to_string()).unwrap();
        let Ok(parsed) = parse_color(&field.get_content()) else {
            show_error(s, "Could not parse color");
            return;
        };
        let value = Value::from(parsed);
        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let field = EditView::new()
        .on_submit(move |s, _| apply(s))
        .content(color_to_string(value))
        .with_name(field_id.to_string())
        .min_width(16);

    let layout = LinearLayout::new(Orientation::Horizontal)
        .child(TextView::new("color: "))
        .child(field);

    let dialog = Dialog::around(layout)
        .title(name)
        .button("ok", move |s| apply(s))
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

pub fn edit_optional_color(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: Option<Color>,
) {
    let field_id = UniqueID::new();
    let check_id = UniqueID::new();
    let (color, checked) = match value {
        Some(c) => (color_to_string(c), true),
        None => ("#00000000".into(), false),
    };

    let apply = move |s: &mut Cursive| {
        let field = s.find_name::<EditView>(&field_id.to_string()).unwrap();
        let checkbox = s.find_name::<Checkbox>(&check_id.to_string()).unwrap();

        let value = Value::from(if checkbox.is_checked() {
            let Ok(color) = parse_color(&field.get_content()) else {
                show_error(s, "Error parsing color.");
                return;
            };
            Some(color)
        } else {
            None
        });

        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let checkbox = Checkbox::new()
        .with_checked(checked)
        .with_name(check_id.to_string())
        .min_width(4);

    let layout1 = LinearLayout::new(Orientation::Horizontal)
        .child(checkbox)
        .child(TextView::new("enabled"));

    let field = EditView::new()
        .content(color)
        .with_name(field_id.to_string())
        .min_width(16);

    let layout2 = LinearLayout::new(Orientation::Horizontal)
        .child(TextView::new("color: "))
        .child(field);

    let layout3 = LinearLayout::new(Orientation::Vertical)
        .child(layout1)
        .child(layout2);

    let dialog = Dialog::around(layout3)
        .title(name)
        .button("ok", apply)
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

// todo: disable unselectable color fields
pub fn edit_gradient(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: Gradient,
) {
    let color1_id = UniqueID::new();
    let color2_id = UniqueID::new();

    let (color1, color2, variant) = match value {
        Gradient::Transparent => ("#00000000".into(), "#00000000".into(), 0),
        Gradient::Plain(x) => (color_to_string(x), color_to_string(x), 1),
        Gradient::Vertical(x, y) => (color_to_string(x), color_to_string(y), 2),
        Gradient::Horizontal(x, y) => {
            (color_to_string(x), color_to_string(y), 3)
        }
    };

    let color1_input = EditView::new()
        .content(color1)
        .with_name(color1_id.to_string())
        .min_width(12);

    let color2_input = EditView::new()
        .content(color2)
        .with_name(color2_id.to_string())
        .min_width(12);

    let mut variant_group = RadioGroup::new();
    let mut variant_buttons = [
        variant_group.button(0, "Transparent"),
        variant_group.button(1, "Plain"),
        variant_group.button(2, "Vertical"),
        variant_group.button(3, "Horizontal"),
    ];
    variant_buttons[variant].select();

    let apply = move |s: &mut Cursive| {
        let color1_input =
            s.find_name::<EditView>(&color1_id.to_string()).unwrap();
        let color2_input =
            s.find_name::<EditView>(&color2_id.to_string()).unwrap();

        let Ok(color_1) = parse_color(&color1_input.get_content()) else {
            show_error(s, "Error parsing color");
            return;
        };
        let Ok(color_2) = parse_color(&color2_input.get_content()) else {
            show_error(s, "Error parsing color");
            return;
        };

        let value = Value::from(match *variant_group.selection() {
            0 => Gradient::Transparent,
            1 => Gradient::Plain(color_1),
            2 => Gradient::Vertical(color_1, color_2),
            3 => Gradient::Horizontal(color_1, color_2),
            _ => unreachable!(),
        });

        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let mut variant_list = LinearLayout::new(Orientation::Vertical);
    for i in variant_buttons {
        variant_list.add_child(i);
    }
    let variant_list = Panel::new(variant_list).title("Variants");

    let layout1 = Panel::new(
        LinearLayout::new(Orientation::Vertical)
            .child(TextView::new("color 1:"))
            .child(color1_input)
            .child(TextView::new("color 2:"))
            .child(color2_input),
    )
    .title("Colors");

    let layout2 = LinearLayout::new(Orientation::Horizontal)
        .child(variant_list)
        .child(layout1);

    let dialog = Dialog::around(layout2)
        .title(name)
        .button("ok", apply)
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

pub fn edit_list_gradient(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: ListGradient,
) {
    let color1_id = UniqueID::new();
    let color2_id = UniqueID::new();

    let (color1, color2, variant) = match value {
        ListGradient::Same(Gradient::Transparent) => {
            ("#00000000".into(), "#00000000".into(), 0)
        }
        ListGradient::Same(Gradient::Plain(x)) => {
            (color_to_string(x), color_to_string(x), 1)
        }
        ListGradient::Same(Gradient::Vertical(x, y)) => {
            (color_to_string(x), color_to_string(y), 2)
        }
        ListGradient::Same(Gradient::Horizontal(x, y)) => {
            (color_to_string(x), color_to_string(y), 3)
        }
        ListGradient::Alternating(x, y) => {
            (color_to_string(x), color_to_string(y), 4)
        }
    };

    let color1_input = EditView::new()
        .content(color1)
        .with_name(color1_id.to_string())
        .min_width(12);

    let color2_input = EditView::new()
        .content(color2)
        .with_name(color2_id.to_string())
        .min_width(12);

    let mut variant_group = RadioGroup::new();
    let mut variant_buttons = [
        variant_group.button(0, "Transparent"),
        variant_group.button(1, "Plain"),
        variant_group.button(2, "Vertical"),
        variant_group.button(3, "Horizontal"),
        variant_group.button(4, "Alternating"),
    ];
    variant_buttons[variant].select();

    let apply = move |s: &mut Cursive| {
        let color1_input =
            s.find_name::<EditView>(&color1_id.to_string()).unwrap();
        let color2_input =
            s.find_name::<EditView>(&color2_id.to_string()).unwrap();

        let Ok(color_1) = parse_color(&color1_input.get_content()) else {
            show_error(s, "Error parsing color");
            return;
        };
        let Ok(color_2) = parse_color(&color2_input.get_content()) else {
            show_error(s, "Error parsing color");
            return;
        };

        let value = Value::from(match *variant_group.selection() {
            0 => ListGradient::Same(Gradient::Transparent),
            1 => ListGradient::Same(Gradient::Plain(color_1)),
            2 => ListGradient::Same(Gradient::Vertical(color_1, color_2)),
            3 => ListGradient::Same(Gradient::Horizontal(color_1, color_2)),
            4 => ListGradient::Alternating(color_1, color_2),
            _ => unreachable!(),
        });

        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let mut variant_list = LinearLayout::new(Orientation::Vertical);
    for i in variant_buttons {
        variant_list.add_child(i);
    }
    let variant_list = Panel::new(variant_list).title("Variants");

    let layout1 = Panel::new(
        LinearLayout::new(Orientation::Vertical)
            .child(TextView::new("color 1:"))
            .child(color1_input)
            .child(TextView::new("color 2:"))
            .child(color2_input),
    )
    .title("Colors");

    let layout2 = LinearLayout::new(Orientation::Horizontal)
        .child(variant_list)
        .child(layout1);

    let dialog = Dialog::around(layout2)
        .title(name)
        .button("ok", apply)
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

pub fn edit_delta_gradient(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: DeltaGradient,
) {
    let color1_id = UniqueID::new();
    let color2_id = UniqueID::new();

    let (color1, color2, variant) = match value {
        DeltaGradient::Gradient(Gradient::Transparent) => {
            ("#00000000".into(), "#00000000".into(), 0)
        }
        DeltaGradient::Gradient(Gradient::Plain(x)) => {
            (color_to_string(x), color_to_string(x), 1)
        }
        DeltaGradient::Gradient(Gradient::Vertical(x, y)) => {
            (color_to_string(x), color_to_string(y), 2)
        }
        DeltaGradient::Gradient(Gradient::Horizontal(x, y)) => {
            (color_to_string(x), color_to_string(y), 3)
        }
        DeltaGradient::DeltaPlain => {
            ("#00000000".into(), "#00000000".into(), 4)
        }
        DeltaGradient::DeltaVertical => {
            ("#00000000".into(), "#00000000".into(), 5)
        }
        DeltaGradient::DeltaHorizontal => {
            ("#00000000".into(), "#00000000".into(), 6)
        }
    };

    let color1_input = EditView::new()
        .content(color1)
        .with_name(color1_id.to_string())
        .min_width(12);

    let color2_input = EditView::new()
        .content(color2)
        .with_name(color2_id.to_string())
        .min_width(12);

    let mut variant_group = RadioGroup::new();
    let mut variant_buttons = [
        variant_group.button(0, "Transparent"),
        variant_group.button(1, "Plain"),
        variant_group.button(2, "Vertical"),
        variant_group.button(3, "Horizontal"),
        variant_group.button(4, "Delta Plain"),
        variant_group.button(5, "Delta Vertical"),
        variant_group.button(6, "Delta Horizontal"),
    ];
    variant_buttons[variant].select();

    let apply = move |s: &mut Cursive| {
        let color1_input =
            s.find_name::<EditView>(&color1_id.to_string()).unwrap();
        let color2_input =
            s.find_name::<EditView>(&color2_id.to_string()).unwrap();

        let Ok(color_1) = parse_color(&color1_input.get_content()) else {
            show_error(s, "Error parsing color");
            return;
        };
        let Ok(color_2) = parse_color(&color2_input.get_content()) else {
            show_error(s, "Error parsing color");
            return;
        };

        let value = Value::from(match *variant_group.selection() {
            0 => DeltaGradient::Gradient(Gradient::Transparent),
            1 => DeltaGradient::Gradient(Gradient::Plain(color_1)),
            2 => DeltaGradient::Gradient(Gradient::Vertical(color_1, color_2)),
            3 => {
                DeltaGradient::Gradient(Gradient::Horizontal(color_1, color_2))
            }
            4 => DeltaGradient::DeltaPlain,
            5 => DeltaGradient::DeltaVertical,
            6 => DeltaGradient::DeltaHorizontal,
            _ => unreachable!(),
        });

        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let mut variant_list = LinearLayout::new(Orientation::Vertical);
    for i in variant_buttons {
        variant_list.add_child(i);
    }
    let variant_list = Panel::new(variant_list).title("Variants");

    let layout1 = Panel::new(
        LinearLayout::new(Orientation::Vertical)
            .child(TextView::new("color 1:"))
            .child(color1_input)
            .child(TextView::new("color 2:"))
            .child(color2_input),
    )
    .title("Colors");

    let layout2 = LinearLayout::new(Orientation::Horizontal)
        .child(variant_list)
        .child(layout1);

    let dialog = Dialog::around(layout2)
        .title(name)
        .button("ok", apply)
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}
