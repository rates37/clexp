# `Clexp` - The Terminal-base File Explorer

Clexp is a fast, keyboard-centric terminal-based file explorer, written in Rust using [ratatui](https://github.com/ratatui/ratatui). It aims to rival graphical file managers in terms of visual intuitiveness and ergonomics, while embracing the simplicity, speed, and convenience of the command line.

---

## Installation:

### Build & Run

1. **Install Rust** (if you haven't):

   Installation instructions are provided on the [rust-lang website](https://www.rust-lang.org/tools/install).

2. **Clone and Build:**

```sh
git clone https://github.com/rates37/clexp.git
cd clexp
cargo build --release
```

3. **Run:**

```sh
cargo run
```

## Usage

### Navigation

- **Arrow Keys**: Navigate the file list with arrow keys`↑/↓`.
- **Enter / →**: Enter a directory.
- **←**: Go up one directory.
- **Path Bar**: Always see your current working directory at the top.

### File Operations

- **Rename (`r`)**: Rename the selected file or directory.
- **Delete (`d`)**: Delete the selected file or directory. Prompts for confirmation (`y`/`n`).
- **Copy (`c`)**: Copy the selected file or directory to the clipboard.
- **Cut (`x`)**: Cut the selected file or directory to the clipboard.
- **Paste (`v`)**: Paste clipboard contents into the current directory. Handles both copy and cut.
- **New File (`n`)**: Create a new file in the current directory.
- **New Directory (`N`)**: Create a new directory in the current directory.
- **Batch Operations**: In multi-selection mode, perform operations on multiple selected files.

### Multi-Selection Mode

- **Enter Selection Mode (`s`)**: Press `s` to enter multi-selection mode.
- **Toggle Selection (`Space`)**: Toggle selection of the current file/directory.
- **Batch Delete (`d`)**: Delete all selected items with confirmation.
- **Batch Copy (`c`)**: Copy all selected items to clipboard.
- **Batch Cut (`x`)**: Cut all selected items to clipboard.
- **Visual Indication**: Selected items show checkboxes `[x]` and highlighted background.

### General Features:

- **Clipboard Modal**: Press `c` to view clipboard contents in a scrollable list.
- **Help Modal**: Press `?` or type `help` in the command window to see a scrollable help dialog with all keybindings and commands.
- **Status Bar**: Context-aware messages and key hints at the bottom.

## Contributing:

Contributions are warmly welcome! Please open issues or pull requests for bugs, features, and improvements! In description of issues/pull requests, please include a detailed explanation of the changes you're making/requesting.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
