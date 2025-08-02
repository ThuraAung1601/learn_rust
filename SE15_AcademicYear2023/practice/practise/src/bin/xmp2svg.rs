struct XPMImage {
    colors: Vec<(String, String)>,
    pixels: Vec<String>,
}

impl XPMImage {
    fn new(ctable: &[(String, String)], rows: &[&str]) -> Self {
        let colors: Vec<(String, String)> = ctable.iter().map(|(symbol, color)| (symbol.clone(), color.clone())).collect();
        let pixels: Vec<String> = rows.iter().map(|&row| row.to_string()).collect();

        XPMImage { colors, pixels }
    }
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

fn main() {
    let ctable = &[
        ("#".into(), "#000000".into()),
        ("-".into(), "#808080".into())
    ];
    let rows = [
        "##--",
        "##--",
        "--##",
        "--##"
    ];

    let scale_factor = 100;
    let image = XPMImage::new(ctable, &rows);
    let svg = generate_svg(&image, scale_factor);

    println!("{}", svg);
}
