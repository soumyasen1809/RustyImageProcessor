use image_processor::core::operations::process_images;

const DIR: &str = "assets/";
const DIR_OUT: &str = "assets_out/";
const PATH: &str = "assets/lenna.png";
// const OUT_PATH: &str = "assets_out/out_cropped.png";
const GAMMA: f64 = 0.5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    process_images(false, DIR, DIR_OUT, GAMMA, PATH).await?;

    Ok(())
}
