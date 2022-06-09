slint::slint! {
    HelloWorld := Window {
        property <image> map;
        Image {
            source: map;
        }
        Text {
            text: "hello world";
            color: green;
        }
    }
}

use slint::*;

fn main() {
    let mut pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(320, 200);

    for (i, pixel) in pixel_buffer.make_mut_slice().iter_mut().enumerate() {
        let val = (i % 255) as u8;
        *pixel = Rgb8Pixel {
            r: val,
            g: val,
            b: val,
        };
    }

    let image = Image::from_rgb8(pixel_buffer);
    let h = HelloWorld::new();
    h.set_map(image);
    h.run();
}
