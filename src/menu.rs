// YT-DLP-REPL VERSION 1.0.0 / MIT LICENSE / COPYRIGHT © 2023 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

// IMPORTS

use std::env;
use std::process::exit;
use std::process::Command;

// DOCUMENTATION

pub fn documentation(reset: &str, blue_underlined: &str, grey: &str, red: &str, violet: &str, yellow: &str) {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // HELP

        if argument == "-h" || argument == "--help" {
            println!("{}", violet.to_owned() + "yt-dlp-repl" + reset + "'" + grey + "s source and documentation" + reset + ":");
            println!();
            println!("{}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/yt-dlp-repl" + reset);
            println!();
            println!("{}", violet.to_owned() + "yt-dlp-repl" + reset + "'" + grey + "s help" + reset + ":");
            println!();
            println!("Provide yt-dlp options as yt-dlp-repl command-line arguments");
            println!("Next, you will be asked for the URL in the interactive shell");
            println!();
            println!("{}", grey.to_owned() + "Example" + reset + ": " + yellow + "yt-dlp-repl -f 140" + reset + "    [" + red + "NOTE" + reset + ": do not insert any URL]");
            println!();
            println!("{}", violet.to_owned() + "yt-dlp" + reset + "'" + grey + "s help" + reset + ":");
            println!();
            Command::new("yt-dlp")
                .arg("--help")
                .status()
                .unwrap();
            exit(0);
        }

        // VERSION

        if argument == "--version" {
            println!("{}", violet.to_owned() + "yt-dlp-repl" + reset + "'" + grey + "s version" + reset + ": " + yellow + "1.0.0" + reset);
            println!("2023.03.29");
            println!();
            println!("{}", violet.to_owned() + "yt-dlp" + reset + "'" + grey + "s version" + reset + ":");
            Command::new("yt-dlp")
                .arg("--version")
                .status()
                .unwrap();
            exit(0);
        }
    }
}
