#![feature(const_convert, const_for, const_mut_refs, const_trait_impl)]

mod terminal_colors;

fn main() {
    println!("{:?}", terminal_colors::XTERM_ANSI_COLORS);
}
