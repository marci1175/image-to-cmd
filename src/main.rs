use std::{fs, io::Write};

const PIXEL: &str = "█";

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    
    let _ = args.next();
    
    // let red_code = "\x1b[31m";

    // // Text to print in red
    // let text = "Hello, this text will be printed in red!";

    // // ANSI escape code to reset color
    // let reset_code = "\x1b[0m";

    // // Concatenate escape code, text, and reset code
    // let red_text = format!("{}{}{}", red_code, text, reset_code);

    // Print the red text
    // println!("{}", red_text);

    let arg = args.next();

    //If the path has already been passed in
    if let Some(path) = arg {
        let opened_file: Vec<u8> = fs::read(path)?;
    
        if let Some(dimens) = terminal_size::terminal_size() {
            
        }

    }

    Ok(())
}
