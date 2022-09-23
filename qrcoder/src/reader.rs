use rqrr::PreparedImage;
use image::{Luma, ImageBuffer};
use screenshots::{Screen, Image};
use std::{fs, io::Error};

pub fn grab_screen() -> (Vec<Vec<u8>>, Vec<Screen>) {

    // Grab screens
    let screens: Vec<Screen> = Screen::all().unwrap();

    // Collect buffers
    let mut buffers: Vec<Vec<u8>> = Vec::new();
  
    for screen in screens.iter() {
        
        // Print screen debug info
        // println!("capturer {:?}", screen);
        
        // Capture image
        let image: Image = screen.capture().unwrap();
        let buffer: Vec<u8> = image.buffer().to_owned();
        buffers.push(buffer);
    }
    (buffers, screens)
}

pub fn save_screenshots(screen_buffers: Vec<Vec<u8>>, screen_info: Vec<Screen>) -> Vec<String> {
    let mut filenames: Vec<String> = Vec::new();
    for (buffer, screen) in screen_buffers.iter().zip(screen_info) {
        let filename_: String = format!("screen-{}.png", screen.display_info.id);
        fs::write(&filename_, buffer).unwrap();
        filenames.push(filename_);
    }
    filenames
}

pub fn delete_screenshot(filename: String) -> Result<(), Error> {
    fs::remove_file(filename)
}

pub fn load_image(filename: String) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    image::open(filename).unwrap().to_luma8()
}

pub fn extract_qr_from_image(image: ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();
    let mut img: PreparedImage<ImageBuffer<Luma<u8>, Vec<u8>>> = PreparedImage::prepare(image);
    let grids = img.detect_grids();

    if !grids.is_empty() {
        for grid in grids.iter() {
            let (_, content) = grid.decode().unwrap();
            contents.push(content);
        }
    }
    contents
}
