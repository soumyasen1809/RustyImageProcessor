#![allow(clippy::ptr_arg)]

use std::{
    fmt::Debug,
    hash::Hash,
    path::{Path, PathBuf},
};

use image::{GenericImageView, ImageReader, Pixel};
use tokio::fs;

use crate::utils::{
    image_io::{image_reader, image_writer},
    statistics::{compute_histogram, compute_mean, compute_variance, print_histogram},
};

use super::image::Images;

pub trait Operation<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T>;
}

pub async fn process_images<T>(
    is_dir: bool,
    dir: Option<&str>,
    dir_out: &str,
    path: Option<&str>,
    operations: &Vec<Box<dyn Operation<T>>>,
    print_stats: bool,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Copy
        + Clone
        + From<u8>
        + std::cmp::PartialEq
        + Sync
        + Send
        + Into<u8>
        + Into<f64>
        + Hash
        + Eq
        + Debug
        + Ord,
{
    match is_dir {
        true => {
            if dir.is_none() {
                return Err("DIR can not be None for directory scan".into());
            }
            let mut dir_entries = fs::read_dir(dir.unwrap()).await?;
            while let Some(entry) = dir_entries.next_entry().await? {
                let img_path = entry.path();
                let image = ImageReader::open(img_path.clone())?.decode()?;

                println!(
                    "Last pixel in image is: {:?}",
                    image
                        .get_pixel(image.width() - 1, image.height() - 1)
                        .channels()
                        .get(2)
                );

                let final_image =
                    computation_image_processing(img_path.clone(), operations).await?;

                if print_stats {
                    print_statistics(&final_image).await;
                }

                let mut out_file_name = Path::new(img_path.to_str().unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("\"", "");

                out_file_name = String::from(dir_out) + &out_file_name + &String::from(".png");
                println!("Writing to: {:?}", out_file_name);
                image_writer((out_file_name).as_str(), &final_image)?;
            }
        }
        false => {
            if path.is_none() {
                return Err("PATH can not be None for single image scan".into());
            }
            let image = ImageReader::open(path.unwrap())?.decode()?;

            println!(
                "Last pixel in image is: {:?}",
                image
                    .get_pixel(image.width() - 1, image.height() - 1)
                    .channels()
                    .get(2)
            );

            let final_image =
                computation_image_processing(PathBuf::from(path.unwrap()), operations).await?;

            if print_stats {
                print_statistics(&final_image).await;
            }

            let mut out_file_name = Path::new(path.unwrap())
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replace("\"", "");
            out_file_name = String::from(dir_out) + &out_file_name + &String::from(".png");
            println!("Writing to: {:?}", out_file_name);
            image_writer(&out_file_name, &final_image)?;
        }
    }

    Ok(())
}

pub async fn computation_image_processing<T>(
    img_path: PathBuf,
    operations: &Vec<Box<dyn Operation<T>>>,
) -> Result<Images<T>, Box<dyn std::error::Error>>
where
    T: Copy
        + Clone
        + From<u8>
        + std::cmp::PartialEq
        + Send
        + Sync
        + Into<u8>
        + Into<f64>
        + Hash
        + Eq
        + Debug
        + Ord,
{
    let image_read: Images<T> = image_reader(img_path.to_str().unwrap()).await?;

    let mut new_image: Images<T> = Images::new(
        image_read.get_width(),
        image_read.get_height(),
        image_read.get_channels(),
        image_read.get_image(),
    );

    for ops in operations.iter() {
        new_image = ops.apply(&new_image);
    }

    Ok(new_image)
}

pub async fn print_statistics<T>(image: &Images<T>)
where
    T: Copy
        + Clone
        + From<u8>
        + std::cmp::PartialEq
        + Send
        + Sync
        + Into<f64>
        + Hash
        + Eq
        + Debug
        + Ord,
{
    let histogram_stats = compute_histogram(image);
    print_histogram(histogram_stats);

    let mean = compute_mean(image);
    println!("Mean for the resized image: {:?}", mean);

    let variance = compute_variance(image);
    println!("Variance for the resized image: {:?}", variance);
}
