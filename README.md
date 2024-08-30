# Image Processing Library

## Overview
Implementation of common image processing algorithms optimized for performance using the `Rayon` library.
The library focuses on modifying images, such as resizing, cropping, rotating, filtering, or applying effects.
It supports various image formats like JPEG, PNG.
It tries to follow a well-structured design that allows for easy extension and customization.

```

image_processor/
├── Cargo.toml
├── src/
│   ├── main.rs              // Entry point for the binary
│   ├── lib.rs               // Main library file
│   ├── core/
│   │   ├── image.rs         // Image data structure
│   │   ├── pixel.rs         // Pixel data structure
│   │   └── <...>.rs         // Other modules
│   ├── filters/
│   │   ├── blur.rs          // Gaussian blur, etc.
│   │   ├── sharpen.rs
│   │   ├── edge_detection.rs // Outline, Sobel, etc.
│   │   └── <...>.rs          // Other modules
│   ├── transformations/
│   │   ├── resize.rs
│   │   ├── rotate.rs         // Flip operations
│   │   ├── crop.rs
│   │   └── <...>.rs          // Other modules
│   └── utils/
│       ├── color_space_converter.rs // RGB, HSV, etc.
│       ├── image_io.rs         // File I/O
│       ├── image_statistics.rs // Histogram, etc.
│       └── <...>.rs            // Other modules
└── tests/
    ├── image_tests.rs          // Integration tests for images
    ├── filter_tests.rs         // Integration tests for filters
    └── transformation_tests.rs // Integration tests for transformations

```

## Explanation:

- Cargo.toml: This file contains metadata about your project, including dependencies and version information.
- src: This directory contains your source code.
- image.rs: Defines the Image struct and related methods.
- pixel.rs: Defines the Pixel struct.
- filters: Contains modules for different filters.
- transformations: Contains modules for different transformations.
- utils: Contains utility functions and classes.
- lib.rs: The main library file that defines the public API.
- tests: Contains test cases for your library.

*lib.rs*
```rust

// Inline Module Declarations

pub mod core {
    pub mod image;
    pub mod pixel;
    pub mod ...
}

pub mod filters {
    pub mod blur;
    pub mod edge_detection;
    pub mod filtering_operations;
    pub mod ...
}

pub mod transformations {
    pub mod crop;
    pub mod transformation_operations;
    pub mod ...
}

pub mod utils {
    pub mod color_space_converter;
    pub mod image_io;
    pub mod ...
}


```

*main.rs*
```rust

use image_processing_lib::Image;

fn main() {
    // Read an existing image
    let image_read = image_reader(PATH);

    // Apply image processing operations
    let transform_operations = vec![
        TransformationOperations::Rotate(RotatingOperations::RotateVertical),
    ];
    let flipped_image =
        TransformationOperations::chain_operations(&image_read.unwrap(), transform_operations);

    // Save the modified image
    let image_write = image_writer(&OUT_PATH, &flipped_image);
}

```

This project structure provides a solid foundation for your image processing library, allowing you to organize your code in a logical and maintainable way.
When designing classes for an image processing library in Rust, it's essential to consider the core concepts and operations involved. Here's a suggested structure:

### Fundamental Classes:
- Pixel: Represents a single pixel in the image. It should contain fields for the red, green, blue, and alpha channels.
- Image: Represents the entire image. It should contain a 2D array of pixels, dimensions (width and height), and potentially metadata.

### Image Processing Operations:
- Filters: Abstract class representing various filters.
- BlurFilter: Implements a blurring filter.
- SharpenFilter: Implements a sharpening filter.
- EdgeDetectionFilter: Implements an edge detection filter.

### Transformations: Abstract class representing various transformations.
- ResizeTransformation: Implements image resizing.
- RotateTransformation: Implements image rotation.
- CropTransformation: Implements image cropping.

### Utility Classes:
- ColorSpaceConverter: Converts images between different color spaces (e.g., RGB, HSV, CMYK).
- ImageIO: Handles reading and writing images to different formats.
- ImageStatistics: Calculates various image statistics (e.g., mean, variance, histogram).

#### Image Statistics: Histogram
In an image statistics histogram, we plot the distribution of pixel intensity values.
Here’s what that typically involves:

