use std::vec::Vec;
use crate::argb_color::ArgbColor;

pub struct Surface{
    pub pixels_data:Vec<ArgbColor>,
    pub width:u32,
    pub height:u32
}

impl Surface{
    pub fn new(data:Vec<ArgbColor>, width:u32, height:u32)->Self{
        if data.len() != (width * height) as usize{
            std::panic!("invalid surface data dimensions do not match");
        }

        return Surface{
            pixels_data:data,
            width:width,
            height:height
        }
    }
    pub fn new_from_raw(raw_data:Vec<u32>, width:u32, height:u32)->Self{
        if raw_data.len() != (width * height) as usize{
            std::panic!("invalid surface data dimensions do not match");
        }

        let mut data:Vec<ArgbColor> = Vec::with_capacity(raw_data.capacity());
        for val in raw_data{
            data.push(ArgbColor::new_from_dword(val));
        }

        return Surface{
            pixels_data:data,
            width:width,
            height:height
        }
    }
}