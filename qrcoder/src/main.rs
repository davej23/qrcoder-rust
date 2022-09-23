use clap::{Arg, App};
use image::{ImageBuffer, Luma};
use cli_clipboard;
mod reader;

fn main() {

    // App Information
    let app = App::new("qrcoder")
        .version("0.1")
        .about("Read QR codes")
        .author("davej23");

    // Add arg for image
    let image_filename_arg = Arg::with_name("filename")  // info for --image flag
                                .long("image") // flag --image
                                .takes_value(true)  // takes value of filename
                                .help("Filename of image with QR code in")  // helper text
                                .required(false);  // not required as o/w use screenshot

    // Look for matching arguments
    let arg_matches = app.arg(image_filename_arg).get_matches();

    // Get image filename, if specified else empty string
    let image_filename = arg_matches.value_of("filename").unwrap_or("");

    // Collect filename(s)
    let mut filenames: Vec<String> = Vec::new();

    if image_filename.is_empty() {  // If no image specified by user, use screenshot
        // Grab screen(s)
        let (buffers, screen_info) = reader::grab_screen();

        // Save screenshot temporary file(s)
        filenames = reader::save_screenshots(buffers, screen_info);
    } else {  // If image specified, use that
        filenames.push(image_filename.to_string());
    }

    // Load image(s), find QR codes, and extract data
    let mut contents: Vec<Vec<String>> = Vec::new();
    for filename in filenames.iter() {
        // Load image
        let image: ImageBuffer<Luma<u8>, Vec<u8>> = reader::load_image(filename.to_string());
        
        // Extract data from QR, if found
        contents.push(reader::extract_qr_from_image(image));
    }

    // Remove temporary screenshot file(s)
    if image_filename.is_empty() {
        for filename in filenames.iter() {
            reader::delete_screenshot(filename.to_string()).unwrap()
        }
    }

    // Print QR contents
    for content_vec in contents.iter() {
        for content in content_vec.iter() {

            // Print content
            println!("{:?}", content);

            // Add to clipboard
            cli_clipboard::set_contents(content.to_owned()).unwrap()

        }
    }
}