- X-Axis (Horizontal): Represents the range of pixel intensity values. For a color image, each channel (red, green, blue) will have its own histogram with the same range.
- Y-Axis (Vertical): Represents the frequency or count of pixels for each intensity value. This shows how many pixels in the image have a particular intensity.

*Example*:
- Color Image: For a color image, you typically have three histograms, one for each color channel (red, green, blue).

For example, a distribution like:
```

5       █  [80]
6       ████  [200]
7       █████████  [370]
8       ██████████████  [606]
9       █████████████████████  [863]
10      ███████████████████████████  [1112]
11      ██████████████████████████████  [1240]
12      ██████████████████████████████  [1228]
13      ███████████████████████████████  [1274]
14      █████████████████████████████  [1205]
15      ██████████████████████████  [1095]
16      ████████████████████  [839]
17      ███████████████  [643]
18      ███████████  [470]
19      ████████  [346]
20      ███████  [295]
21      ██████  [248]
22      █████  [238]
23      ████  [180]

```
represents the number of pixels with that intensity

## IDEAS: Potential Traits for an Image Processing Library

Here are some potential traits you could add to your image processing library, organized by their functionality:

### Image Processing Operations
* **`Filter`:** Defines a generic filtering operation that can be applied to images.
  * `apply(&self, image: &Image) -> Image`
* **`Transformation`:** Defines a generic image transformation that can be applied to images.
  * `apply(&self, image: &Image) -> Image`
* **`EdgeDetectable`:** Defines an operation that can detect edges in an image.
  * `detect_edges(&self, image: &Image) -> Image`
* **`ColorAdjustable`:** Defines an operation that can adjust the color properties of an image.
  * `adjust_color(&self, image: &Image) -> Image`
* **`MorphologicalOperation`:** Defines a morphological operation that can be applied to images.
  * `apply(&self, image: &Image) -> Image`

### Image Features
* **`FeatureExtractor`:** Defines an operation that can extract features from an image.
  * `extract_features(&self, image: &Image) -> Vec<Feature>`
* **`KeypointExtractor`:** Defines an operation that can extract keypoints from an image.
  * `extract_keypoints(&self, image: &Image) -> Vec<Keypoint>`

### Image Analysis
* **`ImageAnalyzer`:** Defines an operation that can analyze an image to extract information.
  * `analyze(&self, image: &Image) -> AnalysisResult`

### Image Comparison
* **`ImageComparator`:** Defines an operation that can compare two images.
  * `compare(&self, image1: &Image, image2: &Image) -> ComparisonResult`

### Image I/O
* **`ImageReadable`:** Defines an operation that can read an image from a file.
  * `read(&self, path: &Path) -> Result<Image, Error>`
* **`ImageWritable`:** Defines an operation that can write an image to a file.
  * `write(&self, image: &Image, path: &Path) -> Result<(), Error>`

### Image Data
* **`PixelData`:** Defines a trait for pixel data, allowing for different pixel formats.
  * `get_value(&self) -> PixelValue`
  * `set_value(&mut self, value: PixelValue)`

### Image Processing Pipelines
* **`ImageProcessorPipeline`:** Defines a pipeline of image processing operations that can be applied sequentially.
  * `process(&self, image: &Image) -> Image`

By defining these traits, you can create a more modular and flexible image processing library. You can then implement specific image processing operations as structs that implement these traits. This allows you to easily combine different operations into pipelines and create new operations by combining existing ones.

## IDEAS: Functionality
Functionality:

Advanced Filters: Implement advanced filters like median blur, non-local means filtering, bilateral filtering, anisotropic diffusion.
Color Manipulation: Add functionality for color space conversions (e.g., RGB to HSV, LAB), color adjustments (e.g., brightness, contrast, saturation), and color correction (e.g., white balance).
Image Segmentation: Implement basic segmentation techniques like thresholding, k-means clustering, watershed segmentation.
Morphological Operations: Include morphological operations like erosion, dilation, opening, closing for object extraction and shape analysis.
Feature Extraction: Extract features relevant to computer vision tasks like edges (canny edge detection), corners (Harris corners), SIFT features, SURF features.
Machine Learning Integration: Allow integration with machine learning libraries (e.g., TensorFlow, PyTorch) for tasks like image classification, object detection, and image generation.


<!-- Check: https://github.com/mbrlabs/pixl/tree/master/src/pixl -->
<!-- https://medium.com/@lahiru.19/a-guide-to-image-processing-from-scratch-7a6a413fb682 -->
