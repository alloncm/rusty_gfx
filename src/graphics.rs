extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;
use sdl2::video::DisplayMode;
use sdl2::Sdl;
use std::option::Option;
use crate::surface::Surface;
use std::mem::size_of;
use std::ptr;
use crate::argb_color::ArgbColor;
use std::mem::transmute;

pub struct Graphics{
    canvas: Canvas<Window>,
    texture:Texture,
    pixels:Vec<u32>,
    pub width:u32,
    pub height:u32,
    pub background_value:u8
}

impl Graphics{
    pub fn init(context:&Sdl, title:&str, x:u32, y:u32, background_color:u8)->Self{
        let video_subsystem = context.video().unwrap();
        let mut window = video_subsystem.window(title, x, y).build().unwrap();
        window.set_display_mode(DisplayMode::new(PixelFormatEnum::ARGB32,x as i32, y as i32, 0)).unwrap();
        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_static(Option::None,x,y).unwrap();
        return Graphics{
            canvas: canvas,
            texture:texture,
            pixels:vec![0;(x*y) as usize],
            width:x,
            height:y,
            background_value: background_color
        };
    }

    pub fn put_pixel(&mut self,x:u32, y:u32, r:u8, g:u8, b:u8){
        let argb_pixel:u32 = (r as u32) <<16 | (g as u32)<<8 | b as u32;
        self.pixels[(y*self.width + x) as usize] = argb_pixel;
    }

    pub fn update(&mut self){
        let (_,raw_pixels,_) = unsafe{self.pixels.as_slice().align_to::<u8>()};
        self.texture.update(Option::None, raw_pixels, (self.width as usize)*size_of::<u32>()).unwrap();
        self.update_canvas();
    }

    pub fn clear(&mut self){
        unsafe{
            ptr::write_bytes(self.pixels.as_mut_ptr(), self.background_value, self.pixels.len());
        }
    }

    pub fn draw_surface(&mut self, x:u32, y:u32, surface: &Surface){
        unsafe{
            let raw_data = transmute::<*const ArgbColor, *const u32>(surface.pixels_data.as_ptr());

            for i in 0..surface.height{
                let dest_ptr = self.pixels.as_mut_ptr().offset(((y+i)*self.width + x) as isize);
                let src_ptr = raw_data.offset((i*surface.width) as isize);
                ptr::copy_nonoverlapping(src_ptr, dest_ptr, surface.width as usize);
            }
        }
    }

    fn update_canvas(&mut self){
        self.canvas.copy(&self.texture, Option::None, Option::None).unwrap();
        self.canvas.present();
    }
}