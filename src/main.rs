use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use std::path::Path;
use std::process::Command;
use std::env;
fn main() {
    let mut siv = cursive::default();
    let args: Vec<String> = env::args().collect();
    
    match usb_status(&args) {
        USBstatus::NotMounted => usb_dne_screen(&mut siv),
        USBstatus::NotDecrypted => todo!(),
        USBstatus::Decrypted => usb_decrypted_screen(&mut siv),
    }

    siv.run();
    siv.quit();
}

enum USBstatus {
    NotMounted,
    NotDecrypted,
    Decrypted,
}

fn usb_status(args: &Vec<String>) -> USBstatus {
    let usb_mounted_path = Path::new(&args[1]);
    let vault_path = Path::new(&args[2]);

    if !usb_mounted_path.exists() {
        return USBstatus::NotMounted;
    }
    else if !vault_path.exists() {
        return USBstatus::NotDecrypted;
    }
    else {
        return USBstatus::Decrypted;
    }
}
fn usb_dne_screen(cursive_root: &mut Cursive) {
    cursive_root.pop_layer();
    cursive_root.add_layer(
        Dialog::around(
            TextView::new("USB Path Not Found. \n Open obsidian blank?"))
            .title("ObsiLogin: USB Not Found")
            .button("Open blank", |_| open_obsidian())
            .button("Quit", |s| s.quit()));
}

fn usb_decrypted_screen(cursive_root: &mut Cursive) {
    cursive_root.pop_layer();
    cursive_root.add_layer(
        Dialog::info("Vault already decrypted, launching obsidian"
        ).title("ObsiLogin: Launching")
    );
    open_obsidian();
}
 
fn open_obsidian() {
    Command::new("obsidian").spawn().expect("failed to launch obsidian");

}
