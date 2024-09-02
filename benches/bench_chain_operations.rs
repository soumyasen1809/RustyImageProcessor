use criterion::async_executor::FuturesExecutor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

async fn async_function() {
    // Code you want to benchmark
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

    let _result = black_box(
        process_images(
            false,
            None,
            "assets_out/",
            Some("assets/lenna.png"),
            &operations,
            false,
        )
        .await,
    );
}

fn bench_async_chain_function(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_group");
    group.bench_function("chain_fn_async", |b| {
        b.to_async(FuturesExecutor).iter(|| async_function());
    });
    group.finish();
}

criterion_group!(benches, bench_async_chain_function);
criterion_main!(benches);
