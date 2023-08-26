use crate::global_state::GlobalState;
use std::fs::File;
use cursive::Cursive;
use cursive::views::Dialog;
use livesplit_core::run::{
    saver::livesplit::{self, IoWrite},
};

pub fn save_data(s: &mut Cursive) {
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
