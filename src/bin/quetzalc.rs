// Copyright (C) 2024 Ezra A. Derrian
// SPDX-License-Identifier: MIT OR Apache-2.0
//! The Quetzal Compiler

use clap::{Arg, ArgAction, Command};
use libquetzal::{Lexer, Parser};
use std::fs;
use std::path::PathBuf;
use std::process;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_NAME"))
        .about("A compiler for the Quetzal programming language")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .after_help("Bug reports: https://github.com/AntimatterReactor/quetzal\n")
        .arg_required_else_help(true)
        // --- pipeline stage flags ---
        .arg(
            Arg::new("lex")
                .long("lex-only")
                .action(ArgAction::SetTrue)
                .help("Stop after lexing; print tokens"),
        )
        .arg(
            Arg::new("ast")
                .long("ast-only")
                .action(ArgAction::SetTrue)
                .help("Stop after parsing; print AST"),
        )
        .arg(
            Arg::new("preprocess")
                .short('E')
                .long("preprocess-only")
                .action(ArgAction::SetTrue)
                .help("Stop after preprocessing"),
        )
        .arg(
            Arg::new("compile")
                .short('S')
                .long("compile-only")
                .action(ArgAction::SetTrue)
                .help("Emit assembly, no link"),
        )
        .arg(
            Arg::new("assemble")
                .short('c')
                .long("assemble-only")
                .action(ArgAction::SetTrue)
                .help("Emit object file, no link"),
        )
        // --- output ---
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .default_value("./a.out")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        // --- optimization -O[0123sf] ---
        .arg(
            Arg::new("opt")
                .short('O')
                .default_value("0")
                .value_parser(["0", "1", "2", "3", "s", "f"])
                .help("Optimization level"),
        )
        // --- warnings -W[0123] ---
        .arg(
            Arg::new("warn")
                .short('W')
                .default_value("1")
                .value_parser(["0", "1", "2", "3"])
                .help("Warning level"),
        )
        // --- verbosity -v / -vv / -vvv ---
        .arg(
            Arg::new("verbose")
                .short('v')
                .action(ArgAction::Count)
                .help("Verbosity (-v, -vv, -vvv)"),
        )
        // --- REPL ---
        .arg(
            Arg::new("repl")
                .long("repl")
                .action(ArgAction::SetTrue)
                .help("Start interactive REPL"),
        )
        // --- input files (positional, 1+) ---
        .arg(
            Arg::new("input")
                .num_args(1..)
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present("repl"),
        )
}

fn compile_source(
    src: &str,
    filename: &str,
    matches: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let verbosity: u8 = matches.get_count("verbose");

    // --- LEX ---
    let mut lexer = Lexer::new(src);
    let tokens = lexer.lexicalize().map_err(|e| {
        eprintln!("quetzalc: {filename}: lexical error: {e}");
        e
    })?;

    if verbosity >= 2 {
        eprintln!("[lex] {} tokens", tokens.len());
    }

    if matches.get_flag("lex") {
        for tok in &tokens {
            println!("{tok:?}");
        }
        return Ok(());
    }

    // --- PARSE ---
    let mut parser = Parser::new(tokens.into_boxed_slice());
    let ast = parser.construct().map_err(|e| {
        eprintln!("quetzalc: {filename}: parse error: {e}");
        e
    })?;

    if verbosity >= 2 {
        eprintln!("[parse] AST built");
    }

    if matches.get_flag("ast") {
        println!("{ast:#?}");
        return Ok(());
    }

    // --- PREPROCESS / COMPILE / ASSEMBLE / LINK ---
    // TODO: wire codegen here
    if matches.get_flag("preprocess") {
        todo!("preprocessor")
    }
    if matches.get_flag("compile") {
        todo!("codegen → asm")
    }
    if matches.get_flag("assemble") {
        todo!("codegen → obj")
    }

    // full compile+link
    todo!("link")
}

fn run_repl() {
    use std::io::{self, BufRead, Write};
    println!("Quetzal REPL — :q to quit, :? for help");

    let stdin = io::stdin();
    let mut line_no: usize = 0;
    let mut buf = String::new();
    let mut open_blocks: i32 = 0;

    loop {
        let prompt = if open_blocks > 0 { "... " } else { ">>> " };
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break, // EOF
            Ok(_) => {}
        }

        let trimmed = line.trim();
        match trimmed {
            ":q" | ":quit" => break,
            ":?" | ":help" => {
                println!(":q      quit");
                println!(":?      help");
                println!(":clear  clear buffer");
                continue;
            }
            ":clear" => {
                buf.clear();
                open_blocks = 0;
                continue;
            }
            "" if open_blocks == 0 => continue,
            _ => {}
        }

        buf.push_str(&line);
        line_no += 1;

        // track indent-block depth heuristic (colon at end = open block)
        if trimmed.ends_with(':') {
            open_blocks += 1;
        } else if open_blocks > 0 && trimmed.is_empty() {
            open_blocks -= 1;
        }

        if open_blocks > 0 {
            continue;
        } // accumulate multi-line

        // eval accumulated buffer
        let src = std::mem::take(&mut buf);
        let mut lexer = Lexer::new(&src);
        match lexer.lexicalize() {
            Err(e) => eprintln!("error: {e}"),
            Ok(tokens) => {
                for tok in &tokens {
                    println!("{tok:?}");
                }
            }
        }
    }

    println!("\nbye");
}

fn main() {
    let matches = cli().get_matches();

    if matches.get_flag("repl") {
        run_repl();
        return;
    }

    let inputs: Vec<PathBuf> = matches
        .get_many::<PathBuf>("input")
        .unwrap()
        .cloned()
        .collect();

    let mut had_error = false;
    for path in &inputs {
        let src = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("quetzalc: cannot read '{}': {e}", path.display());
                had_error = true;
                continue;
            }
        };
        let name = path.to_string_lossy();
        if let Err(_) = compile_source(&src, &name, &matches) {
            had_error = true;
        }
    }

    if had_error {
        process::exit(1);
    }
}
