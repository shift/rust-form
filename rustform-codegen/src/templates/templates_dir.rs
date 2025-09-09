use include_dir::{include_dir, Dir};

pub static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn get_templates() -> &'static Dir<'static> {
    &TEMPLATES_DIR
}
