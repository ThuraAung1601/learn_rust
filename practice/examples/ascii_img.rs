fn vflip(img: &[String]) -> Vec<String> {
    let mut flipped = Vec::new();
    for i in (0..img.len()).rev() {
        let line = format!("{}", img[i]);
        flipped.push(line);
    }
    flipped
}

fn _vflip(img: &[String]) -> Vec<String> {
    let mut flipped = Vec::new();
    for l in img.iter().rev() {
        let line = format!("{}", l);
        flipped.push(line);
    }
    flipped
}

fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut concatenated = Vec::new();
    for i in img1 {
        let line = format!("{}", i);
        concatenated.push(line);
    }
    for j in img2 {
        let line = format!("{}", j);
        concatenated.push(line);
    }
    concatenated
}

fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut concatenated = Vec::new();
    let maxi_len = img1.iter().map(|s| s.len()).max().unwrap_or(0);
    for (i, j) in img1.iter().zip(img2.iter()) {
        let space = " ".repeat(maxi_len - i.len());
        let line = format!("{}{}{}", i, space, j);
        concatenated.push(line);
    }
    if img1.len() > img2.len() {
        let diff = img1.len() - img2.len();
        for i in (0..diff).rev() {
            let line = format!("{}", img1[img1.len()-(i+1)]);
            concatenated.push(line);
        }
    }
    else if img1.len() < img2.len() {
        let diff = img2.len() - img1.len();
        for i in (0..diff).rev() {
            let space = " ".repeat(maxi_len);
            let line = format!("{}{}", space, img2[img2.len()-(i+1)]);
            concatenated.push(line);
        }
    }
    concatenated
}

fn max_len(img: &[String]) -> usize {
    let mut maxi: usize;
    if img.len() == 0 {
        maxi = 0;
    }
    else {
        maxi = img[0].len();
        for i in img {
            if maxi < i.len() {
                maxi = i.len();
            }
        }
    }
    maxi
}

fn hflip(img: &[String]) -> Vec<String> {
    let maxi_len = img.iter().map(|s| s.len()).max().unwrap_or(0);
    // let maxi_len2 = max_len(&img);
    let mut hflipped = Vec::new();
    for line in img {
        let space = " ".repeat(maxi_len - line.len());
        let mut reversed = String::from(space);
        for pixel in line.chars().rev() {
            reversed.push(pixel);
        }

        hflipped.push(reversed);
    }
    hflipped
}

fn main() {
    let data = [
        "<--",
        "######",
        "<=="
        ].map(|v| v.to_string());

    let vflipped = vflip(&data);
    println!("{:?}", vflipped);

    let hflipped = hflip(&data);
    println!("{:?}", hflipped);

    println!("{:?}", vcat(&data, &vflipped));
    println!("{:?}", hcat(&data[..2], &data));

    let data1 = [
        "<--",
        "######",
        "<==",
        "^^^^^^^",
        "@@@@@"
        ].map(|v| v.to_string());

    let data2 = [
        "<--",
        "######",
        "<=="
        ].map(|v| v.to_string());

    println!("{:?}", hcat(&data1, &data2));
    println!("{:?}", hcat(&data2, &data1));
}

// Test for No.1
#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);

    let data = [
        "<--",
        "######",
        "<=="
        ].map(|v| v.to_string());

    assert_eq!(vflip(&data), [
        "<==",
        "######",
        "<--"]);

    assert_eq!(hflip(&data), [
        "   --<",
        "######",
        "   ==<"]);
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = [
        "<--",
        "#####",
        "<=="
        ].map(|v| v.to_string());

    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(vcat(&data, &data), [
                                    "<--",
                                    "#####",
                                    "<==",
                                    "<--",
                                    "#####",
                                    "<=="
                                ]);
    assert_eq!(hcat(&data, &data[..2]), [
                                    "<--  <--",
                                    "##########",
                                    "<=="]);

    assert_eq!(hcat(&data[..2], &data), [
                                    "<--  <--",
                                    "##########",
                                    "     <=="]);
}
