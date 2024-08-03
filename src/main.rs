use std::{collections::HashSet, hash::{DefaultHasher, Hash, Hasher}};

use image::{imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};


const TILE_SIZE: u32 = 16;
const IMAGE_TILE_SIZE: u32 = 10;

fn main() {

    let img = image::open("input.png")
        .expect("failed to load input image");


    let tiles = get_tiles(&img);
    let size = (tiles.len() as f32).sqrt().ceil() as u32;
    
    let mut img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(size * TILE_SIZE, size * TILE_SIZE);

    for i in 0..tiles.len() {
        let x_index : u32 = i.clone() as u32 % size;
        let y_index : u32 = i.clone() as u32 / size;
        img_buffer.copy_from(&tiles[i], x_index * TILE_SIZE, y_index * TILE_SIZE).unwrap();
    }

    img_buffer.save("output.png").expect("failed to save output");

}


fn get_tiles(img: &DynamicImage) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {

    let front_layer_indexes: Vec<u8> = (20..21).collect();
    let mid_layer_indexes: Vec<u8> = (0..14).collect();
    let back_layer_indexes: Vec<u8> = (21..22).collect();
    let mut all_tiles = Vec::new();


    let front_tiles = get_tiles_by_index(img, &front_layer_indexes);
    let middle_tiles = get_tiles_by_index(img, &mid_layer_indexes);
    let back_tiles = get_tiles_by_index(img, &back_layer_indexes);

    all_tiles.append(&mut back_tiles.clone());
    //all_tiles.append(&mut overlay_tiles(&back_tiles, &front_tiles));

    let mut back_middle_tiles = overlay_tiles(&back_tiles, &middle_tiles);
    back_middle_tiles = get_rotated_permutations(&back_middle_tiles);
    let mut back_middle_front_tiles = overlay_tiles(&back_middle_tiles, &front_tiles);

    all_tiles.append(&mut back_middle_tiles);
    all_tiles.append(&mut back_middle_front_tiles);
    //let all_distinct_tiles = distinct_image_buffers(all_tiles);

    return all_tiles;
}

fn overlay_tiles(back_tiles: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, front_tiles: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut tiles = Vec::new();

    for back_tile in back_tiles {
        for front_tile in front_tiles {
            let mut tile = back_tile.clone();
            imageops::overlay(&mut tile, front_tile, 0, 0);
            tiles.push(tile);
        }
    }
    return tiles;
}

fn distinct_image_buffers(buffers: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut unique_buffers: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = Vec::new();
    let mut seen_hashes = HashSet::new();

    for buffer in buffers {
        let hash = calculate_hash(&buffer);
        if seen_hashes.insert(hash) {
            unique_buffers.push(buffer);
        }
    }

    unique_buffers
}

fn calculate_hash(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> u64 {

    let mut hasher = DefaultHasher::new();
    buffer.pixels().for_each(|pixel| pixel.hash(&mut hasher));
    hasher.finish()
}
fn get_tiles_by_index(img: &DynamicImage, indices: &Vec<u8>) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {

    let mut tiles = Vec::new();

    for i in indices {
        let x_start_index : u32 = i.clone() as u32 % IMAGE_TILE_SIZE;
        let y_start_index : u32 = i.clone() as u32 / IMAGE_TILE_SIZE;
        let x_start_px = x_start_index * TILE_SIZE;
        let y_start_px = y_start_index * TILE_SIZE;

        let mut tile: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(TILE_SIZE, TILE_SIZE);
        for ty in 0..TILE_SIZE {
            for tx in 0..TILE_SIZE {
                let pixel = img.get_pixel(x_start_px + tx, y_start_px + ty);
                tile.put_pixel(tx, ty, pixel)
            }
        }
        tiles.push(tile);
    }

    return tiles;

}

fn get_rotated_permutations(tiles: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut all_tiles = Vec::new();
    
    for tile in tiles {
        all_tiles.push(tile.clone());
        all_tiles.push(image::imageops::rotate90(tile));
        all_tiles.push(image::imageops::rotate180(tile));
        all_tiles.push(image::imageops::rotate270(tile));
    }
    

    return all_tiles;
}



