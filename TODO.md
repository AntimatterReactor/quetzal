# TODOs

These are TODOs for quetzal. Every release will start a new TODO list.

## Lexer

- [x] complete the match statement
- [x] operator token matching currently doesn't work correctly, fix it
- [x] fix how line/column is handled
- [ ] make emit token INDENT and DEDENT (like python, yes)
- [x] handle `char`s
- [ ] ignore comments
- [ ] fix operator table

## Parser

- [x] create good AST objects
- [x] create AST printing
- [ ] modify `ParserError`s to be meaningful
- [ ] change AST pretty-printing to iterative instead of recursive

## Frontend

- [x] create proper cli using `clap`
- [ ] create a `rustyline` REPL
- [ ] make error handling better
  - [ ] fix error.rs
  - [ ] make more descriptive error messages

## Backend

- [ ] use `cxx` to interop c++ to use llvm

## Tests and Benchmarks

- [ ] fix lexer unit tests
- [ ] add lexer benchmarkings
- [ ] add lexer-parser integration testing

## Chore

- [x] copyright text on top of every file
- [ ] contribution guideline & docs
  - [x] code of conduct
  - [ ] technical specification
    - [x] grammar specification
  - [ ] technical details
- [ ] complete build instruction
- [ ] language documentation
  - [ ] standardized docs static page?
  - [ ] references
  - [ ] tutorial
