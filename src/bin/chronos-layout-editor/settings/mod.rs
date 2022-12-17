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
use cursive::views::SelectView;
use cursive::views::TextView;
use cursive::Cursive;
use livesplit_core::component::splits::ColumnStartWith;
use livesplit_core::component::splits::ColumnUpdateTrigger;
use livesplit_core::component::splits::ColumnUpdateWith;
use livesplit_core::layout::LayoutDirection;
use livesplit_core::settings::Alignment;
use livesplit_core::settings::ColumnKind;
use livesplit_core::settings::Value;
use livesplit_core::timing::formatter::Accuracy;
use livesplit_core::timing::formatter::DigitsFormat;
use livesplit_core::LayoutEditor;
use livesplit_core::TimingMethod;

mod color;
use color::edit_color;
use color::edit_delta_gradient;
use color::edit_gradient;
use color::edit_list_gradient;
use color::edit_optional_color;

mod font;
use font::edit_font;

pub fn setting_editor(s: &mut Cursive, component: bool, idx: usize) {
    let global = s.user_data::<GlobalState>().unwrap();
    let state = global.layout_editor.state();

    let field = if component {
        &state.component_settings.fields[idx]
    } else {
        &state.general_settings.fields[idx]
    };

    match &field.value {
        Value::Bool(x) => edit_bool(s, component, idx, &field.text, *x),
        Value::UInt(x) => edit_uint(s, component, idx, &field.text, *x),
        Value::Int(x) => edit_int(s, component, idx, &field.text, *x),
        Value::String(x) => edit_string(s, component, idx, &field.text, x),
        Value::OptionalString(x) => {
            edit_optional_string(s, component, idx, &field.text, x)
        }
        Value::Accuracy(x) => edit_accuracy(s, component, idx, &field.text, *x),
        Value::DigitsFormat(x) => {
            edit_digits_format(s, component, idx, &field.text, *x)
        }
        Value::Alignment(x) => {
            edit_alignment(s, component, idx, &field.text, *x)
        }
        Value::ColumnKind(x) => {
            edit_column_kind(s, component, idx, &field.text, *x)
        }
        Value::ColumnStartWith(x) => {
            edit_column_start_with(s, component, idx, &field.text, *x)
        }
        Value::ColumnUpdateWith(x) => {
            edit_column_update_with(s, component, idx, &field.text, *x)
        }
        Value::ColumnUpdateTrigger(x) => {
            edit_column_update_trigger(s, component, idx, &field.text, *x)
        }
        Value::LayoutDirection(x) => {
            edit_layout_direction(s, component, idx, &field.text, *x)
        }
        Value::Color(x) => edit_color(s, component, idx, &field.text, *x),
        Value::OptionalColor(x) => {
            edit_optional_color(s, component, idx, &field.text, *x)
        }
        Value::Gradient(x) => edit_gradient(s, component, idx, &field.text, *x),
        Value::ListGradient(x) => {
            edit_list_gradient(s, component, idx, &field.text, *x)
        }
        Value::DeltaGradient(x) => {
            edit_delta_gradient(s, component, idx, &field.text, *x)
        }
        Value::OptionalTimingMethod(x) => {
            edit_optional_timing_method(s, component, idx, &field.text, *x)
        }
        Value::Font(x) => edit_font(s, component, idx, &field.text, x),
        Value::Hotkey(_) => unreachable!(), // no
    }
}

fn show_error(s: &mut Cursive, err: &str) {
    let dialog = Dialog::text(err).title("Error").button("Ok", |s| {
        s.pop_layer();
    });

    s.add_layer(dialog)
}

fn set_setting(
    editor: &mut LayoutEditor,
    component: bool,
    idx: usize,
    value: Value,
) {
    if component {
        editor.set_component_settings_value(idx, value)
    } else {
        editor.set_general_settings_value(idx, value)
    }
}

