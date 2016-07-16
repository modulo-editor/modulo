modulo
===
[![Build Status](https://travis-ci.org/modulo-editor/modulo.svg?branch=master)](https://travis-ci.org/modulo-editor/modulo)

Modulo is a new text editor with the goals of being highly parallel and native. We are currently targeting OS X, but plan to expand to Linux and Windows in the future.

Modulo is written in Rust and is composed of 3 main crates:
  * `modulo` ([docs](https://modulo-editor.github.io/modulo/modulo/)) - Holds the backbone of the editor. This handles loading and editing files and storing file-related state.
  * `modulo_traits` ([docs](https://modulo-editor.github.io/modulo/modulo_traits/)) - Stores the shared data structures that will be used in `modulo`, `modulo_ui`, and any modulo plugins.
  * `modulo_ui` ([docs](https://modulo-editor.github.io/modulo/modulo_ui/)) - Holds the front end renderer for modulo. This will handle all user interaction and will be rendered using `webrender`.
