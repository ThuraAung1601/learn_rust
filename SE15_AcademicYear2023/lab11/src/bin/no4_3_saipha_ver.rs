use std::fs::File;
use std::io::{BufRead, BufReader, Write};

struct XPM2Image {
    width: usize,
    height: usize,
    colors: Vec<(String, String)>,
    pixels: Vec<String>,
}

fn read_xpm2<R: BufRead>(reader: R) -> XPM2Image {
    let mut lines = reader.lines().map(|l| l.unwrap());
    let header = lines.next().unwrap();
    if !header.starts_with("! XPM2") {
        panic!("Invalid XPM2 file format");
    }

    let dimensions: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let width = dimensions[0];
    let height = dimensions[1];
    let num_colors = dimensions[2];

    let mut colors = Vec::new();
    for _ in 0..num_colors {
        let color_line = lines.next().unwrap();
        let parts: Vec<&str> = color_line.split_whitespace().collect();
        if parts.len() == 3 {
            let symbol = parts[0].to_string();
            let color = parts[2].to_string();
            colors.push((symbol, color));
        } else {
            panic!("Invalid color definition");
        }
    }

    let pixels: Vec<String> = lines.collect();

    XPM2Image {
        width,
        height,
        colors,
        pixels,
    }
}

fn generate_svg_from_xpm2(xpm2: &XPM2Image, scale: usize) -> String {
    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
        xpm2.width * scale,
        xpm2.height * scale
    ));

    svg.push_str(r#"<style type="text/css">rect { stroke: #000; }"#);

    for (index, (_, color)) in xpm2.colors.iter().enumerate() {
        svg.push_str(&format!(
            r#"rect.c{} {{ fill: {}; }}"#,
            index + 1,
            color
        ));
    }

    svg.push_str(r#"</style>"#);

    for (y, row) in xpm2.pixels.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let color_class =
                format!(
                    "c{}",
                    xpm2.colors.iter().position(|(ch, _)| ch == &c.to_string()).unwrap() + 1
                );
            let x_pos = x * scale;
            let y_pos = y * scale;
            svg.push_str(&format!(
                r#"<rect class="{}" x="{}" y="{}" width="{}" height="{}" />"#,
                color_class, x_pos, y_pos, scale, scale
            ));
        }
    }

    svg.push_str("</svg>");
    svg
}

fn main() {
    let file_path = "test4_3.xpm";
    let scale = 100;

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        let xpm2 = read_xpm2(reader);
        let svg = generate_svg_from_xpm2(&xpm2, scale);

        let mut myfile = File::create("./test.svg").unwrap();
        myfile.write_all(svg.as_bytes()).unwrap();
        // println!("{}", svg);
    } else {
        eprintln!("Failed to open the file: {}", file_path);
    }
}
