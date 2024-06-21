# use crate in this project
- syn
  use parse rust code to Abstract Syntax Tree
- proc-macro2
- quoto
- darling
  simple parse data structure


# use tool in this project
- cargo-expand
  note: expand marcro to original code
  install: cargo install cargo-expand
  use: cargo expand --example enum_macro

# implement AutoDeref AutoDebug
- #[derive(AutoDeref(mut = true, field = "inner")), AutoDebug]
- pub struct RespBulkString {
- innder: String,
 - #[debug(skip)],
- nothing: ()
- }

- impl Deref for RespMap {
- type Target = BTreeMap<String, RespFrame>;
- fn deref(&self) -> &self.target {
-   &self.0
- }
- }
