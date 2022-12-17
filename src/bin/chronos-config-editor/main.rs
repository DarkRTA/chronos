use chronos::Config;
use chronos::ConfigValue;
use chronos::UniqueID;
use clap::Parser;
use cursive::view::Nameable;
use cursive::view::Scrollable;
use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::SelectView;
use cursive::Cursive;
use livesplit_core::hotkey::Hotkey;
use livesplit_core::hotkey::KeyCode;
use std::error::Error;
use std::str::FromStr;

#[derive(Parser)]
struct Args {
    config_file: String,
}

struct GlobalState {
    filename: String,
    cfg: Config,
}

// this whole thing is a mess because cursive has some of the most atrocious
// state management i have ever seen
//
// this will eventually be rewritten to use either dioxus or intuitive

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let cfg = match std::fs::read(&args.config_file) {
        Ok(v) => toml::de::from_slice(&v)?,
        Err(_) => Config::default(),
    };

    let mut siv = cursive::crossterm();
    siv.load_toml(chronos::CURSIVE_THEME).unwrap();
    let globals = GlobalState {
        cfg,
        filename: args.config_file,
    };

    siv.set_user_data(globals);
    let cfg_menu = config_menu(&mut siv);
    siv.add_layer(cfg_menu);
    siv.run();

    Ok(())
}

fn config_menu(s: &mut Cursive) -> Dialog {
    let mut setting_select = SelectView::new();
    rebuild_list(s, &mut setting_select);

    setting_select.set_on_submit(move |s, v| {
        edit_setting(s, *v);
    });

    let setting_select = setting_select.with_name("setting_list").scrollable();

    Dialog::around(setting_select)
        .title("chronos config editor")
        .button("Save", save_data)
        .button("Quit", |s| s.quit())
}

fn rebuild_list(s: &mut Cursive, sel: &mut SelectView<usize>) {
    let globals: &GlobalState = s.user_data().unwrap();
    sel.clear();
    for (i, setting) in globals.cfg.list().iter().enumerate() {
        let label = match setting.value {
            chronos::ConfigValue::Dummy => setting.name.to_string(),
            _ => format!("{}: {}", setting.name, setting.value),
        };

        sel.add_item(label, i);
    }
}

fn edit_setting(s: &mut Cursive, idx: usize) {
    let globals: &mut GlobalState = s.user_data().unwrap();
    let fields = globals.cfg.list();
    let field = &fields[idx];

    match field.value {
        ConfigValue::LocalHotkey(x) => local_hotkey_editor(s, idx, x),
        ConfigValue::GlobalHotkey(_) => global_hotkey_editor(s, idx),
        _ => (),
    }
}

fn global_hotkey_editor(s: &mut Cursive, idx: usize) {
    let editor_id = UniqueID::new();

    let on_submit = move |s: &mut Cursive| {
        let editor = s.find_name::<EditView>(&editor_id.to_string()).unwrap();
        {
            let globals: &mut GlobalState = s.user_data().unwrap();
            if let Ok(keycode) = KeyCode::from_str(&editor.get_content()) {
                let hotkey = Hotkey::from(keycode);
                globals.cfg.set(idx, Some(hotkey).into())
            }
        }
        s.pop_layer();
        let mut list =
            s.find_name::<SelectView<usize>>("setting_list").unwrap();

        let id = list.selected_id().unwrap();
        rebuild_list(s, &mut list);
        list.set_selection(id);
    };

    let editor = EditView::new()
        .on_submit(move |s, _v| on_submit(s))
        .with_name(editor_id.to_string());

    let dialog = Dialog::new()
        .title("Edit Hotkey")
        .content(editor)
        .button("Ok", on_submit)
        .button("Cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog)
}

fn local_hotkey_editor(s: &mut Cursive, idx: usize, initial: char) {
    let editor_id = UniqueID::new();

    let on_submit = move |s: &mut Cursive| {
        let editor = s.find_name::<EditView>(&editor_id.to_string()).unwrap();
        let globals: &mut GlobalState = s.user_data().unwrap();
        globals
            .cfg
            .set(idx, editor.get_content().chars().next().unwrap().into());
        s.pop_layer();
        let mut list =
            s.find_name::<SelectView<usize>>("setting_list").unwrap();

        let id = list.selected_id().unwrap();
        rebuild_list(s, &mut list);
        list.set_selection(id);
    };

    let editor = EditView::new()
        .content(initial.to_string())
        .on_submit(move |s, _v| on_submit(s))
        .with_name(editor_id.to_string());

    let dialog = Dialog::new()
        .title("Edit Hotkey")
        .content(editor)
        .button("Ok", on_submit)
        .button("Cancel", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog)
}

fn save_data(s: &mut Cursive) {
    let globals: &GlobalState = s.user_data().unwrap();
    let res = std::fs::write(
        &globals.filename,
        toml::ser::to_string(&globals.cfg).unwrap(),
    );

    match res {
        Ok(_) => {
            s.add_layer(Dialog::text("Save Successful").button("Close", |s| {
                s.pop_layer();
            }));
        }
        Err(x) => {
            s.add_layer(Dialog::text(format!("Save Failed: {}", x)).button(
                "Close",
                |s| {
                    s.pop_layer();
                },
            ));
        }
    }
}
