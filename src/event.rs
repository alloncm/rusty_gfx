extern crate sdl2;
pub use sdl2::keyboard::Keycode;

pub enum Event{
    Quit,
    KeyPressed(Keycode)
}