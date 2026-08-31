use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{Result, Write};

pub fn save_bmp(framebuffer: &Framebuffer, path: &str) -> Result<()> {
    let width = framebuffer.width;
    let height = framebuffer.height;

    let row_size = (width * 3 + 3) & !3;
    let pixel_data_size = row_size * height;
    let file_size = 54 + pixel_data_size;

    let mut file = File::create(path)?;

    // BMP Header
    file.write_all(b"BM")?;
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&(54u32).to_le_bytes())?;

    // DIB Header
    file.write_all(&(40u32).to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(height as i32).to_le_bytes())?;
    file.write_all(&(1u16).to_le_bytes())?;
    file.write_all(&(24u16).to_le_bytes())?;
    file.write_all(&[0; 24])?;

    let padding = vec![0u8; row_size - width * 3];

    // BMP guarda las filas de abajo hacia arriba
    for y in (0..height).rev() {
        for x in 0..width {
            let color = framebuffer.buffer[y * width + x];

            let r = ((color >> 16) & 255) as u8;
            let g = ((color >> 8) & 255) as u8;
            let b = (color & 255) as u8;

            // BMP utiliza BGR
            file.write_all(&[b, g, r])?;
        }

        file.write_all(&padding)?;
    }

    Ok(())
}
