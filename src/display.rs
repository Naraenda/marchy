use palette::Hsv;
use palette::rgb::LinSrgb;
use ultraviolet::Vec3;

use std::io::Write;
use crossterm::{
    QueueableCommand,
    cursor,
    style::{ self, StyledContent }
};

#[allow(unused_must_use)]
pub fn draw_image(data: Vec<Vec3>, width: i32, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.queue(cursor::MoveTo(0, 0));
    let lines = data
        .into_iter()
        .map(|p| style::PrintStyledContent(color_to_styled(p)))
        .collect::<Vec<_>>();

    for row in lines.chunks(width as usize) {
        for x in row {
            stdout.queue(x);
        }
        stdout.queue(cursor::MoveToNextLine(0))?;
    }
    stdout.flush()?;
    Ok(())
}

/// Converts boring floats into cool ascii.
pub fn quantize_to_char(value: f32) -> char {
    if value > 0.9 { 
        '@'
    } else if value > 0.80 {
        '$'
    } else if value > 0.70 {
        '#'
    } else if value > 0.60 {
        '+'
    } else if value > 0.30 {
        '*'
    } else if value > 0.15 {
        '/'
    } else if value > 0.06 {
        ':'
    } else if value > 0.03 {
        ','
    } else if value > 0.00 {
        '.'
    } else { 
        ' '
    }
}

/// Full RGB, not all terminals support this
pub fn color_to_styled(color: Vec3) -> StyledContent<String> {
    let c_rgb = LinSrgb::new(color.x, color.y, color.z);
    let mut c_hsl = Hsv::from(c_rgb);
    let value = c_hsl.value;
    c_hsl.value = c_hsl.value * 0.25 + 0.75;
    let c_rgb = LinSrgb::from(c_hsl);
    let (r, g, b): (u8, u8, u8) = c_rgb.into_format::<u8>().into_components();

    style::style(quantize_to_char(value).to_string())
        .with(style::Color::Rgb { r: r, g: g, b: b })
}
