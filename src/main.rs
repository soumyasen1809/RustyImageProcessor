use image_processor::core::operations::process_images;

const DIR: &str = "assets/";
const DIR_OUT: &str = "assets_out/";
const PATH: &str = "assets/lenna.png";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    process_images(false, None, DIR_OUT, Some(PATH)).await?;

    Ok(())
}
