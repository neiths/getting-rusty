mod editor;

use editor::Editor;

fn main() {
    #![warn(clippy::all, clippy::pedantic, clippy::print_stdout, clippy::print_stderr)]
    let mut editor = Editor::default();
    editor.run();
}
