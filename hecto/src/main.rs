mod editor;

use editor::Editor;

fn main() {
    #![warn(clippy::all, clippy::pedantic)]
    let mut editor = Editor::default();
    editor.run();
}
