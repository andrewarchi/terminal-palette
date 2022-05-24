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

const TERMINAL_CLEAR: &str = "\x1b[0m";

fn main() {
    let colors = [
        Rgb::from(0xFECB00),
        Rgb::from(0x009FDA),
        Rgb::from(0x952D98),
        Rgb::from(0xFF7900),
        Rgb::from(0x0065BD),
        Rgb::from(0x69BE28),
        Rgb::from(0xED2939),
    ];

    let ansi = xterm::ansi_256();
    let palette = Palette(ansi[16..].to_vec()).to_lab();
    for c in colors {
        let nearest = palette.nearest(c.to_lab()).unwrap() + 16;
        println!(
            "{:#x} {}  {}  \x1b[48;5;{}m  {} {:#x} ({})",
            c,
            c.format_terminal(true),
            ansi[nearest].format_terminal(true),
            nearest,
            TERMINAL_CLEAR,
            ansi[nearest],
            nearest
        );
    }
}
