extern crate colored; // not needed in Rust 2018

use colored::Colorize;
use std::process::Command;


fn main() {

    // println!("{} {} !", "it".green(), "works".blue().bold());
    // println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    println!("{}", "this is blue".blue());
    // println!("{}", "this is red".red());
    // println!("{}", "this is red on blue".red().on_blue());
    // println!("{}", "this is also red on blue".on_blue().red());
    // println!("{}", "bright colors are welcome as well".on_bright_blue().bright_red());
    // println!("{}", "you can also make bold comments".bold());
    // println!("{}", "or change advice. This is red".yellow().blue().red());
    // println!("{}", "or clear things up. This is default color and style".red().bold().clear());
    // println!("{}", "purple and magenta are the same".purple().magenta());
    // println!("{}", "and so are normal and clear".normal().clear());
    // println!("{}", "you can specify color by string".color("blue").on_color("red"));
    // println!("{}", String::from("this also works!").green().bold());

    let output = Command::new("git")
                        .arg("status")
                        .output()
                        .expect("failed to execute process");

    println!("status: {}", &output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout).blue());
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    // print unicode emoji
    let emoji = '\u{1F600}';
    println!("{:?}", emoji);
}