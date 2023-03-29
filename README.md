# YT-DLP-REPL

[yt-dlp-repl](https://github.com/piotrbajdek/yt-dlp-repl) utilises the command-line arguments of [yt-dlp](https://github.com/yt-dlp/yt-dlp), and subsequently enters a loop, enabling the user to continually input URLs.

![example-image-1](https://github.com/piotrbajdek/yt-dlp-repl/blob/main/docs/images/example-image-1.png?raw=true)

# INSTALLATION ON LINUX

The Rust-based program [yt-dlp-repl](https://github.com/piotrbajdek/yt-dlp-repl) requires the Python-based program [yt-dlp](https://github.com/yt-dlp/yt-dlp) to be installed on the system as a dependency. To use them together, you must separately install [yt-dlp](https://github.com/yt-dlp/yt-dlp) through your Linux distribution's package repositories.

The current version of yt-dlp-repl (v1.0.0) has been verified to work properly on Fedora Linux 37 and Ubuntu 22.10.

## METHOD 1 – USING CARGO

**[Recommended for programmers]**

**1.** To install yt-dlp-repl from [crates.io](https://crates.io/crates/yt-dlp-repl), use the following [cargo](https://www.rust-lang.org/tools/install) command:

_cargo install yt-dlp-repl_

The executable will be saved in the hidden `.cargo/bin/` directory within your home directory.

**2a.** For easy access, you may want to copy the yt-dlp-repl file to the `/usr/bin/` directory. This can be done by following the instructions in Method 2 (3a, 3b).

**2b.** As an alternative, you can add the `~/.cargo/bin/` directory to your system's PATH variable, which can be configured using [rustup](https://www.rust-lang.org/tools/install).

## METHOD 2 – UNIVERSAL LINUX BINARIES

**1.** To install yt-dlp-repl, first download the distro-independent [binary](https://github.com/piotrbajdek/yt-dlp-repl/releases/download/v1.0.0/yt-dlp-repl) from GitHub.

**2.** Then, make the file executable by running the command:

_sudo chmod +x ./yt-dlp-repl_

**3a.** On most Linux distributions, install yt-dlp-repl by copying the binary to `/usr/bin/`:

_sudo cp yt-dlp-repl /usr/bin/_

**3b.** For Fedora Silverblue / Kinoite, use this command:

_sudo cp yt-dlp-repl /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

**[Recommended for most users]**

Distro-specific packages for [.rpm](https://github.com/piotrbajdek/yt-dlp-repl/releases/download/v1.0.0/yt-dlp-repl-1.0.0-1.x86_64.rpm) and [.deb](https://github.com/piotrbajdek/yt-dlp-repl/releases/download/v1.0.0/yt-dlp-repl_1.0.0_amd64.deb)-based Linux distributions are also available for download. To install yt-dlp-repl on different Linux distributions, follow these instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i yt-dlp-repl-1.0.0-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install yt-dlp-repl-1.0.0-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i yt-dlp-repl_1.0.0_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

First, download and unpack the yt-dlp-repl [source code](https://github.com/piotrbajdek/yt-dlp-repl/archive/refs/tags/v1.0.0.zip) from GitHub. Next, to build and install the program, use the command:

_cargo build \--release && sudo cp target/release/yt-dlp-repl /usr/bin/_
