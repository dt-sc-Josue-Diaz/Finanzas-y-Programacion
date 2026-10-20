use clap::Parser;
use colored::Colorize;

#[derive(Parser)]

struct Options{
    #[clap(default_value = "Meow!")]
    message: String, // [1]

    #[clap(short = 'd', long = "dead")]
    dead:bool,
}

fn main(){
    let options = Options::parse(); // [2]
    let message = options.message;

    let eye = if options.dead {"x"} else {"o"};
    if message.to_lowercase() == "woof" {
        eprintln!("Cats shouldn't bark!!")
    }
    println!(
        "{}",
        message.bright_yellow().underline().on_purple()
    );

    println!(" \\");
    println!(" \\");
    println!(" /\\_/\\");
    println!(" ( {eye} {eye} )", eye = eye.red().bold());
    println!(" =( I )=");

}