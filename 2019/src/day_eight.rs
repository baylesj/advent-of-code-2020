use std::fs::File;
use std::io::Read;

const INPUT_FILENAME: &'static str = "input/day_eight.txt";

struct ImageSize {
    width: usize,
    height: usize,
}

const IMAGE_SIZE: ImageSize = ImageSize {
    width: 25,
    height: 6,
};

const LAYER_SIZE: usize = IMAGE_SIZE.width * IMAGE_SIZE.height;

const PIXEL_BLACK: char = '0';
const PIXEL_WHITE: char = '1';
const PIXEL_TRANSPARENT: char = '2';
type Pixel = u8;
type Layer = [Pixel];
type Image = Vec<Pixel>;

#[derive(Debug, Default)]
struct Matches {
    black: i32,
    white: i32,
    transparent: i32,
}

fn get_matches_for_layer(layer: &Layer) -> Matches {
    let mut matches = Matches::default();
    for pixel in layer {
        match *pixel as char {
            PIXEL_BLACK => matches.black += 1,
            PIXEL_WHITE => matches.white += 1,
            PIXEL_TRANSPARENT => matches.transparent += 1,
            _ => (),
        }
    }
    matches
}

fn load_input() -> Image {
    let mut f = File::open(INPUT_FILENAME).expect("failed file open");
    let mut buffer: Vec<Pixel> = Vec::new();
    f.read_to_end(&mut buffer).expect("failed buf read");
    buffer
}

fn get_matches(image: &Image) -> Vec<Matches> {
    let mut matches = Vec::new();
    for chunk in image.chunks(LAYER_SIZE) {
        matches.push(get_matches_for_layer(chunk));
    }
    matches
}

fn part_one(image: &Image) -> String {
    let matches = get_matches(&image);
    let fewest_black = matches.iter().min_by_key(|m| m.black).expect("failed min");
    (fewest_black.white * fewest_black.transparent).to_string()
}

fn part_two(image: &Image) -> String {
    let mut chunks_it = image.chunks(LAYER_SIZE);
    let mut image_buffer: Vec<Pixel> = chunks_it.next().expect("missing first layer").to_vec();
    while let Some(chunk) = chunks_it.next() {
        for (i, pixel) in chunk.iter().enumerate() {
            if image_buffer[i] as char == PIXEL_TRANSPARENT && *pixel as char != PIXEL_TRANSPARENT {
                image_buffer[i] = *pixel;
            }
        }
    }

    let mut output = String::default();
    for line in image_buffer.chunks(IMAGE_SIZE.width) {
        output += &format!(
            "    {}\n",
            line.iter()
                .map(|c| if *c as char == PIXEL_BLACK {
                    '⬛'
                } else {
                    '⬜'
                })
                .collect::<String>()
        );
    }
    output
}

pub fn solve() -> String {
    let image = load_input();
    format!(
        "part one: {}, part two:\n{}",
        part_one(&image),
        part_two(&image)
    )
}
