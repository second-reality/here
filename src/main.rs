use image::io::Reader as ImageReader;
use rand::Rng;
use slint::*;

slint::slint! {
    export component MainWindow inherits Window {
        Image {
            source: build_map(parent.width, parent.height);
        }
        pure callback build_map(length, length) -> image;
    }
}

fn get_tile() -> image::RgbImage {
    let mut buf = Vec::new();
    let _ = ureq::get("https://tile.openstreetmap.org/13/1294/2788.png")
        .call()
        .unwrap()
        .into_reader()
        .read_to_end(&mut buf);

    let tile: image::RgbImage = ImageReader::new(std::io::Cursor::new(buf))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    tile
}

fn build_map(w: u32, h: u32) -> image::RgbImage {
    let tile = get_tile();
    //tile

    let mut map: image::RgbImage = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in map.enumerate_pixels_mut() {
        *pixel = *tile.get_pixel(x % 256, y % 256);
    }

    //      let mut a: RgbImage = ImageBuffer::new(10, 10);
    //      {
    //          let b = a.get_mut(3 * 10).unwrap();
    //          *b = 255;
    //      }
    //      assert_eq!(a.get_pixel(0, 1)[0], 255)
    //  Image::from_rgba8(buffer)

    //let mut pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(256, 256);

    //for (i, pixel) in pixel_buffer.make_mut_slice().iter_mut().enumerate() {
    //    let x = i % w as usize;
    //    let val = 10;
    //    *pixel = Rgb8Pixel {
    //        r: val,
    //        g: val,
    //        b: val,
    //    };
    //}
    map
}

fn main() {
    let w = MainWindow::new().unwrap();
    w.on_build_map(move |width, height| {
        let img = build_map(width as u32, height as u32);
        let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
            img.as_raw(),
            img.width(),
            img.height(),
        );
        Image::from_rgb8(buffer)
    });
    w.run().unwrap();
}
