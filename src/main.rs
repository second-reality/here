use rand::Rng;
use slint::*;

slint::slint! {
    HelloWorld := Window {
        Image {
            source: build_map(parent.width, parent.height);
        }
        callback build_map(length, length) -> image;
    }
}

fn build_map(w: u32, h: u32, aim: u8) -> Image {
    let mut pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(w, h);

    for (i, pixel) in pixel_buffer.make_mut_slice().iter_mut().enumerate() {
        let x = i % w as usize;
        let val = 255 - (((x * aim as usize) / w as usize) as u8);
        *pixel = Rgb8Pixel {
            r: val,
            g: val,
            b: val,
        };
    }

    Image::from_rgb8(pixel_buffer)
}

use native_dialog::*;

fn main() {
    let h = HelloWorld::new();
    let mut rng = rand::thread_rng();
    h.on_build_map(move |width, height| {
        let aim = rng.gen_range(150..210);
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Show range")
            .set_text(&format!("Aim is {aim}"))
            .show_alert()
            .unwrap();
        build_map(width as u32, height as u32, aim)
    });
    h.run();
}
