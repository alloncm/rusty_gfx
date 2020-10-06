extern crate sdl2;
pub use sdl2::keyboard::Scancode;

pub enum Event{
    Quit,
    KeyPressed(Scancode)
}