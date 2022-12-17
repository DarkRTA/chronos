use clap::Parser;
use cursive::direction::Orientation;
use cursive::traits::*;
use cursive::view::Scrollable;
use cursive::view::SizeConstraint::Fixed;
use cursive::view::SizeConstraint::Full;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::SelectView;
use cursive::Cursive;
use livesplit_core::component::*;
use livesplit_core::layout::Component;
use livesplit_core::layout::Editor as LayoutEditor;
use livesplit_core::layout::Layout;
use livesplit_core::layout::LayoutSettings;
use std::error::Error;
use std::io::Cursor;
use std::rc::Rc;

mod settings;

#[derive(Parser)]
struct Args {
    layout_file: String,
}

pub struct GlobalState {
    layout_editor: LayoutEditor,
    layout_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let layout = match std::fs::read(&args.layout_file) {
        Ok(file) => {
            let cursor = Cursor::new(file);
            Layout::from_settings(LayoutSettings::from_json(cursor)?)
        }
        Err(_) => {
            let cursor = Cursor::new(chronos::DEFAULT_LAYOUT);
            Layout::from_settings(LayoutSettings::from_json(cursor)?)
        }
    };

    let editor = LayoutEditor::new(layout)?;

    let globals = GlobalState {
        layout_editor: editor,
        layout_file: args.layout_file,
    };

    let backend = {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered = cursive_buffered_backend::BufferedBackend::new(backend);
        Box::new(buffered)
    };

    let mut siv = cursive::CursiveRunner::new(Cursive::new(), backend);
    siv.load_toml(chronos::CURSIVE_THEME).unwrap();
    siv.set_user_data(globals);
    main_layout(&mut siv);
    refresh_layout(&mut siv);
    siv.run();
    Ok(())
}

fn main_layout(s: &mut Cursive) {
    let component_list = Panel::new(
        SelectView::<usize>::new()
            .on_select(on_component_change)
            .on_submit(on_component_select)
            .with_name("components")
            .scrollable(),
    )
    .title("components")
    .fixed_width(32);

    let layout_settings_list = Panel::new(
        SelectView::<usize>::new()
            .on_submit(on_layout_setting_select)
            .with_name("layout_settings")
            .scrollable(),
    )
    .title("layout settings")
    .resized(Full, Fixed(10));

    let component_settings_list = Panel::new(
        SelectView::<usize>::new()
            .on_submit(on_component_setting_select)
            .with_name("component_settings")
            .scrollable(),
    )
    .title("component settings")
    .resized(Full, Fixed(10));

    let right_side = LinearLayout::new(Orientation::Vertical)
        .child(layout_settings_list)
        .child(component_settings_list);

    let select_layout = LinearLayout::new(Orientation::Horizontal)
        .child(component_list)
        .child(right_side);

    let main = Dialog::around(select_layout)
        .title("chronos layout editor")
        .button("save", save_data)
        .button("cancel", |s| s.quit())
        .resized(Fixed(80), Fixed(24));

    s.add_layer(main);
}

