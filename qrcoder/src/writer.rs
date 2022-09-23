use qrcode_png::*;
use std::fs;

pub fn create_qr_code(data: &[u8], filename: &str) {
    let mut qrcode = QrCode::new(data, QrCodeEcc::Medium).unwrap();
    qrcode.margin(10);
    qrcode.zoom(10);
    
    let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
    fs::write(filename.to_owned() + ".png", buf).unwrap();
}
