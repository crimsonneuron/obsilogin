use cursive::views::{Dialog, TextView};
use cursive::{Cursive,CbSink};
use std::path::Path;
use std::process::Command;
use std::process;
use std::env;
fn main() {
    let mut siv = cursive::default();
    let cb_sink = siv.cb_sink().clone();
    let args: Vec<String> = env::args().collect();
    
    match usb_status(&args) {
        USBstatus::NotMounted => usb_dne_screen(&mut siv, cb_sink),
        USBstatus::NotDecrypted => usb_not_decrypted_screen(&mut siv, cb_sink),
        USBstatus::Decrypted => usb_decrypted_screen(&mut siv, cb_sink),
    }

    siv.run();
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
fn usb_not_decrypted_screen(cursive_root: &mut Cursive, callback_sink: CbSink) {
    cursive_root.pop_layer();
    cursive_root.add_layer(
        Dialog::around(
            TextView::new("USB mounted, but not decrypted \n Decrypt?"))
            .button("Quit", |s| s.quit())
            .button("Decrypt", |s| todo!())
            .title("ObsiLogin: USB not decrypted"));
}
fn usb_dne_screen(cursive_root: &mut Cursive, callback_sink: CbSink) {
    let cb_sink_clone = callback_sink.clone();
    cursive_root.pop_layer();
    cursive_root.add_layer(
        Dialog::around(
            TextView::new("USB Path Not Found. \n Open obsidian blank?"))
            .title("ObsiLogin: USB Not Found")
            .button("Open blank", move |_| open_obsidian(cb_sink_clone.clone()))
            .button("Quit", |s| s.quit()));
}

fn usb_decrypted_screen(cursive_root: &mut Cursive, callback_sink: CbSink) {
    cursive_root.pop_layer();
    cursive_root.add_layer(
        Dialog::info("Vault already decrypted, launching obsidian"
        ).title("ObsiLogin: Launching")
    );
    open_obsidian(callback_sink);
}
 
fn open_obsidian(callback_sink: CbSink) {
    Command::new("obsidian").spawn().expect("failed to launch obsidian");
    callback_sink.send(Box::new(|s: &mut Cursive| {
        s.quit()
    })).unwrap();
    process::exit(0);
}
