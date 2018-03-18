// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! internal class to handle OpenAl context and device.
//!
//! Work as a Singleton, check_al_context must be called before each OpenAl object
//! to be sure that the context is created.

#![macro_use]

use std::ffi::CString;
use std::cell::RefCell;
use std::ptr;
use std::sync::Mutex;
use libc;
use openal::ffi;
use record_context;
use record_context::RecordContext;

lazy_static! {
    static ref AL_CONTEXT: Mutex<Result<OpenAlData, String>> = Mutex::new(OpenAlData::new());
}

#[derive(Clone)]
pub struct OpenAlData {
    pub al_context: ffi::ALCcontextPtr,
    pub al_device: ffi::ALCdevicePtr,
    pub al_capt_device: ffi::ALCdevicePtr
}

impl OpenAlData {
    /// Create a new OpenAlData struct
    ///
    /// Private method.
    fn new() -> Result<OpenAlData, String> {
        let device = unsafe { ffi::alcOpenDevice(ptr::null_mut()) };
        if device == 0 {
            return Err("internal error: cannot open the default device.".to_string());
        }
        let context = unsafe { ffi::alcCreateContext(device, ptr::null_mut()) };
        if context == 0 {
            return Err("internal error: cannot create the OpenAL context.".to_string());
        }
        if unsafe { ffi::alcMakeContextCurrent(context) } == ffi::ALC_FALSE {
            return Err("internal error: cannot make the OpenAL context current.".to_string());
        }

        unsafe {
            libc::atexit(cleanup_openal_context);
        }

        Ok(
            OpenAlData {
                al_context: context,
                al_device: device,
                al_capt_device: 0
            }
        )
    }

    /// Check if the context is created.
    ///
    /// This function check is the OpenAl context is already created.
    /// If context doesn't exist, create it, and store it in a local_data,
    /// else get it from the local data and return it.
    ///
    /// # Return
    /// A result containing nothing if the OpenAlData struct exist,
    /// otherwise an error message.
    pub fn check_al_context() -> Result<(), String> {
        if unsafe { ffi::alcGetCurrentContext() != 0 } {
            return Ok(())
        }
        match AL_CONTEXT.lock() {
            Ok(guard) => {
                match *guard {
                    Ok(_) => Ok(()),
                    Err(ref err) => Err(err.clone())
                }
            },
            Err(poison_error) => Err(
                String::from(format!("Can't lock OpenAL context mutex: {}", poison_error))
            )
        }
    }

    fn is_input_context_init() -> Result<RecordContext, String> {
        match AL_CONTEXT.lock() {
            Ok(mut guard) => {
                if let Ok(ref mut new_context) = *guard {
                    if new_context.al_capt_device != 0 {
                        Ok(record_context::new(new_context.al_capt_device))
                    } else {
                        let c_str = CString::new("ALC_EXT_CAPTURE").unwrap();
                        if unsafe {
                            ffi::alcIsExtensionPresent(new_context.al_device, c_str.as_ptr()) } == ffi::ALC_FALSE {
                            return Err("Error: no input device available on your system.".to_string())
                        } else {
                            new_context.al_capt_device = unsafe {
                            ffi::alcCaptureOpenDevice(ptr::null_mut(),
                                                        44100,
                                                        ffi::AL_FORMAT_MONO16,
                                                        44100) };
                            if new_context.al_capt_device == 0 {
                                return Err("internal error: cannot open the default capture device.".to_string())
                            } else {
                                let cap_device = new_context.al_capt_device;
                                return Ok(record_context::new(cap_device))
                            }
                        }
                    }
                } else {
                    Err("Error: you must request the input context, \
                        in the task where you initialize ears.".to_string())
                }
            },
            Err(poison_error) => Err(
                String::from(format!("Can't lock OpenAL context mutex: {}", poison_error))
            )
        }
    }

    /// Check if AL_SOFT_direct_channels extension is present
    ///
    /// # Return
    /// true if the extension is present, otherwise false.
    pub fn direct_channel_capable() -> bool {
        let c_str = CString::new("AL_SOFT_direct_channels").unwrap();
        unsafe {
            ffi::alIsExtensionPresent(c_str.as_ptr()) == ffi::AL_TRUE
        }
    }

    /// Check if the input context is created.
    ///
    /// This function check if the input OpenAl context is already created.
    /// The input openAL context need the normal AL context + its own extension.
    /// So check if the context exist first, then load the input extension.
    ///
    /// # Return
    /// A result containing nothing if the OpenAlData struct exist,
    /// otherwise an error message.
    pub fn check_al_input_context() -> Result<RecordContext, String> {
        if unsafe { !ffi::alcGetCurrentContext() == 0 } {
            OpenAlData::is_input_context_init()
        } else {
            match OpenAlData::check_al_context() {
                Ok(_)    => OpenAlData::is_input_context_init(),
                Err(err) => Err(err)
            }
        }
    }
}

extern "C" fn cleanup_openal_context() {
    if let Ok(mut guard) = AL_CONTEXT.lock() {
        if let Ok(ref mut context) = *guard {
            unsafe {
                ffi::alcDestroyContext(context.al_context);
                if context.al_capt_device != 0 {
                    ffi::alcCaptureCloseDevice(context.al_capt_device);
                }
                ffi::alcCloseDevice(context.al_device);
            }
        }
    }
}

macro_rules! check_openal_context(
    ($def_ret:expr) => (
            match OpenAlData::check_al_context() {
                Ok(_)    => {},
                Err(err) => { println!("{}", err); return $def_ret; }
            }
        );
);
