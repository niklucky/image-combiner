use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageError,
  ImageFormat,
};

pub struct Combiner {
  pub image_1: String,
  pub image_2: String,
}

impl Combiner {
  pub fn new(image_1: String, image_2: String) -> Self {
    Combiner {
      image_1,
      image_2
    }
  }

  pub fn process_images(&mut self) -> Result<(), ImageDataErrors> {
    Ok(())
  }
}


pub struct FloatingImage {
  pub width: u32,
  pub height: u32,
  pub data: Vec<u8>,
  pub name: String,
}

#[derive(Debug)]
pub enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError),
}

impl FloatingImage {
  pub fn new(width: u32, height: u32, name: String) -> Self {
      let buffer_capacity = width * height * 4;
      let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
      FloatingImage {
          width,
          height,
          data: buffer,
          name,
      }
  }
  pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
      if data.len() > self.data.capacity() {
          return Err(ImageDataErrors::BufferTooSmall);
      }
      self.data = data;
      Ok(())
  }
}
