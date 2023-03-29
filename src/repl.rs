// YT-DLP-REPL VERSION 1.0.0 / MIT LICENSE / COPYRIGHT © 2023 PIOTR BAJDEK

// MODULE REPL

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cognitive_complexity, clippy::missing_panics_doc, clippy::similar_names, clippy::too_many_lines)]

// IMPORTS

use std::env;
use std::io;
use std::process::Command;

// HELP

fn help(reset: &str, blue_underlined: &str, grey: &str, red: &str, violet: &str, yellow: &str) {
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
}

// LICENSE

fn license(reset: &str, yellow: &str) {
    println!("{}", yellow.to_owned() + "MIT License" + reset);
    println!();
    println!("Copyright © 2023 Piotr Bajdek");
    println!();
    println!("Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:");
    println!();
    println!("The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.");
    println!();
    println!("THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
}

// REPL

pub fn interactive(reset: &str, blue_underlined: &str, grey: &str, red: &str, violet: &str, yellow: &str) {
    let args: Vec<String> = env::args().collect();

    for argument in env::args() {
        if argument.starts_with("http") {
            println!("{}", red.to_owned() + "ERROR: Incorrect arguments provided: Detected a URL" + reset);
            println!();
            println!("{}", violet.to_owned() + "yt-dlp-repl" + reset + " source and documentation:");
            println!();
            println!("{}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/yt-dlp-repl" + reset);
            println!();
            println!("{}", violet.to_owned() + "yt-dlp-repl" + reset + " help:");
            println!();
            println!("Provide yt-dlp options as yt-dlp-repl command-line arguments");
            println!("Next, you will be asked for the URL in the interactive shell");
            println!();
            println!("{}", grey.to_owned() + "Example" + reset + ": " + yellow + "yt-dlp-repl -f 140" + reset + "    [" + red + "NOTE" + reset + ": do not insert any URL]");
            return;
        }
    }

    let arg_cnt = args.len();

    if arg_cnt == 1 {
        println!("{}", red.to_owned() + "WARNING: No yt-dlp options provided" + reset);
        println!();
        println!("Provide yt-dlp options as yt-dlp-repl command-line arguments");
        println!("Next, you will be asked for the URL in the interactive shell");
        println!();
        println!("{}", grey.to_owned() + "Example" + reset + ": " + yellow + "yt-dlp-repl -f 140" + reset + "    [" + red + "NOTE" + reset + ": do not insert any URL]");
        println!();
    }

    println!("Type {}", violet.to_owned() + "quit" + reset + " or " + violet + "exit" + reset + " to close the REPL");
    println!("Type {}", violet.to_owned() + "help" + reset + " to see the usage and exit");
    println!("Type {}", violet.to_owned() + "license" + reset + " to display the license");
    println!();
    println!("{}", grey.to_owned() + "Insert your URL" + reset + ":");

    if arg_cnt == 1 {
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 2 {
        let arg1 = args.get(1).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 3 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 4 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 5 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 6 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 7 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 8 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        let arg7 = args.get(7).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .arg(arg7)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 9 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        let arg7 = args.get(7).unwrap();
        let arg8 = args.get(8).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .arg(arg7)
                .arg(arg8)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 10 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        let arg7 = args.get(7).unwrap();
        let arg8 = args.get(8).unwrap();
        let arg9 = args.get(9).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .arg(arg7)
                .arg(arg8)
                .arg(arg9)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 11 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        let arg7 = args.get(7).unwrap();
        let arg8 = args.get(8).unwrap();
        let arg9 = args.get(9).unwrap();
        let arg10 = args.get(10).unwrap();
        loop {
            println!("{violet}");
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{reset}");
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, grey, red, violet, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("yt-dlp")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .arg(arg7)
                .arg(arg8)
                .arg(arg9)
                .arg(arg10)
                .status()
                .unwrap();
        }
    }
}
