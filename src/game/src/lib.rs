#![crate_type = "dylib"]
extern crate amethyst;
extern crate amethyst_ecs;
extern crate amethyst_renderer;

use amethyst::engine::{Application, Duration, State, Trans};
use amethyst_ecs::*;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

struct HelloWorld {
    world: World
}

impl State for HelloWorld {
    fn on_start(&mut self) {
        for i in 0..180 {
            let ent = self.world.create_entity();
            self.world.insert_component(ent, Position { x: i as f32 * 0.1, y: 0.0, z: 0.0 });
        }

        println!("Game started!");
    }

    fn update(&mut self, _delta: Duration) -> Trans {
        println!("Component Position #{}: {:?}", 60, self.world.component::<Position>(60).unwrap().1);
        println!("Hello from Amethyst!");
        Trans::Quit
    }

    fn on_stop(&mut self) {
        println!("Game stopped!");
    }
}

#[no_mangle]
pub extern fn run() {
    let mut game = Application::new(HelloWorld {
        world: World::new()
    });

    game.run();
}
