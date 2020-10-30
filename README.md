# stupid_gfx
A stupid simple minimalist and easy to use wrapper around rust sdl2 bindings

I created this wrapper in order to use sdl in my gameboy emulator but in a more convient and easy way.

## how to build 

Currently this crate is not on crates.io so I recommend to add it as a git submodule
In order to build it you need to install cmake (sdl2 needs it to build itself) and then just hit the good old ```cargo build```

## How to use
Initialize the lib:
```rust
use stupid_gfx::{
    event::Event,
    event_handler::EventHandler, 
    graphics::Graphics, 
    initializer::Initializer,
    surface::Surface,
    event::Scancode
};

fn main{
    let gfx_initializer: Initializer = Initializer::new();
    //0xFF for white background, true for vsync
    let mut graphics: Graphics = gfx_initializer.init_graphics("app_name", 800, 600, 0xFF, true);
    let mut event_handler: EventHandler = gfx_initializer.init_event_handler();
}
```

Use the lib 

this code draws a cube that is being moved by the keys arrows 

(table for all the keys - https://wiki.libsdl.org/SDLScancodeLookup)

```rust
    let mut alive = true;
    let mut x = 200;
    let mut y = 200;
    let data = vec![0xFF; 400];
    let surface = Surface::new_from_raw(data, 20, 20);
    while alive {
        graphics.clear();
        for event in event_handler.get_events() {
            match event {
                Event::KeyPressed(key) => match key {
                    Scancode::Right => x += 1,
                    Scancode::Left => x -= 1,
                    Scancode::Down => y += 1,
                    Scancode::Up => y -= 1,
                    _ => {}
                },  
                Event::Quit => alive = false,
            }
        }
        graphics.draw_surface(x, y, &surface);
        graphics.update();
    }
```
