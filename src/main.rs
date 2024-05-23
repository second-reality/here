use rand::Rng;
use slint::*;
use ureq::*;

slint::slint! {
    export component MainWindow inherits Window {
        Image {
            source: build_map(parent.width, parent.height);
        }
        pure callback build_map(length, length) -> image;
    }
}

fn get_tile() -> Image {
    let mut buffer = SharedPixelBuffer::<Rgb8Pixel>::new(256, 256);
    let body: String = ureq::get("https://tile.openstreetmap.org/13/1294/2788.png")
        .call().unwrap().into_string().unwrap();
    println!("{}", body);
    Image::from_rgb8(buffer)
}

fn build_map(w: u32, h: u32, aim: u8) -> Image {
    let tile = get_tile();
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
    let w = MainWindow::new().unwrap();
    let mut rng = rand::thread_rng();
    w.on_build_map(move |width, height| {
        let aim = rng.gen_range(150..210);
        build_map(width as u32, height as u32, aim)
    });
    w.run().unwrap();
}
