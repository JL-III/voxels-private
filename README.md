# voxels

Project structure - functionality is broken up into a set of plugins.
each plugin is designed as a module inside the project.

Each module will contain the following files at minimum
- **mod.rs** - contains the visibility modifiers only
- **plugin.rs** - contains the plugin implementation for the module
- (if applicable) **events.rs** - if a module emits an event, it must exist in the events.rs source file. This is to help in identifying what events are emitted.
- (if applicable) **components.rs** - if a significant number of components exist, they will be broken out into a components.rs file.
