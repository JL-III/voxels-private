# voxels

Project structure - functionality is broken up into a set of plugins.
each plugin is designed as a module inside the project.

Each module will contain the following files at minimum
- **mod.rs** - contains the visibility modifiers only
- **plugin.rs** - contains the plugin implementation for the module
- (if applicable) **events.rs** - if a module emits an event, it must exist in the events.rs source file. This is to help in identifying what events are emitted.
- (if applicable) **components.rs** - if a significant number of components exist, they will be broken out into a components.rs file.

## Running
- `cargo build --bins`
- `cargo run --bin voxels`

## TODO
  ### physics
  - add physics
  - https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/
  ### rendering
  - occlusion? field of view?
  - lod?
  - add some sort of queue for chunks? the client will miss some chunks when it is sent from the server.
  - greedy meshing
  - remove mesh from chunks that have neighbors, no reason to render sides of the chunk that cant be seen.
  ### server <-> client
  - fix the fact that all players are updated to the same position on server updates.
  - timing issues based on FixedUpdate scheduler and events. not sure what is causing it.
  - should we keep the same event / io channel structure? right now an event fires, the listener picks it up and emits information over the channel, a sort of linking events cross binary.

## thoughts
- does the client need all the chunk data? this seems to lend itself to cheating later on and also seems like unecessary network load. clients could recieve only what they need in order to render the visible layers, then load all the data in immediate chunks? or only chunks the client is within arms reach of? maybe we only load in the data that can be mutated, so the blocks that are exposed to the client and within arms reach, not sure how much latency this would introduce when a client tries to break blocks.
- i think we need some sort of "primary chunks" registry for the client, like the chunks that are immediately needed for the player to exist and then a "secondary chunks" registry that is further from the player and can take longer to load in. maybe this primary and secondary registry is immutable for the client? maybe a third registry for an even smaller subset of mutable chunks. later on, how does this affect projectiles that might alter chunk data?
- we need a way to ensure the chunks have been loaded for the client. would this be done with manifest that is sent and then loaded with the client? is there a limit to how many times the client will request chunk data? so for instance the manifest contains 10 chunks, the client only recieves 7 for whatever network reason, then the client requests the remaining 3 chunks. what do we do if the client doesnt recieve the manifest? do we require a response to continue?
- will probably need to see how other video games handle stuff like this.