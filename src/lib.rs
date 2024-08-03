use std::collections::{HashMap, HashSet};

use hash::calculate_hash;
use image::{imageops::{overlay, rotate180, rotate270, rotate90}, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use stamp::*;

mod stamp;
mod hash;
const TILE_SIZE : u32 = 16;

struct ImagePair {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    adjacent_tiles: [Option<bool>; 8]
}

enum TileIndex {
    Top,
    InnerCorner,
    SideCorner,
    OuterCorner,
    Back
}

pub fn generate() {
    if TILE_SIZE % 2 != 0 { panic!("Tile size must be divisible by 2"); }
    let img = image::open("input.png").expect("failed to load input image");

    // Note - could simplify adjacent tile code by just cycling the array each new image pair
    //      - since there is a clear pattern
    //      - but to hell with that, this works.
    let mut image_pairs = vec![];
    image_pairs.append(&mut generate_inner_corner_pairs(&img));
    image_pairs.append(&mut generate_side_corner_pairs(&img));
    image_pairs.append(&mut generate_outer_corner_pairs(&img));
    image_pairs.push(generate_back_pair(&img));

    let tiles = generate_tiles_from_image_pairs(&img, &image_pairs);
    save_tiles_to_output(&tiles);
}

fn save_tiles_to_output(tiles: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>) {
    let size = (tiles.len() as f32).sqrt().ceil() as u32;
    
    let mut img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(size * TILE_SIZE, size * TILE_SIZE);

    for i in 0..tiles.len() {
        let x_index : u32 = i.clone() as u32 % size;
        let y_index : u32 = i.clone() as u32 / size;
        img_buffer.copy_from(&tiles[i], x_index * TILE_SIZE, y_index * TILE_SIZE).unwrap();
    }

    img_buffer.save("output.png").expect("failed to save output");
}

fn generate_tiles_from_image_pairs(img: &DynamicImage, image_pairs: &Vec<ImagePair>) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {

    let mut tiles: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = vec![];
    let mut atlas_indices: [usize; 256] = [0; 256];
    let mut seen_tiles = HashMap::new();


    for bitmask in 0..=u8::MAX {

        let mut current_tile = copy_pixels(img, TILE_SIZE * TileIndex::Back as u32, 0, TILE_SIZE, TILE_SIZE);

        for image_pair in image_pairs {
            if fits_bitmask(bitmask, image_pair) {
                overlay(&mut current_tile, &image_pair.buffer, 0, 0);
            }
        }
        if bitmask & 1 == 0 {
            // There is no tile above this, therefore top layer added.
            overlay(&mut current_tile, &copy_pixels(img, TILE_SIZE * TileIndex::Top as u32, 0, TILE_SIZE, TILE_SIZE), 0, 0);
        }

        let current_tile_hash = calculate_hash(&current_tile);
        if !seen_tiles.contains_key(&current_tile_hash) {
            tiles.push(current_tile);
            seen_tiles.insert(current_tile_hash, tiles.len() - 1);
        }
        atlas_indices[bitmask as usize] = *seen_tiles.get(&current_tile_hash).unwrap();
    }

    //yes this is horrible, but man...it works.
    print!("[");
    for index in atlas_indices {
        print!("{}, ", index);
    }
    print!("]");

    return tiles;
}

fn fits_bitmask(bitmask: u8, image_pair: &ImagePair) -> bool {

    let mut current_shift: u8 = 0;
    for opt_adjacent_tile in image_pair.adjacent_tiles {
        if let Some(adjacent_tile) = opt_adjacent_tile {
            if adjacent_tile == true && bitmask & (1 << current_shift) == 0 {
                return false;
            }
            else if adjacent_tile == false && bitmask & (1 << current_shift) != 0 {
                return false;
            }
        }
        current_shift += 1;
    }
    return true;
}

fn generate_back_pair(img: &DynamicImage) -> ImagePair {
    return ImagePair {
        buffer: copy_pixels(img, TILE_SIZE * TileIndex::Back as u32, 0, TILE_SIZE, TILE_SIZE),
        adjacent_tiles: [true.into(), true.into(), true.into(), true.into(), true.into(), true.into(), true.into(), true.into()],
    };

}

fn generate_inner_corner_pairs(img: &DynamicImage) -> Vec<ImagePair>{

    let inner_corner = copy_pixels(img, TILE_SIZE * TileIndex::InnerCorner as u32, 0, TILE_SIZE / 2, TILE_SIZE / 2);
    let empty_tile_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(TILE_SIZE, TILE_SIZE);

    return vec![
        ImagePair { 
            buffer: stamp_top_left(empty_tile_buffer.clone(), &inner_corner), 
            adjacent_tiles: [true.into(), None, None, None, None, None, true.into(), false.into()]
        },
        ImagePair { 
            buffer: stamp_top_right(empty_tile_buffer.clone(), &rotate90(&inner_corner)), 
            adjacent_tiles: [true.into(), false.into(), true.into(), None, None, None, None, None] 
        },
        ImagePair { 
            buffer: stamp_bottom_right(empty_tile_buffer.clone(), &rotate180(&inner_corner)), 
            adjacent_tiles: [None, None, true.into(), false.into(), true.into(), None, None, None] 
        },
        ImagePair { 
            buffer: stamp_bottom_left(empty_tile_buffer.clone(), &rotate270(&inner_corner)), 
            adjacent_tiles: [None, None, None, None, true.into(), false.into(), true.into(), None] 
        }
    ];
}

fn generate_side_corner_pairs(img: &DynamicImage) -> Vec<ImagePair>{

    let side_corner = copy_pixels(img, TILE_SIZE * TileIndex::SideCorner as u32, 0, TILE_SIZE / 2, TILE_SIZE / 2);
    let empty_tile_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(TILE_SIZE, TILE_SIZE);

    return vec![
        ImagePair { 
            buffer: stamp_top_left(empty_tile_buffer.clone(), &side_corner), 
            adjacent_tiles: [true.into(), None, None, None, None, None, false.into(), None]
        },
        ImagePair { 
            buffer: stamp_top_left(empty_tile_buffer.clone(), &rotate90(&side_corner)), 
            adjacent_tiles: [false.into(), None, None, None, None, None, true.into(), None]
        },
        ImagePair { 
            buffer: stamp_top_right(empty_tile_buffer.clone(), &rotate90(&side_corner)), 
            adjacent_tiles: [false.into(), None, true.into(), None, None, None, None, None]
        },
        ImagePair { 
            buffer: stamp_top_right(empty_tile_buffer.clone(), &rotate180(&side_corner)), 
            adjacent_tiles: [true.into(), None, false.into(), None, None, None, None, None]
        },
        ImagePair { 
            buffer: stamp_bottom_right(empty_tile_buffer.clone(), &rotate180(&side_corner)), 
            adjacent_tiles: [None, None, false.into(), None, true.into(), None, None, None]
        },
        ImagePair { 
            buffer: stamp_bottom_right(empty_tile_buffer.clone(), &rotate270(&side_corner)), 
            adjacent_tiles: [None, None, true.into(), None, false.into(), None, None, None]
        },
        ImagePair { 
            buffer: stamp_bottom_left(empty_tile_buffer.clone(), &rotate270(&side_corner)), 
            adjacent_tiles: [None, None, None, None, false.into(), None, true.into(), None]
        },
        ImagePair { 
            buffer: stamp_bottom_left(empty_tile_buffer.clone(), &side_corner), 
            adjacent_tiles: [None, None, None, None, true.into(), None, false.into(), None]
        },
    ];
}

fn generate_outer_corner_pairs(img: &DynamicImage) -> Vec<ImagePair>{

    let outer_corner = copy_pixels(img, TILE_SIZE * TileIndex::OuterCorner as u32, 0, TILE_SIZE / 2, TILE_SIZE / 2);
    let empty_tile_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(TILE_SIZE, TILE_SIZE);

    return vec![
        ImagePair { 
            buffer: stamp_top_left(empty_tile_buffer.clone(), &outer_corner), 
            adjacent_tiles: [false.into(), None, None, None, None, None, false.into(), None]
        },
        ImagePair { 
            buffer: stamp_top_right(empty_tile_buffer.clone(), &rotate90(&outer_corner)), 
            adjacent_tiles: [false.into(), None, false.into(), None, None, None, None, None]
        },
        ImagePair { 
            buffer: stamp_bottom_right(empty_tile_buffer.clone(), &rotate180(&outer_corner)), 
            adjacent_tiles: [None, None, false.into(), None, false.into(), None, None, None]
        },
        ImagePair { 
            buffer: stamp_bottom_left(empty_tile_buffer.clone(), &rotate270(&outer_corner)), 
            adjacent_tiles: [None, None, None, None, false.into(), None, false.into(), None]
        },
    ];
}

fn copy_pixels(source_img: &DynamicImage, start_x: u32, start_y: u32, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

    let mut img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = source_img.get_pixel(start_x + x, start_y + y);
            img_buffer.put_pixel(x, y, pixel)
        }
    }

    return img_buffer;
}

