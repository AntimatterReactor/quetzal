use clap::{arg, Arg, Command};
use libquetzal::Lexer;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_NAME"))
        .about("An interpreter for the Quetzal programming language")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg_required_else_help(true)
}

fn main() {
    
}
