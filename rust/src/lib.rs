use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


// HERE ADD ALL THE CREATED MODULES
// WITH mod {yourmodule};
