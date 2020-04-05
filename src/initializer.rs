extern crate sdl2;
use sdl2::Sdl;
use crate::event_handler::EventHandler;
use crate::graphics::Graphics;
use crate::event::Event;

pub struct Initializer{
    sdl_context: Sdl
}

impl Initializer{
    pub fn new()->Self{
        let context = sdl2::init().unwrap();

        return Initializer{
            sdl_context: context
        };
    }

    pub fn init_event_handler<F>(&self)->EventHandler<F> 
    where F:FnMut(Event)
    {
        return EventHandler::init(&self.sdl_context);
    }

    pub fn init_graphics(&self, title:&str, x:u32, y:u32)->Graphics{    
        return Graphics::init(&self.sdl_context, title, x, y);
    }
}