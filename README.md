# bevy_mod_debug_console

The standard plugin takes over the stdin/stdout from bevy to get runtime information
from bevy.  In `examples/egui_console.rs` you can also see how to integrate with bevy_console.

```
Running `target\release\bevy_test_game.exe`
Bevy Console Debugger.  Type 'help' for list of commands.
>>> archetypes find --componentname Player

archetype ids:
8, 9, 10,

>>> archetype info --id 10

id: ArchetypeId(8)
table_id: TableId(7)
entities (1): 262,
table_components (17): 114 Transform, 115 GlobalTransform, 116 Draw, 120 Animations, 121 Animator, 122 Handle<Text
ureAtlas>, 123 TextureAtlasSprite, 126 PixelPosition, 128 Layer, 129 SpriteSize, 130 Hurtbox, 131 Player, 136 Curr
entPosition, 145 Visible, 147 RenderPipelines, 153 MainPass, 155 Handle<Mesh>,
sparse set components (0):
```

## Usage

Add to you `Cargo.toml` file:

```toml
[dependencies]
bevy = "0.5"
bevy_mod_debug_console = "0.0.2"
```

Add Plugin:

```rs
use bevy::prelude::*;
use bevy_mod_debug_console::ConsoleDebugPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsoleDebugPlugin)
        .run();
}
```

Once your bevy application is running type commands into the the console. Type `help` to get a list of commands.

## Selection of Available Commands

* `archetype info --id 10` lists id, table_id, entities, table_components, and sparse set components belonging to archetype id `10`
* `components list --long --filter bevy_test_game` lists components from the `bevy_test_game` namespace.
* `counts` print counts of archetypes, components, and entities.
* `pause` pause the game loop to freeze changes in the ecs for inspection. **Warning** This can have adverse affects with physics as the tick is paused and the time delta on resume can then be very large.

## Future Work

* Add RenderGraph information
* Add System and Schedule information
* Add Reflection information