fn save_data(s: &mut Cursive) {
    let globals: GlobalState = s.take_user_data().unwrap();
    let res: Result<(), Box<dyn Error>> = (move || {
        let layout = globals.layout_editor.close();
        let settings = layout.settings();
        let file = std::fs::File::create(&globals.layout_file)?;
        settings.write_json(file)?;
        Ok(())
    })();

    match res {
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

fn on_component_change(s: &mut Cursive, v: &usize) {
    let globals = s.user_data::<GlobalState>().unwrap();
    globals.layout_editor.select(*v);
    refresh_layout(s)
}

fn on_component_setting_select(s: &mut Cursive, v: &usize) {
    settings::setting_editor(s, true, *v);
}

fn on_layout_setting_select(s: &mut Cursive, v: &usize) {
    settings::setting_editor(s, false, *v);
}

fn on_component_select(s: &mut Cursive, _v: &usize) {
    let globals = s.user_data::<GlobalState>().unwrap();
    let state = globals.layout_editor.state();

    let mut menu = SelectView::new();
    menu.add_item("Add Component", 0);
    if state.buttons.can_remove {
        menu.add_item("Remove Component", 1);
    }
    menu.add_item("Clone Component", 2);
    if state.buttons.can_move_up {
        menu.add_item("Move Up", 3);
    }
    if state.buttons.can_move_down {
        menu.add_item("Move Down", 4);
    }

    menu.set_on_submit(|s, v| {
        let globals = s.user_data::<GlobalState>().unwrap();
        match v {
            0 => {
                // bail out early
                s.pop_layer();
                add_component_menu(s);
                return;
            }
            1 => globals.layout_editor.remove_component(),
            2 => globals.layout_editor.duplicate_component(),
            3 => globals.layout_editor.move_component_up(),
            4 => globals.layout_editor.move_component_down(),
            _ => unreachable!(),
        }
        refresh_layout(s);
        s.pop_layer();
    });

    let dialog =
        Dialog::around(menu)
            .title("component actions")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog)
}

#[rustfmt::skip]
fn add_component_menu(s: &mut Cursive) {
    let menu = SelectView::new()
        .item_str("Current Comparison")
        .item_str("Current Pace")
        .item_str("Delta")
        .item_str("Detailed Timer")
        .item_str("Graph")
        .item_str("PB Chance")
        .item_str("Possible Time Save")
        .item_str("Previous Segment")
        .item_str("Segment Time")
        .item_str("Splits")
        .item_str("Sum of Best Segments")
        .item_str("Text")
        .item_str("Timer")
        .item_str("Title")
        .item_str("Total Playtime")
        .item_str("Blank Space")
        .item_str("Separator")
        .on_submit(|s, v: &str| {
            let globals = s.user_data::<GlobalState>().unwrap();
            let component = match v {
                "Current Comparison" => {
                    Component::from(CurrentComparison::new())
                }
                "Current Pace" => {
                    Component::from(CurrentPace::new())
                }
                "Delta" => {
                    Component::from(Delta::new())
                }
                "Detailed Timer" => {
                    Component::from(Box::new(DetailedTimer::new()))
                }
                "Graph" => {
                    Component::from(Graph::new())
                }
                "PB Chance" => {
                    Component::from(PbChance::new())
                }
                "Possible Time Save" => {
                    Component::from(PossibleTimeSave::new())
                }
                "Previous Segment" => {
                    Component::from(PreviousSegment::new())
                }
                "Segment Time" => {
                    Component::from(SegmentTime::new())
                }
                "Splits" => {
                    Component::from(Splits::new())
                }
                "Sum of Best Segments" => {
                    Component::from(SumOfBest::new())
                }
                "Text" => {
                    Component::from(Text::new())
                }
                "Timer" => {
                    Component::from(Timer::new())
                }
                "Title" => {
                    Component::from(Title::new())
                }
                "Total Playtime" => {
                    Component::from(TotalPlaytime::new())
                }
                "Blank Space" => {
                    Component::from(BlankSpace::new())
                }
                "Separator" => {
                    Component::from(Separator::new())
                }
                _ => unreachable!(),
            };
            globals.layout_editor.add_component(component);
            s.pop_layer();
            refresh_layout(s);
        });

    let dialog =
        Dialog::around(menu)
            .title("add component")
            .button("close", |s| {
                s.pop_layer();
            });

    s.add_layer(dialog);
}

// this is disgusting
fn refresh_layout(s: &mut Cursive) {
    let globals = s.user_data::<GlobalState>().unwrap();
    let state = globals.layout_editor.state();

    /* component list */
    let mut component_list =
        s.find_name::<SelectView<usize>>("components").unwrap();

    component_list.clear();
    for (i, component) in state.components.iter().enumerate() {
        component_list.add_item(component, i);
    }
    component_list.set_selection(state.selected_component as usize);

    /* component settings */
    let mut component_settings_list = s
        .find_name::<SelectView<usize>>("component_settings")
        .unwrap();
    let selection = component_settings_list
        .selection()
        .unwrap_or_else(|| Rc::new(0));
    component_settings_list.clear();
    for (i, setting) in state.component_settings.fields.iter().enumerate() {
        component_settings_list.add_item(&setting.text, i);
    }
    component_settings_list.set_selection(*selection);

    /* layout settings */
    let mut layout_settings_list =
        s.find_name::<SelectView<usize>>("layout_settings").unwrap();
    let selection = layout_settings_list
        .selection()
        .unwrap_or_else(|| Rc::new(0));
    layout_settings_list.clear();
    for (i, setting) in state.general_settings.fields.iter().enumerate() {
        layout_settings_list.add_item(&setting.text, i);
    }
    layout_settings_list.set_selection(*selection);
}
