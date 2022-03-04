pub mod abi;
use abi::{operation::Event, *};
use abi::{image::*};
use bytes::Bytes;

impl Operation {
    pub fn new_resize(width: usize, height: usize) -> Self {
        Self {
            event: Some(Event::Resize(Resize{ 
                width: width as u32, height: height as u32,
            }))
        }
    }
}

impl Image {
    pub fn new(width: usize, height: usize, data: Bytes) -> Self {
        Self {
            width: width as u32,
            height: height as u32,
            mode: Mode::Rgba.into(),
            data: data,
        }
    }
}