extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use std::thread;
use std::time::Duration;

use std::fs;
use std::io::prelude::*;

fn main() {
    let contents = fs::read_to_string("./catfacts.txt").expect("Something went wrong :/");

    loop {
        for i in contents.split("\n") {

            thread::sleep(Duration::from_millis(50));  //If you dont have it windows throws a hissy fit
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(i.to_owned());
        }
    }
}
