extern crate image;   
                                                                               
use image::{GenericImageView, ImageBuffer, RgbaImage};

pub struct Filter {
    buffer: Option<RgbaImage>,
    path: Option<String>,
    img: Option<image::DynamicImage>,
}

impl Filter {
    pub fn new(path: &str) -> Self {
        Self {
            buffer: None,
            path: Some(path.to_string()),
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

    pub fn save(&mut self, path: String) {
        self.buffer.as_mut().unwrap().save(path).unwrap();
    }

    pub fn light_filter(&mut self, offset: f32) -> &mut Self {
        let mut original_pixels = Vec::new();
        let mut cursor = 0;
        for pixel in self.img.as_mut().unwrap().pixels() {
            original_pixels.push(pixel.2);
        }
        for pixel in self.buffer.as_mut().unwrap().enumerate_pixels_mut() {
            original_pixels[cursor][0] = (original_pixels[cursor][0] as f32 * offset) as u8;
            original_pixels[cursor][1] = (original_pixels[cursor][1] as f32 * offset) as u8;
            original_pixels[cursor][2] = (original_pixels[cursor][2] as f32 * offset) as u8;
            *pixel.2 = original_pixels[cursor];
            cursor+=1;
        }
        self
    }

    pub fn box_blur(&mut self, offset: u32) -> &mut Self {
        let mut original_pixels = Vec::new();
        let mut cursor = 0;
        let mut red_sum = 0;
        let mut green_sum = 0;
        let mut blue_sum = 0;
        for pixel in self.img.as_mut().unwrap().pixels() {
            original_pixels.push(pixel.2);
        }
        for pixel in self.buffer.as_mut().unwrap().enumerate_pixels_mut() {
            if original_pixels.len() - cursor == (offset as i32).try_into().unwrap() {
                return self;
            }
            for i in 0..offset as usize {
                red_sum += original_pixels[cursor + i][0] / offset as u8;
                green_sum += original_pixels[cursor + i][1] / offset as u8;
                blue_sum += original_pixels[cursor + i][2] / offset as u8;
            }
            original_pixels[cursor][0] = red_sum as u8;
            original_pixels[cursor][1] = green_sum as u8;
            original_pixels[cursor][2] = blue_sum as u8;
            *pixel.2 = original_pixels[cursor];
            cursor+=1;
            red_sum = 0;
            green_sum = 0;
            blue_sum = 0;
        }
        self
    }
}