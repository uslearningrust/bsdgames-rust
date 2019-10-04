/*
 * wargames : A silly program either launches a program in /usr/games/
 *            or dispenses timeless advice. 
 */

/*
 * This is a shellscript in bsdgames. There's no reason it can't be
 * here either, but the goal is to learn Rust, so here goes. 
 */

use std::io;
use std::process;
use regex::Regex;

fn display_advice_and_exit() -> ! {
    println!("A strange game.\nThe only winning move is\nnot to play.\n");
    process::exit(0);
}

fn main() {
    let mut gamestr = String::new();

    println!("Shall we play a game?");
    /* How to make this ignore the output? */
    match io::stdin().read_line(&mut gamestr) {
        Err(_) => display_advice_and_exit(),
        Ok(_) => (),
    };

    let re = match Regex::new(r"[^-a-z0-9]") {
        Ok(r) => r,
        Err(_e) => { 
            println!("Error compiling regex\n");
            process::exit(-1);
        },
    };

    let sanitized = re.replace_all(&gamestr, "");

    let prog_path = format!("/usr/games/{}", sanitized);
    let status = match process::Command::new(prog_path).status() {
        Ok(_) => 0,
        Err(_) => { 
            display_advice_and_exit() 
        },
    };

    process::exit(status);
}
