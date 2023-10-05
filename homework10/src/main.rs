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

// No.1: Write functions that flip lines of text
// No.1.1
fn vflip(img: &[String]) -> Vec<String> {
    let mut flipped = Vec::new();
    for _i in img.iter().rev() {
        // println!("{}", _i);
        let line = format!("{}", _i);
        flipped.push(line);
    }
    return flipped;
}

// No.1.2
fn hflip(img: &[String]) -> Vec<String> {
    let mut flipped = Vec::new();
    // let max_length = img.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_length = maxi_len(&img);
    for _i in img.iter() {
        let mut reversed = String::new();
        for _j in _i.chars().rev() {
        //  print!("{}", _j);
            reversed.push(_j);
        }
        // println!("{}", reversed);
        // let reversed = _i.chars().rev().collect();
        let repeated = " ".repeat(max_length - reversed.len());
        let line = format!("{}{}", repeated, reversed);
        flipped.push(line);
    }
    return flipped;
}

// No.2: Write functions that concatenate lines of text
// No.2.1
fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut vcat = Vec::new();
    for _i in img1 {
        // println!("{}", _i);
        let line = format!("{}", _i);
        vcat.push(line);
    }
    for _j in img2 {
        // println!("{}", _j);
        let line = format!("{}", _j);
        vcat.push(line);
    }
    return vcat;
}

// No.2.2
fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut hcat = Vec::new();
    // let max_img1 = img1.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_length = maxi_len(img1);
    // println!("{}", max_length);
    for (_i, _j) in img1.iter().zip(img2.iter()) {
        // println!("{}{}", _i, _j);
        let repeated = " ".repeat(max_length-_i.len());
        let line = format!("{}{}{}", _i, repeated, _j);
        hcat.push(line);
    }    
    if img1.len() > img2.len() {
        // println!("{}", img1[img1.len()-1]);
        let line = format!("{}", img1[img1.len()-1]);
        hcat.push(line);
    }
    else if img1.len() < img2.len() {
        // println!("{}{}", " ".repeat(img1[img1.len()-1].len()), img2[img2.len() -1]);
        let repeated = " ".repeat(max_length);
        // println!("{}", max_length);
        let line = format!("{}{}", repeated, img2[img2.len()-1]);
        hcat.push(line);
    }
    return hcat;
}

fn main() {
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    println!("{:?}", data);

    println!("vertical flip: {:?}", vflip(&data));
    println!("horizontal flip: {:?}", hflip(&data));

    println!("vertical concatenate: {:?}", vcat(&data, &data));
    println!("horizontal concatenate: {:?}", hcat(&data[..2], &data));
    println!("horizontal concatenate: {:?}", hcat(&data, &data[..2]));
}

// Output

// ["<--", 
// "#####", 
// "<=="]

// vertical flip: 
// ["<==", 
// "#####", 
// "<--"]

// horizontal flip: ["  --<", "#####", "  ==<"]

// vertical concatenate: 
// ["<--", 
// "#####", 
// "<==", 
// "<--", 
// "#####", 
// "<=="]

// horizontal concatenate: 
// ["<--  <--", 
// "##########", 
// "     <=="]

// horizontal concatenate: 
// ["<--  <--",
// "##########",
// "<=="]

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

// Test for No.2
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

// Test2 for No.2
#[test]
fn test_img_cat2() {
    let data1 = [
        "<--",
        "****",
        "<=="
        ].map(|v| v.to_string());

    let data2 = [
        "<--",
        "#####",
        "<=="
        ].map(|v| v.to_string());

    
    assert_eq!(vcat(&data1, &data2), [
                                    "<--",
                                    "****",
                                    "<==",
                                    "<--",
                                    "#####",
                                    "<=="
                                ]);
    assert_eq!(hcat(&data1, &data2), [
                                    "<-- <--",
                                    "****#####",
                                    "<== <=="]);

    assert_eq!(hcat(&data2, &data1), [
                                    "<--  <--",
                                    "#####****",
                                    "<==  <=="]);
}
