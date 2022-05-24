#![feature(const_trait_impl)]

mod terminal_colors;

fn main() {
    for c in terminal_colors::xterm_ansi_256() {
        println!("{:#x}", c);
    }
}
