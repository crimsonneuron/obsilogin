use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use std::path::Path;
use std::process::Command;
use std::env;
fn main() {
    let mut siv = cursive::default();
    if !vault_file_path_exists() {
        path_dne_screen(&mut siv);
    }
    siv.run()
}

fn vault_file_path_exists()-> bool {
    let args:Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]);
    file_path.exists()
}

fn path_dne_screen(cursive_root: &mut Cursive) {
    cursive_root.pop_layer();
    cursive_root.add_layer(
        Dialog::around(
            TextView::new("Vault Path Not Found. \n Open obsidian blank?"))
            .title("ObsiLogin: Path Not Found")
            .button("Open blank", |_| open_obsidian()));
}

fn open_obsidian() {
    Command::new("obsidian");
}
