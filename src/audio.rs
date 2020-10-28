use sdl2::sys;


pub struct Audio{
    device_id: sys::SDL_AudioDeviceID,
    frequency:i32,
    channels:u8
}

impl Audio{
    pub fn new(freq:i32, channels:u8)->Self{
        let desired_audio_spec = sys::SDL_AudioSpec{
            freq: freq ,
            format: sys::AUDIO_F32SYS as u16,
            channels:channels,
            silence:0,
            samples:(freq  / std::mem::size_of::<f32>() as i32) as u16,
            padding:0,
            size:133*4,
            callback:Option::None,
            userdata:std::ptr::null_mut()
        };
    
        let audio_spec:*mut sys::SDL_AudioSpec = std::ptr::null_mut();

        unsafe{
            let device_id = sys::SDL_OpenAudioDevice(std::ptr::null(), 0, &desired_audio_spec, audio_spec , 0);

            let audio = Audio{
                frequency:(*audio_spec).freq,
                channels: (*audio_spec).channels,
                device_id:device_id
            };

            return audio;
        }
    }
}