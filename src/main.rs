use cursive::views::{Dialog, TextView};
use std::path::Path;
use std::env;
fn main() {
    let mut siv = cursive::default();

     


    siv.run()
}

fn vault_file_path_exists()-> boolean {
    let args:Vec<String> = env::args().collect();
    let file_path = Path::new(&args[2]);
    file_path.exists()
}

fn path_does_not_exist_screen(&mut cursive_root:CursiveRunnable) {
    cursive_root.add_layer(
        Dialog::around(
            TextView::new("Vault Path Not Found. \n Open obsidian blank?"))
            .title("ObsiLogin: Path Not Found")
            .button("Open blank", |s| ))
}
