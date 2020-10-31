pub use sdl2::keyboard::Scancode;

pub enum Event{
    Quit,
    KeyPressed(Scancode)
}