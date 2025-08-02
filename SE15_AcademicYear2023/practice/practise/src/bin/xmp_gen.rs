#[derive(Debug)]
struct XPMImage {
    colors: Vec<(String, String)>,
    pixels: Vec<String>,
}

fn make_xpm2(ctable: &[(String, String)], rows: &[&str]) -> XPMImage {
    let colors: Vec<(String, String)> = ctable.iter().map(|(symbol, color)| (symbol.clone(), color.clone())).collect();
    let pixels: Vec<String> = rows.iter().map(|&row| row.to_string()).collect();

    XPMImage { colors, pixels }
}

fn maxi_len(img: &[String]) -> usize {
    let mut max = 0;
    for _i in img {
        let length = _i.len();
        if length > max {
            max = length;
        }
    }
    return max;
}

fn _make_xpm2(ctable: &[(String, String)], rows: &[String]) -> String {
    let mut result = String::from("\
                            ! XPM2\n");
    let height = rows.len();
    let width = maxi_len(rows);
    let n_color = ctable.len();
    let per_pixel = 1;

    let info = format!("{} {} {} {}\n", height, width, n_color, per_pixel);
    result.push_str(&info);

    for _c in ctable {
        let character = &_c.0;
        let color = &_c.1;
        let color_info = format!("{} c {}\n", character, color);
        result.push_str(&color_info);
    }

    for _r in rows {
        let row = format!("{}\n", _r);
        result.push_str(&row);
    }

    result
}


fn main() {
    let ctable = &[
        ("#".into(), "#000000".into()),
        ("-".into(), "#FFFFFF".into()),
    ];
    let rows = ["##--", "##--", "--##", "--##"];
    let pixels: Vec<_> = rows.iter().map(|r| r.to_string()).collect();
    let img = _make_xpm2(ctable, &pixels);
    println!("{}", img);
}

#[test]
fn test_make_xpm2() {
    let ctable = &[
        ("#".into(), "#000000".into()),
        ("-".into(), "#FFFFFF".into()),
    ];
    let rows = ["##--", "##--", "--##", "--##"];
    let img = make_xpm2(ctable, &rows);

    assert_eq!(
        img.colors,
        [("#".into(), "#000000".into()), ("-".into(), "#FFFFFF".into())]
    );
    assert_eq!(img.pixels.len(), 4);
    assert_eq!(img.pixels.iter().map(|r| r.len()).max(), Some(4));
    assert_eq!(img.colors.len(), 2);
}
