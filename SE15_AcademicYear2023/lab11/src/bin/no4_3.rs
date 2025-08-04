use std::io::{BufRead, BufReader, Read};
use std::error::Error;

struct XPMImage {
    colors: Vec<(String, String)>,
    pixels: Vec<String>,
}

impl XPMImage {
    fn new(ctable: &[(String, String)], rows: &[String]) -> Self {
        let colors: Vec<(String, String)> = ctable.iter().map(|(symbol, color)| (symbol.clone(), color.clone())).collect();
        let pixels: Vec<String> = rows.iter().map(|row| row.to_string()).collect();

        XPMImage { colors, pixels }
    }
}

fn read_xpm2<R: Read>(reader: &mut R) -> Result<XPMImage, Box<dyn Error>> {
    let mut lines = BufReader::new(reader).lines();
    let mut colors: Vec<(String, String)> = Vec::new();
    let mut pixels: Vec<String> = Vec::new();

    // Read XPM header
    let _ = lines.next(); // Skip the "! XPM2" line

    // Read dimensions and color table size
    let dimensions_line = lines.next().ok_or("Missing dimensions line")??;
    let dimensions: Vec<_> = dimensions_line.split_whitespace().collect();
    let width = dimensions[0].parse::<usize>()?;
    let height = dimensions[1].parse::<usize>()?;
    let color_table_size = dimensions[2].parse::<usize>()?;

    // Read color table
    for _ in 0..color_table_size {
        let color_line = lines.next().ok_or("Missing color line")??;
        let color_parts: Vec<_> = color_line.split_whitespace().collect();
        let symbol = color_parts[0].to_string();
        let hex_color = color_parts[2].to_string();
        colors.push((symbol, hex_color));
    }

    // Read pixel data
    for _ in 0..height {
        let pixel_line = lines.next().ok_or("Missing pixel line")??;
        pixels.push(pixel_line.to_string());
    }

    Ok(XPMImage::new(&colors, &pixels))
}

fn generate_svg(image: &XPMImage, scale_factor: u32) -> String {
    let mut svg = String::new();

    // SVG header
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&(image.pixels[0].len() as u32 * scale_factor).to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&(image.pixels.len() as u32 * scale_factor).to_string());
    svg.push_str("\">\n");

    // SVG style definitions
    svg.push_str("<style type=\"text/css\">\n");
    for (index, color) in image.colors.iter().enumerate() {
        svg.push_str(&format!(".c{} {{ fill: {}; }}\n", index, color.1));
    }
    svg.push_str("</style>\n");

    // SVG rectangles
    for (y, row) in image.pixels.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if let Some(color_index) = image.colors.iter().position(|(symbol, _)| symbol.chars().next().unwrap() == ch) {
                let x_scaled = x as u32 * scale_factor;
                let y_scaled = y as u32 * scale_factor;
                svg.push_str(&format!("<rect class=\"c{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" />\n", color_index, x_scaled, y_scaled, scale_factor, scale_factor));
            }
        }
    }

    // Closing SVG tag
    svg.push_str("</svg>");

    svg
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "test4_3.xpm"; // Replace with your XPM file path.
    let file = std::fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let image = read_xpm2(&mut reader)?;

    // Generate SVG from the image (as shown in previous responses)
    let scale_factor = 20;
    let svg = generate_svg(&image, scale_factor);

    println!("{}", svg);

    Ok(())
}
