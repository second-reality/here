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
    x: u32,
    y: u32,
    zoom: u32,
    data: image::RgbImage,
}

struct TileStorage {
    tiles: std::collections::HashMap<String, Tile>,
}

impl TileStorage {
    fn new() -> Self {
        TileStorage {
            tiles: HashMap::new(),
        }
    }
    fn get_tile(&self, url: &String) -> Option<&Tile> {
        self.tiles.get(url)
    }
    fn set_tile(&mut self, tile: Tile) {
        self.tiles.insert(tile.url.clone(), tile);
    }
}

fn get_tile(storage: &mut TileStorage, zoom: u32, x: u32, y: u32) -> &Tile {
    let url = format!("https://tile.openstreetmap.org/{zoom}/{x}/{y}.png");

    let cached = storage.get_tile(&url);

    if cached.is_none() {
        let mut buf = Vec::new();
        println!("Download {url}");
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
        let tile = Tile {
            url: url.clone(),
            data,
            x,
            y,
            zoom,
        };
        storage.set_tile(tile);
    }

    storage.get_tile(&url).unwrap()
}

fn build_map(storage: &mut TileStorage, w: u32, h: u32) -> image::RgbImage {
    let zoom = 15;

    let lat = 48.85321351868104;
    let long = 2.3494646920113937;
    let pixel = googleprojection::from_ll_to_pixel(&(long, lat), zoom as usize).unwrap();

    let x_center = pixel.0 as u32;
    let y_center = pixel.1 as u32;

    let x_left = x_center - w / 2;
    let y_top = y_center - h / 2;

    let mut map: image::RgbImage = image::ImageBuffer::new(w, h);

    let mut tile = get_tile(storage, zoom, x_left / 256, y_top / 256);
    for (x, y, pixel) in map.enumerate_pixels_mut() {
        let x = x_left + x;
        let y = y_top + y;
        let x_tile = x / 256;
        let y_tile = y / 256;
        if tile.x != x_tile || tile.y != y_tile {
            tile = get_tile(storage, zoom, x_tile, y_tile);
        }
        *pixel = *tile.data.get_pixel(x % 256, y % 256);
    }

    map
}

fn main() {
    let w = MainWindow::new().unwrap();
    let mut storage = TileStorage::new();
    w.on_build_map(move |width, height| {
        let img = build_map(&mut storage, width as u32, height as u32);
        let buffer = slint::SharedPixelBuffer::<slint::Rgb8Pixel>::clone_from_slice(
            img.as_raw(),
            img.width(),
            img.height(),
        );
        slint::Image::from_rgb8(buffer)
    });
    w.run().unwrap();
}
