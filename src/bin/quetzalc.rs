//! The Quetzal Compiler
// Copyright (C) 2024  Ezra Alvarion

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::{arg, Arg, ArgAction, Command};
use libquetzal::{Lexer, /* Parser */};
use log::debug;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{env, process};

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_NAME"))
        .about("A compiler for the Quetzal programming language")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .after_help(
            "For bug reporting purposes please go to
https://github.com/AntimatterReactor/quetzal-lang\n",
        )
        .arg_required_else_help(true)
        .arg(
            Arg::new("file")
                .short('o')
                .long("output")
                .default_value("./a.out")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(Arg::new("lex").long("lex-only").action(ArgAction::SetTrue))
        .arg(Arg::new("ast").long("ast-only").action(ArgAction::SetTrue))
        .arg(
            Arg::new("preprocess")
                .short('E')
                .long("preprocess-only")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("compile")
                .short('S')
                .long("compile-only")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("assemble")
                .short('c')
                .long("assemble-only")
                .action(ArgAction::SetTrue),
        )
    // TODO: IMPLEMENT THESE:
    // -O[0,1,2,3,s,f]
    // -W[0,1,2,3]
    // -v[,vv,vvv]
}

fn main() {
    let matches = cli().get_matches();
    let input = String::new(); // replace with from file
    let tokens = Lexer::new(input.as_str()).lexicalize().expect("msg");
    debug!("Tokens: {tokens:#?}");
    // let ast = Parser::new(tokens.into()).construct();
}
