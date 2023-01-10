use image;
use rand;
use vectorize::Matrix;

/// Returns an ImageBuffer containing non-interpolated noise
/// 
/// ## Arguments
/// 
/// * `imgx` - Width of image
/// * `imgx` - Height of image
/// 
pub fn noise(imgx: u32, imgy: u32) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);    
    for pixel in imgbuf.pixels_mut() {
        let val = (rand::random::<f32>()*256.0) as u8;
        *pixel = image::Rgb([val, val, val]);
    };
    
    imgbuf
}