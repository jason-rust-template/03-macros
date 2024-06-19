# use crate in this project
- syn
- proc-macro2
- quoto

# use tool in this project
- cargo-expand
  note: expand marcro to original code
  install: cargo install cargo-expand
  use: cargo expand --example enum_macro
