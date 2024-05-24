use std::collections::HashMap;

slint::slint! {
    export component MainWindow inherits Window {
        Image {
            source: build_map(parent.width, parent.height);
        }
        pure callback build_map(length, length) -> image;
    }
}

#[derive(Debug, Clone)]
struct Tile {
    url: String,
    data: image::RgbImage,
}

struct TileStorage {
    tiles: std::collections::HashMap<String, Tile>
}

impl TileStorage {
    fn new() -> Self {
        TileStorage { tiles: HashMap::new() }
    }
    fn get_tile(&self, url: &String) -> Option<&Tile> {
        self.tiles.get(url)
    }
    fn set_tile(&mut self, tile: &Tile) {
        self.tiles.insert(tile.url.clone(), tile.clone());
    }
}

fn get_tile(storage: &mut TileStorage, zoom: u32, x: u32, y: u32) -> Tile {
    let url = format!("https://tile.openstreetmap.org/{zoom}/{x}/{y}.png");
    
    let cached = storage.get_tile(&url);

    if cached.is_some() {
        return cached.unwrap().clone();
    }

    let mut buf = Vec::new();
    let _ = ureq::get(url.as_str())
        .call()
        .unwrap()
        .into_reader()
        .read_to_end(&mut buf);
    let data: image::RgbImage = image::io::Reader::new(std::io::Cursor::new(buf))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    assert_eq!(data.width(), 256);
    assert_eq!(data.height(), 256);
    let tile = Tile { url, data };
    storage.set_tile(&tile);
    tile
}

fn build_map(w: u32, h: u32) -> image::RgbImage {
    let mut storage = TileStorage::new();
    let zoom = 13;
    let x = 1294;
    let y = 2788;

    let mut map: image::RgbImage = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in map.enumerate_pixels_mut() {
        let tile = get_tile(&mut storage, 13, 1294, 2788);
        *pixel = *tile.data.get_pixel(x % 256, y % 256);
    }

    map
}

fn main() {
    let w = MainWindow::new().unwrap();
    w.on_build_map(move |width, height| {
        let img = build_map(width as u32, height as u32);
        let buffer = slint::SharedPixelBuffer::<slint::Rgb8Pixel>::clone_from_slice(
            img.as_raw(),
            img.width(),
            img.height(),
        );
        slint::Image::from_rgb8(buffer)
    });
    w.run().unwrap();
}
