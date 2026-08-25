mod editor;

use editor::Editor;

fn main() {
    #![warn(clippy::all, clippy::pedantic)]
    let editor = Editor::default();
    editor.run();
}