fn enum_helper<T>(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: T,
    list: &[(&'static str, T)],
) where
    Value: From<T>,
    T: std::cmp::PartialEq + Copy + 'static,
{
    let tmp = list.iter().find(|(_, x)| *x == value).unwrap();
    let label = TextView::new(format!("currently selected: {}", tmp.0));

    let callback = move |s: &mut Cursive, value: &T| {
        let value = Value::from(*value);
        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let mut selection_list = SelectView::new().on_submit(callback);

    for (string, value) in list.iter() {
        selection_list.add_item(*string, *value);
    }

    let layout = LinearLayout::new(Orientation::Vertical)
        .child(label)
        .child(Panel::new(selection_list));

    let dialog = Dialog::around(layout).title(name).button("cancel", |s| {
        s.pop_layer();
    });

    s.add_layer(dialog);
}

fn edit_string(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: &str,
) {
    let field_id = UniqueID::new();

    let apply = move |s: &mut Cursive| {
        let text_field =
            s.find_name::<EditView>(&field_id.to_string()).unwrap();
        let value = Value::from((*text_field.get_content()).clone());
        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let text_field = EditView::new()
        .on_submit(move |s, _| apply(s))
        .content(value)
        .with_name(field_id.to_string())
        .min_width(30);

    let dialog = Dialog::around(text_field)
        .title(name)
        .button("ok", move |s| apply(s))
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

fn edit_bool(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    enabled: bool,
) {
    let field_id = UniqueID::new();

    let apply = move |s: &mut Cursive| {
        let checkbox = s.find_name::<Checkbox>(&field_id.to_string()).unwrap();
        let value = Value::from(checkbox.is_checked());
        let global = s.user_data::<GlobalState>().unwrap();
        set_setting(&mut global.layout_editor, component, idx, value);
        s.pop_layer();
        refresh_layout(s);
    };

    let checkbox = Checkbox::new()
        .with_checked(enabled)
        .with_name(field_id.to_string())
        .min_width(4);

    let layout = LinearLayout::new(Orientation::Horizontal)
        .child(checkbox)
        .child(TextView::new("enabled"));

    let dialog = Dialog::around(layout)
        .title(name)
        .button("ok", move |s| apply(s))
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

fn edit_int(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: i64,
) {
    let field_id = UniqueID::new();

    let apply = move |s: &mut Cursive| {
        let field = s.find_name::<EditView>(&field_id.to_string()).unwrap();
        let Ok(parsed) = (*field.get_content()).parse::<i64>() else {
            show_error(s, "Could not parse number");
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
        .content(value.to_string())
        .with_name(field_id.to_string())
        .min_width(24);

    let layout = LinearLayout::new(Orientation::Horizontal)
        .child(TextView::new("i64: "))
        .child(field);

    let dialog = Dialog::around(layout)
        .title(name)
        .button("ok", move |s| apply(s))
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

fn edit_uint(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: u64,
) {
    let field_id = UniqueID::new();

    let apply = move |s: &mut Cursive| {
        let field = s.find_name::<EditView>(&field_id.to_string()).unwrap();
        let Ok(parsed) = (*field.get_content()).parse::<u64>() else {
            show_error(s, "Could not parse number");
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
        .content(value.to_string())
        .with_name(field_id.to_string())
        .min_width(24);

    let layout = LinearLayout::new(Orientation::Horizontal)
        .child(TextView::new("u64: "))
        .child(field);

    let dialog = Dialog::around(layout)
        .title(name)
        .button("ok", move |s| apply(s))
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

pub fn edit_optional_string(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: &Option<String>,
) {
    let field_id = UniqueID::new();
    let check_id = UniqueID::new();

    let tmp = String::new();
    let (content, checked) = match value {
        Some(c) => (c, true),
        None => (&tmp, false),
    };

    let apply = move |s: &mut Cursive| {
        let field = s.find_name::<EditView>(&field_id.to_string()).unwrap();
        let checkbox = s.find_name::<Checkbox>(&check_id.to_string()).unwrap();

        let value = Value::from(if checkbox.is_checked() {
            Some((*field.get_content()).clone())
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
        .content(content)
        .with_name(field_id.to_string())
        .min_width(32);

    let layout3 = LinearLayout::new(Orientation::Vertical)
        .child(layout1)
        .child(field);

    let dialog = Dialog::around(layout3)
        .title(name)
        .button("ok", apply)
        .button("cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}

fn edit_accuracy(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: Accuracy,
) {
    let list = vec![
        ("Seconds", Accuracy::Seconds),
        ("Tenths", Accuracy::Tenths),
        ("Hundredths", Accuracy::Hundredths),
        ("Milliseconds", Accuracy::Milliseconds),
    ];

    enum_helper(s, component, idx, name, value, &list)
}

fn edit_digits_format(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: DigitsFormat,
) {
    let list = [
        ("1", DigitsFormat::SingleDigitSeconds),
        ("01", DigitsFormat::DoubleDigitSeconds),
        ("0:01", DigitsFormat::SingleDigitMinutes),
        ("00:01", DigitsFormat::DoubleDigitMinutes),
        ("0:00:01", DigitsFormat::SingleDigitHours),
        ("00:00:01", DigitsFormat::DoubleDigitHours),
    ];

    enum_helper(s, component, idx, name, value, &list)
}

fn edit_alignment(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: Alignment,
) {
    let list = [
        ("Auto", Alignment::Auto),
        ("Left", Alignment::Left),
        ("Center", Alignment::Center),
    ];

    enum_helper(s, component, idx, name, value, &list)
}

fn edit_column_kind(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: ColumnKind,
) {
    let list = [
        ("Time", ColumnKind::Time),
        ("Variable", ColumnKind::Variable),
    ];

    enum_helper(s, component, idx, name, value, &list)
}

fn edit_column_start_with(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: ColumnStartWith,
) {
    let list = [
        ("Empty", ColumnStartWith::Empty),
        ("Time", ColumnStartWith::ComparisonTime),
        ("Segment Time", ColumnStartWith::ComparisonSegmentTime),
        ("Possible Time Save", ColumnStartWith::PossibleTimeSave),
    ];

    enum_helper(s, component, idx, name, value, &list)
}

fn edit_column_update_with(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: ColumnUpdateWith,
) {
    let list = [
        ("Don't Update", ColumnUpdateWith::DontUpdate),
        ("Split Time", ColumnUpdateWith::SplitTime),
        ("Delta", ColumnUpdateWith::Delta),
        ("Delta with Fallback", ColumnUpdateWith::DeltaWithFallback),
        ("Segment Time", ColumnUpdateWith::SegmentTime),
        ("Segment Delta", ColumnUpdateWith::SegmentDelta),
        (
            "Segment Delta with Fallback",
            ColumnUpdateWith::SegmentDeltaWithFallback,
        ),
    ];

    enum_helper(s, component, idx, name, value, &list)
}

fn edit_column_update_trigger(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: ColumnUpdateTrigger,
) {
    let list = [
        (
            "On Starting Segment",
            ColumnUpdateTrigger::OnStartingSegment,
        ),
        ("Contextual", ColumnUpdateTrigger::Contextual),
        ("On Ending Segment", ColumnUpdateTrigger::OnEndingSegment),
    ];

    enum_helper(s, component, idx, name, value, &list)
}

fn edit_layout_direction(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: LayoutDirection,
) {
    let list = [
        ("Vertical", LayoutDirection::Vertical),
        ("Horizontal", LayoutDirection::Horizontal),
    ];

    enum_helper(s, component, idx, name, value, &list)
}

fn edit_optional_timing_method(
    s: &mut Cursive,
    component: bool,
    idx: usize,
    name: &str,
    value: Option<TimingMethod>,
) {
    let list = [
        ("Unspecified", None),
        ("Real Time", Some(TimingMethod::RealTime)),
        ("Game Time", Some(TimingMethod::GameTime)),
    ];

    enum_helper(s, component, idx, name, value, &list)
}
