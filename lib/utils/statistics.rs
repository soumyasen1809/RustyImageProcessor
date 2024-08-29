use std::collections::HashMap;

use crate::core::image::Images;

fn sort_and_print_map(input_map: &HashMap<u8, u32>) {
    let mut input_vec: Vec<(&u8, &u32)> = input_map.iter().collect();
    input_vec.sort_by(|a, b| a.0.cmp(&b.0));
    let max_val = input_vec.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    println!("Intensity \t Count");
    for val in input_vec.iter() {
        print!("{:?}\t", val.0);
        for _ in 0..((*val.1 as f64 / *max_val as f64) * 40.0) as usize {
            print!("█");
        }
        print!("  [{:?}]\n", val.1);
    }
}

///In an image statistics histogram, we plot the distribution of pixel intensity values
/// X-Axis: Represents the intensity values (0 to 255).
/// Y-Axis: Represents the frequency of pixels with that intensity.
pub fn compute_histogram(image: &Images) -> Vec<HashMap<u8, u32>> {
    let mut red_histogram_map: HashMap<u8, u32> = HashMap::new();
    let mut green_histogram_map: HashMap<u8, u32> = HashMap::new();
    let mut blue_histogram_map: HashMap<u8, u32> = HashMap::new();

    for index in 0..=255 {
        red_histogram_map.insert(index, 0);
        green_histogram_map.insert(index, 0);
        blue_histogram_map.insert(index, 0);
    }

    for pix in image.get_image().iter() {
        let red_channel = pix.get_red();
        let green_channel = pix.get_green();
        let blue_channel = pix.get_blue();

        *red_histogram_map.entry(red_channel).or_insert(0) += 1;
        *green_histogram_map.entry(green_channel).or_insert(0) += 1;
        *blue_histogram_map.entry(blue_channel).or_insert(0) += 1;
    }

    vec![red_histogram_map, green_histogram_map, blue_histogram_map]
}

pub fn print_histogram(histogram_map: Vec<HashMap<u8, u32>>) {
    for (index, color_map) in histogram_map.iter().enumerate() {
        match index {
            0 => {
                println!("Red histogram!");
                println!("{:?}", sort_and_print_map(color_map));
            }
            1 => {
                println!("Green histogram!");
                println!("{:?}", sort_and_print_map(color_map));
            }
            2 => {
                println!("Blue histogram!");
                println!("{:?}", sort_and_print_map(color_map));
            }
            _ => {
                println!("Wrong index. Histogram does not exist")
            }
        }
    }
}

pub fn compute_variance(image: &Images) -> (f64, f64, f64, f64) {
    let mut variance = (0.0, 0.0, 0.0, 0.0);
    let mut mean = (0.0, 0.0, 0.0, 0.0);
    for pix in image.get_image() {
        mean.0 += pix.get_red() as f64;
        mean.1 += pix.get_green() as f64;
        mean.2 += pix.get_blue() as f64;
        mean.3 += pix.get_alpha() as f64;
    }
    mean.0 /= image.get_image().len() as f64;
    mean.1 /= image.get_image().len() as f64;
    mean.2 /= image.get_image().len() as f64;
    mean.3 /= image.get_image().len() as f64;

    for pix in image.get_image() {
        variance.0 += (pix.get_red() as f64 - mean.0).powi(2);
        variance.1 += (pix.get_green() as f64 - mean.1).powi(2);
        variance.2 += (pix.get_blue() as f64 - mean.2).powi(2);
        variance.3 += (pix.get_alpha() as f64 - mean.3).powi(2);
    }
    variance.0 /= image.get_image().len() as f64;
    variance.1 /= image.get_image().len() as f64;
    variance.2 /= image.get_image().len() as f64;
    variance.3 /= image.get_image().len() as f64;

    variance
}
