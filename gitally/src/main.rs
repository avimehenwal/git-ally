extern crate colored; // not needed in Rust 2018

use colored::*;
// use std::process::Command;


fn main() {

    println!("{} {} !", "it".green(), "works".blue().bold());
    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    println!("{}", "this is blue".blue());
    println!("{}", "this is red".red());
    println!("{}", "this is red on blue".red().on_blue());
    println!("{}", "this is also red on blue".on_blue().red());
    println!("{}", "bright colors are welcome as well".on_bright_blue().bright_red());
    println!("{}", "you can also make bold comments".bold());
    println!("{}", "or change advice. This is red".yellow().blue().red());
    println!("{}", "or clear things up. This is default color and style".red().bold().clear());
    println!("{}", "purple and magenta are the same".purple().magenta());
    println!("{}", "and so are normal and clear".normal().clear());
    println!("{}", "you can specify color by string".color("blue").on_color("red"));
    println!("{}", String::from("this also works!").green().bold());
    // println!("{}", format!("{:30}", "format works as expected. This will be padded".blue());
    // println!("{}", format!("{:.3}", "and this will be green but truncated to 3 chars".green());
}
    // let output = if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //             .args(&["/C", "echo hello"])
    //             .output()
    //             .expect("failed to execute process")
    // } else {
    //     Command::new("sh")
    //             .arg("-c")
    //             .arg("echo hello")
    //             .output()
    //             .expect("failed to execute process")
    // };

    // let hello = output.stdout;