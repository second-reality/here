use rand::Rng;
use slint::*;

slint::slint! {
    import { Button } from "std-widgets.slint";
    HelloWorld := Window {
        property <image> map;
        img := Image {
            source: map;
        }
        callback build_map(length, length) -> image;
        Button {
            width: 100px;
            height: 100px;
            text: "click here!";
            clicked => {parent.map = parent.build_map(parent.width, parent.height);}
        }
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

fn main() {
    let h = HelloWorld::new();
    let mut rng = rand::thread_rng();
    h.on_build_map(move |width, height| {
        build_map(width as u32, height as u32, rng.gen_range(150..210))
    });
    h.run();
}
