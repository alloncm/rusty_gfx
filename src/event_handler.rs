extern crate sdl2;
use sdl2::event::Event as SdlEvent;
use sdl2::EventPump;
use sdl2::Sdl;
use std::option::Option;
use crate::event::Event;

pub struct EventHandler{
    event_pump:EventPump,
}

impl EventHandler{
    pub fn init(context:&Sdl)->Self{
        let event_handler = context.event_pump().unwrap();
        return EventHandler{
            event_pump: event_handler
        };
    }

    pub fn get_events(&mut self)->Vec<Event>{
        let mut events = Vec::new();
        for sdl_event in self.event_pump.poll_iter(){
            match Self::sdlevent_into_event(&sdl_event){
                Some(event)=>events.push(event),
                None=>{}
            }
        }

        return events;
    }

    pub fn pull_events(&mut self){
        self.event_pump.pump_events();
    }

    pub fn has_input_device_event_occurred(&self, event:Event)->bool{
        match event{
            Event::KeyPressed(key) => self.event_pump.keyboard_state().is_scancode_pressed(key),
            _=>std::panic!("not supported event type")
        }
    }

    fn sdlevent_into_event(sdl_event: &SdlEvent)->Option<Event>{
        return match sdl_event{
            SdlEvent::Quit{timestamp:_}=>Option::Some(Event::Quit),
            SdlEvent::KeyDown{timestamp:_, window_id:_, keycode:_, scancode:key,keymod:_, repeat:_}=>{
                match key{
                    Some(val)=>Option::Some(Event::KeyPressed(*val)),
                    None=>Option::None
                }
            }
            _=>Option::None
        }
    }
}