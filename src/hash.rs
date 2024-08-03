use std::hash::{DefaultHasher, Hash, Hasher};

use image::{ImageBuffer, Rgba};



pub fn calculate_hash(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> u64 {

    let mut hasher = DefaultHasher::new();
    buffer.pixels().for_each(|pixel| pixel.hash(&mut hasher));
    hasher.finish()
}