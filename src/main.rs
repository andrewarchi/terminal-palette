#![feature(const_trait_impl)]

mod rgb;
mod xterm;

fn main() {
    for c in xterm::ansi_256() {
        println!("{:#x}", c);
    }
}
