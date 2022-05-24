#![feature(const_trait_impl)]

mod lab;
mod palette;
mod point;
mod rgb;
mod xterm;
mod xyz;

pub use lab::Lab;
pub use palette::Palette;
pub use point::Point3;
pub use rgb::Rgb;
pub use xyz::Xyz;

fn main() {
    for c in xterm::ansi_256() {
        println!("{:#x}", c);
    }
}
