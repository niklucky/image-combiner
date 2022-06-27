mod args;
use args::Args;

mod combine;
use combine::{ FloatingImage, ImageDataErrors, Combiner };

use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView,
    ImageFormat,
};


fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let mut comb = Combiner::new(args.image_1, args.image_2);
    comb.process_images();
    

    let (image_1, image_format_1) = find_image_from_path(comb.image_1)?;
    let (image_2, image_format_2) = find_image_from_path(comb.image_2)?;

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    let (image_1, image_2) = correct_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data = combine_images(image_1, image_2);
    output.set_data(combined_data)?;

    if let Err(e) = image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_format_1,
    ) {
        Err(ImageDataErrors::UnableToSaveImage(e))
    } else {
        Ok(())
    }
}

fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    match Reader::open(&path) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UnableToDecodeImage(e)),
                }
            } else {
                return Err(ImageDataErrors::UnableToFormatImage(path));
            }
        }
        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e)),
    }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn correct_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if image_2.dimensions() == (width, height) {
        (image_1.resize(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize(width, height, Triangle))
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();
    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rbga(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rbga(&vec_2, i, i + 3));
        }
        i += 4;
    }
    combined_data
}

fn set_rbga(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds"),
        };
        rgba.push(val);
    }
    rgba
}
