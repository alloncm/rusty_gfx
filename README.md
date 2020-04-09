# stupid_gfx
A stupid simple minimalist and easy to use wrapper around rust sdl2 bindings

I created this wrapper in order to use sdl in my gameboy emulator but in a more convient and easy way.

## how to build 

Currently this crate is not on crates.io so I recommend to add it as a git submodule
In order to build it you need to install cmake (sdl2 needs it to build itself) and then just hit the good old ```cargo build```

## How to use
Initialize the lib:
```rust
extern crate stupid_gfx;
use stupid_gfx::{
    event::Event,
    event_handler::EventHandler, 
    graphics::Graphics, 
    initializer::Initializer,
    surface::Surface,
    event::Keycode
};

fn main{
    let gfx_initializer: Initializer = Initializer::new();
    //0xFF for white background
    let mut graphics: Graphics = gfx_initializer.init_graphics("app_name", 800, 600, 0xFF);
    let mut event_handler: EventHandler = gfx_initializer.init_event_handler();
}
```

Use the lib 

this code draws a cube that is being moved by the keys arrows 

(table for all the keys - https://wiki.libsdl.org/SDL_Keycode?highlight=%28%5CbCategoryEnum%5Cb%29%7C%28CategoryKeyboard%29)

```rust
    let mut alive = true;
    let mut x = 200;
    let mut y = 200;
    let data = vec![0xFF; 400];
    let surface = Surface::new_from_raw(data, 20, 20);
    while alive {
        graphics.clear();
        for event in event_handler.poll_events() {
            match event {
                Event::KeyPressed(key) => match key {
                    Keycode::Right => x += 1,
                    Keycode::Left => x -= 1,
                    Keycode::Down => y += 1,
                    Keycode::Up => y -= 1,
                    _ => {}
                },  
                Event::Quit => alive = false,
            }
        }
        graphics.draw_surface(x, y, &surface);
        graphics.update();
    }
```
