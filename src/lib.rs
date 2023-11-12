extern crate image;   
                                                                               
use image::{GenericImageView, ImageBuffer, RgbaImage,};

#[allow(dead_code)]
pub enum FilterType {
    Darker,
    Lighter,
}

pub struct Filter {
    buffer: Option<RgbaImage>,
    path: Option<String>,
    filter_type: FilterType,
    img: Option<image::DynamicImage>,
}

impl Filter {
    pub fn new(path: &str, typ: FilterType) -> Self {
        Self {
            buffer: None,
            path: Some(path.to_string()),
            filter_type: typ,
            img: None,
        }
    }

    pub fn open(&mut self) -> &mut Self {
        if !self.path.clone().unwrap().to_string().ends_with(".png") {
            panic!("File needs to end with .png");
        }
        let img = image::open(self.path.as_mut().unwrap()).unwrap();
        let (width, height) = img.dimensions();
        self.img = Some(img);
        let buffer = ImageBuffer::new(width, height);
        self.buffer = Some(buffer);
        self
    }

    pub fn filter(&mut self) -> &mut Self {
        match self.filter_type {
            FilterType::Darker => {
                let mut original_pixels = Vec::new();
                let mut cursor = 0;
                for pixel in self.img.as_mut().unwrap().pixels() {
                    original_pixels.push(pixel.2);
                }
                for pixel in self.buffer.as_mut().unwrap().enumerate_pixels_mut() {
                    original_pixels[cursor][0] = (original_pixels[cursor][0] as f32 * 0.7) as u8;
                    original_pixels[cursor][1] = (original_pixels[cursor][1] as f32 * 0.7) as u8;
                    original_pixels[cursor][2] = (original_pixels[cursor][2] as f32 * 0.7) as u8;
                    *pixel.2 = original_pixels[cursor];
                    cursor+=1;
                }
            },
            FilterType::Lighter => {
                let mut original_pixels = Vec::new();
                let mut cursor = 0;
                for pixel in self.img.as_mut().unwrap().pixels() {
                    original_pixels.push(pixel.2);
                }
                for pixel in self.buffer.as_mut().unwrap().enumerate_pixels_mut() {
                    original_pixels[cursor][0] = (original_pixels[cursor][0] as f32 * 1.3) as u8;
                    original_pixels[cursor][1] = (original_pixels[cursor][1] as f32 * 1.3) as u8;
                    original_pixels[cursor][2] = (original_pixels[cursor][2] as f32 * 1.3) as u8;
                    *pixel.2 = original_pixels[cursor];
                    cursor+=1;
                }
            }
        }
        self
    }

    pub fn save(&mut self, path: String) {
        self.buffer.as_mut().unwrap().save(path).unwrap();
    }
}