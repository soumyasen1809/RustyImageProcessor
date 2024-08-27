// Inline Module Declarations

pub mod core {
    pub mod image;
    pub mod pixel;
}

pub mod filters {
    pub mod blur;
    pub mod edge_detection;
    pub mod filtering_operations;
    pub mod gray_scale;
    pub mod sharpen;
}

pub mod transformations {
    pub mod crop;
    pub mod resize;
    pub mod rotate;
}

pub mod utils {
    pub mod color_space_converter;
    pub mod image_io;
}
