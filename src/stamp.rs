use image::{imageops::overlay, ImageBuffer, Rgba};

pub fn stamp_bottom_left(mut target: ImageBuffer<Rgba<u8>, Vec<u8>>, stamp: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    
    if target.width() < stamp.width() || target.height() < stamp.height() { 
        panic!("Stamp larger than stamp target"); 
    }
    let x_offset = 0;
    let y_offset = target.height() - stamp.height();
    stamp_with_offset(&mut target, stamp, x_offset, y_offset);
    return target;
}
pub fn stamp_top_left(mut target: ImageBuffer<Rgba<u8>, Vec<u8>>, stamp: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    
    if target.width() < stamp.width() || target.height() < stamp.height() { 
        panic!("Stamp larger than stamp target"); 
    }
    let x_offset = 0;
    let y_offset = 0;
    stamp_with_offset(&mut target, stamp, x_offset, y_offset);
    return target;
}
pub fn stamp_bottom_right(mut target: ImageBuffer<Rgba<u8>, Vec<u8>>, stamp: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    
    if target.width() < stamp.width() || target.height() < stamp.height() { 
        panic!("Stamp larger than stamp target"); 
    }
    let x_offset = target.width() - stamp.width();
    let y_offset = target.height() - stamp.height();
    stamp_with_offset(&mut target, stamp, x_offset, y_offset);
    return target;
}
pub fn stamp_top_right(mut target: ImageBuffer<Rgba<u8>, Vec<u8>>, stamp: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    
    if target.width() < stamp.width() || target.height() < stamp.height() { 
        panic!("Stamp larger than stamp target"); 
    }
    let x_offset = target.width() - stamp.width();
    let y_offset = 0;
    stamp_with_offset(&mut target, stamp, x_offset, y_offset);
    return target;
}

fn stamp_with_offset(target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, stamp: &ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: u32, y_offset: u32) {
    overlay(target, stamp, x_offset as i64, y_offset as i64);
}