use img_utils;

fn main() {
    let img_buffer = img_utils::blank::generators::noise(256, 256);
    img_buffer.save("test.png").unwrap();
}
