#![warn(clippy::all, clippy::pedantic)]
// #[allow(dead_code)]
mod editor;
mod terminal;
mod document;
mod row;

pub use terminal::Terminal;
pub use editor::Position;
use editor::Editor;
use document::Document;
use row::Row;

fn main() {
    Editor::new().run();
}
