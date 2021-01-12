
#[repr(C)]
pub struct ArgbColor{
    pub dword: u32
}

impl Copy for ArgbColor{}

impl Clone for ArgbColor{
    fn clone(&self)->Self{
        ArgbColor{
            dword: self.dword
        }
    }
}

impl ArgbColor{

    pub fn new_from_argb(a:u8, r:u8, g:u8, b:u8)->Self{
        let dword:u32 = b as u32|((g as u32)<<8)|((r as u32)<<16)|((a as u32)<<24);
        return ArgbColor{
            dword:dword
        }
    }

    pub fn new_from_dword(dword:u32)->Self{
        ArgbColor{
            dword:dword
        }
    }

    pub fn get_r(&self)->u8{
        let r = (self.dword & 0x00FF0000)>>16;
        return r as u8;
    }
    
    pub fn get_g(&self)->u8{
        let r = (self.dword & 0x0000FF00)>>8;
        return r as u8;
    }
    pub fn get_b(&self)->u8{
        let r = self.dword & 0x000000FF;
        return r as u8;
    }
    pub fn get_a(&self)->u8{
        let r = (self.dword & 0xFF000000)>>24;
        return r as u8;
    }
}