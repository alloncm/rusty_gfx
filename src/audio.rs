use sdl2::sys;
use sdl2::libc::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem::MaybeUninit;

pub struct Audio{
    device_id: sys::SDL_AudioDeviceID,
    pub frequency:i32,
    pub channels:u8
}

impl Audio{
    pub fn init(freq:i32, channels:u8, fps:u16)->Self{

        unsafe{
            sys::SDL_ClearError();
            if sys::SDL_InitSubSystem(sys::SDL_INIT_AUDIO) != 0{
                std::panic!("{}", Self::get_sdl_error_message());
            }
        }

        let samples = (freq / fps as i32) as u16;
        let desired_audio_spec = sys::SDL_AudioSpec{
            freq: freq ,
            format: sys::AUDIO_F32SYS as u16,
            channels:channels,
            silence:0,
            samples:samples,
            padding:0,
            size:(samples as i32 / std::mem::size_of::<f32>() as i32) as u32,
            callback:Option::None,
            userdata:std::ptr::null_mut()
        };
    
        let mut uninit_audio_spec:MaybeUninit<sys::SDL_AudioSpec> = MaybeUninit::uninit();

        unsafe{
            sys::SDL_ClearError();
            let device_id = sys::SDL_OpenAudioDevice(std::ptr::null(), 0, &desired_audio_spec, uninit_audio_spec.as_mut_ptr() , 0);

            if device_id == 0{
                std::panic!("{}",Self::get_sdl_error_message());
            }

            let init_audio_spec:sys::SDL_AudioSpec = uninit_audio_spec.assume_init();

            //This will start the audio processing
            sys::SDL_PauseAudioDevice(device_id, 0);

            return Audio{
                frequency:init_audio_spec.freq,
                channels: init_audio_spec.channels,
                device_id:device_id
            };
        }
    }

    pub fn push_audio_to_device(&self, audio:&[f32])->Result<(),&str>{
        unsafe{
            let audio_ptr: *const c_void = audio.as_ptr() as *const c_void;
            let data_byte_len = (audio.len() * std::mem::size_of::<f32>()) as u32;

            sys::SDL_ClearError();
            if sys::SDL_QueueAudio(self.device_id, audio_ptr, data_byte_len) != 0{
                return Err(Self::get_sdl_error_message());
            }
            
            Ok(())
        }
    }

    fn get_sdl_error_message()->&'static str{
        unsafe{
            let error_message:*const c_char = sys::SDL_GetError();
            
            return CStr::from_ptr(error_message).to_str().unwrap();
        }
    }
}

impl Drop for Audio{
    fn drop(&mut self) {
        unsafe{
            sys::SDL_CloseAudioDevice(self.device_id);
            sys::SDL_QuitSubSystem(sys::SDL_INIT_AUDIO);
        }
    }
}