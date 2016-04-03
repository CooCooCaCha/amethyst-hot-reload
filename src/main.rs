extern crate libloading;
extern crate notify;

use libloading::{Library, Symbol};
use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;
use std::process::Command;

fn load_game() {
    println!("File change detected - Building");
    let build_output = Command::new("cargo")
        .arg("build")
        .current_dir("./src/game")
        .output()
        .unwrap_or_else(|e| {
            panic!("DYNAMIC BUILD FAILED: {}", e);
        });

    if build_output.status.success() {
        println!("Build successful - Reloading library");
        let lib = Library::new("./src/game/target/debug/libgame.dylib").unwrap();
        let run_game: Symbol<extern fn()> = unsafe {
            lib.get(b"run").unwrap()
        };

        run_game();
    } else {
        println!("status: {}", build_output.status);
        println!("stdout: {}", String::from_utf8_lossy(&build_output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&build_output.stderr));
    }
}

fn main() {
    load_game();

    let (tx, rx) = channel();
    let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

    match w {
        Ok(mut watcher) => {
            watcher.watch("./src/game");

            loop {
                match rx.recv() {
                    _ => {
                        load_game();
                    }
                }
            }
        },
        Err(_) => println!("Error")
    }
}
