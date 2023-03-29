// YT-DLP-REPL VERSION 1.0.0 / MIT LICENSE / COPYRIGHT © 2023 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod menu;
pub mod repl;

// MAIN

fn main() {
    let reset = "\x1b[0m";
    let blue_underlined = "\x1b[34;4m";
    let grey = "\x1b[38;5;240m";
    let red = "\x1b[31m";
    let violet = "\x1b[38;5;133m";
    let yellow = "\x1b[38;5;220m";

    menu::documentation(reset, blue_underlined, grey, red, violet, yellow);
    repl::interactive(reset, blue_underlined, grey, red, violet, yellow);
}
