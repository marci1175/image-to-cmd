use std::{path::PathBuf};

use anyhow::Error;
use image::{GenericImageView, Pixel, Rgb};

const PIXEL: &str = "█";

enum PrintColor {
    Red,
    Green,
    Blue,
    Cyan,
    Grey,
    Purple,
    Yellow,
    Dark,
    White,
}

impl PrintColor {
    fn print(&self, input: &str) {
        let color_code = match self {
            PrintColor::Red => "\x1b[31m",
            PrintColor::Green => "\x1b[32m",
            PrintColor::Blue => "\x1b[34m",
            PrintColor::Cyan => "\x1b[36m",
            PrintColor::Grey => "\x1b[37m",
            PrintColor::Purple => "\x1b[35m",
            PrintColor::Yellow => "\x1b[33m",
            PrintColor::Dark => "\x1b[30m",
            PrintColor::White => "\x1b[0m",
        };

        let terminator = "\x1b[0m";

        print!("{color_code}{input}{terminator}");
    }
}

fn get_pixel_color(pixel_color: Rgb<u8>) -> PrintColor {
    if pixel_color[0] > pixel_color[1] && pixel_color[0] > pixel_color[2] {
        PrintColor::Red
    } else if pixel_color[1] > pixel_color[0] && pixel_color[1] > pixel_color[2] {
        PrintColor::Green
    } else if pixel_color[2] > pixel_color[0] && pixel_color[2] > pixel_color[1] {
        PrintColor::Blue
    } else if pixel_color[0] == 0 && pixel_color[1] == 255 && pixel_color[2] == 255 {
        PrintColor::Cyan
    } else if pixel_color[0] == pixel_color[1] && pixel_color[1] == pixel_color[2] {
        PrintColor::Grey
    } else if pixel_color[0] > 128 && pixel_color[1] == 0 && pixel_color[2] > 128 {
        PrintColor::Purple
    } else if pixel_color[0] > 128 && pixel_color[1] > 128 && pixel_color[2] == 0 {
        PrintColor::Yellow
    } else if pixel_color[0] < 128 && pixel_color[1] < 128 && pixel_color[2] < 128 {
        PrintColor::Dark
    } else {
        PrintColor::White
    }
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();

    let _ = args.next();
    let arg = args.next();

    //If the path has already been passed in
    if let Some(path) = dbg!(arg) {
        if !PathBuf::from(&path).exists() {
            PrintColor::Red.print("INVALID PATH!");
            return Err(Error::msg("Invalid path!"));
        }

        let opened_img = image::open(path)?;

        if let Some(terminal_dimensions) = terminal_size::terminal_size() {
            let dimensions = opened_img.dimensions();

            println!("{:?}", dimensions);

            //Lenght
            for lenght in (0..dimensions.1)
                .step_by((dimensions.0 as f32 / terminal_dimensions.0 .0 as f32).ceil() as usize)
            {
                //Width
                for width in (0..dimensions.0).step_by(
                    (dimensions.1 as f32 / terminal_dimensions.1 .0 as f32).ceil() as usize,
                ) {
                    let pixel = opened_img.get_pixel(width, lenght).to_rgb();

                    let pixel_color = get_pixel_color(pixel);
                    pixel_color.print(PIXEL);
                }
            }
        } else {
            PrintColor::Red.print("TERMINAL NOT FOUND!");
        }
    } else {
        return Err(Error::msg("NO PATH"));
    }

    let _ = std::io::stdin().read_line(&mut String::new());

    Ok(())
}
