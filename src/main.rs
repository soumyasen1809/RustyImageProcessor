use image_processor::{
    core::operations::{process_images, Operation},
    filters::{
        blur::{Blur, SmoothingKernelChoices},
        edge_detection::{EdgeDetectingKernelChoices, EdgeDetection},
        gamma_correction::GammaCorrection,
        gray_scale::{GrayScale, GrayScaleAlgorithms},
        morphological::{Erosion, MorphologicalKernelChoices},
        sharpen::{Sharpen, SharpeningKernelChoices},
    },
    transformations::{
        crop::Crop,
        resize::ResizeBilinearInterpolation,
        rotate::{Flip90Right, FlipHorizontal},
    },
};

const IS_DIR: bool = false;
const DIR: &str = "assets/";
const DIR_OUT: &str = "assets_out/";
const PATH: &str = "assets/lenna.png";
const PRINT_STATS: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operations: Vec<Box<dyn Operation<u8>>> = vec![
        Box::new(FlipHorizontal::new()),
        Box::new(Flip90Right::new()),
        Box::new(ResizeBilinearInterpolation::new(256, 256)),
        Box::new(GrayScale::new(GrayScaleAlgorithms::Luminosity)),
        Box::new(Blur::new(SmoothingKernelChoices::Gaussian)),
        Box::new(Crop::new((50, 50), 128, 128)),
        Box::new(EdgeDetection::new(EdgeDetectingKernelChoices::Emboss)),
        Box::new(Sharpen::new(SharpeningKernelChoices::EdgeEnhancement)),
        Box::new(Erosion::new(MorphologicalKernelChoices::Diamond)),
        Box::new(GammaCorrection::new(1.5)),
    ];

    process_images(
        IS_DIR,
        Some(DIR),
        DIR_OUT,
        Some(PATH),
        &operations,
        PRINT_STATS,
    )
    .await?;

    Ok(())
}
