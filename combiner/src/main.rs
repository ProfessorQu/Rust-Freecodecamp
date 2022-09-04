mod args;
use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat, GenericImageView, imageops::FilterType, ImageError};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError),
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        Self {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;

        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();

    let (image1, image1_format) = find_image_from_path(args.image1)?;
    let (image2, image2_format) = find_image_from_path(args.image2)?;

    if image1_format != image2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image1, image2) = standardize_size(image1, image2);
    let mut output = FloatingImage::new(image1.width(), image1.height(), args.output);

    let combined_data = combine_images(image1, image2);

    output.set_data(combined_data)?;

    if let Err(e) = image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image1_format) {
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
            }
            else {
                return Err(ImageDataErrors::UnableToFormatImage(path));
            }
        },
        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e))
    }
}

fn get_smallest_dimension(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let pix1 = dim1.0 * dim1.1;
    let pix2 = dim2.0 * dim2.1;
    
    if pix1 < pix2 {
        dim1
    } else {
        dim2
    }
}

fn standardize_size(image1: DynamicImage, image2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(image1.dimensions(), image2.dimensions());
    println!("Width: {}, Height: {}", width, height);

    if image2.dimensions() == (width, height) {
        (image1.resize_exact(width, height, FilterType::Triangle), image2)
    }
    else {
        (image1, image2.resize_exact(width, height, FilterType::Triangle))
    }
}

fn combine_images(image1: DynamicImage, image2: DynamicImage) -> Vec<u8> {
    let vec1 = image1.to_rgba8().into_vec();
    let vec2 = image2.to_rgba8().into_vec();

    alternate_pixels(vec1, vec2)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec1.len()];

    let mut i = 0;
    while i < vec1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec2, i, i + 3));
        }

        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index is out of bounds!"),
        };
        rgba.push(val);
    }

    rgba
}
