#![cfg(feature = "image")]

use crate::render::{Canvas, Pixel as MyPixel};
use crate::types::Color;
use image::{ImageBuffer, Rgb, Rgba, Luma, LumaA};

// Implement MyPixel for Rgb<u8>
impl MyPixel for Rgb<u8> {
    type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;
    type Canvas = (Rgb<u8>, Self::Image);

    fn default_color(color: Color) -> Self {
        let p = match color.select(0, u8::max_value()) {
            p => p,
        };
        Rgb([p, p, p])
    }
}

// Implement MyPixel for Rgba<u8>
impl MyPixel for Rgba<u8> {
    type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;
    type Canvas = (Rgba<u8>, Self::Image);

    fn default_color(color: Color) -> Self {
        let p = match color.select(0, u8::max_value()) {
            p => p,
        };
        Rgba([p, p, p, u8::max_value()])
    }
}

// Example implementation of Canvas for Rgb<u8>
impl Canvas for (Rgb<u8>, ImageBuffer<Rgb<u8>, Vec<u8>>) {
    type Pixel = Rgb<u8>;
    type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

    // Implement the required methods
    fn new(width: u32, height: u32, dark_pixel: Self::Pixel, light_pixel: Self::Pixel) -> Self {
        (dark_pixel, ImageBuffer::from_pixel(width, height, light_pixel))
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        self.1.put_pixel(x, y, self.0);
    }

    fn into_image(self) -> Self::Image {
        self.1
    }
}

// Example implementation of Canvas for Rgba<u8>
impl Canvas for (Rgba<u8>, ImageBuffer<Rgba<u8>, Vec<u8>>) {
    type Pixel = Rgba<u8>;
    type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

    // Implement the required methods
    fn new(width: u32, height: u32, dark_pixel: Self::Pixel, light_pixel: Self::Pixel) -> Self {
        (dark_pixel, ImageBuffer::from_pixel(width, height, light_pixel))
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        self.1.put_pixel(x, y, self.0);
    }

    fn into_image(self) -> Self::Image {
        self.1
    }
}

// Implement MyPixel for Luma<u8>
impl MyPixel for Luma<u8> {
    type Image = ImageBuffer<Luma<u8>, Vec<u8>>;
    type Canvas = (Luma<u8>, Self::Image);

    fn default_color(color: Color) -> Self {
        let p = match color.select(0, u8::max_value()) {
            p => p,
        };
        Luma([p])
    }
}

// Implement MyPixel for LumaA<u8>
impl MyPixel for LumaA<u8> {
    type Image = ImageBuffer<LumaA<u8>, Vec<u8>>;
    type Canvas = (LumaA<u8>, Self::Image);

    fn default_color(color: Color) -> Self {
        let p = match color.select(0, u8::max_value()) {
            p => p,
        };
        LumaA([p, u8::max_value()])
    }
}

// Implementations for Rgb<u8> and Rgba<u8>
// ...

// Example implementation of Canvas for Luma<u8>
impl Canvas for (Luma<u8>, ImageBuffer<Luma<u8>, Vec<u8>>) {
    type Pixel = Luma<u8>;
    type Image = ImageBuffer<Luma<u8>, Vec<u8>>;

    // Implement the required methods
    fn new(width: u32, height: u32, dark_pixel: Self::Pixel, light_pixel: Self::Pixel) -> Self {
        (dark_pixel, ImageBuffer::from_pixel(width, height, light_pixel))
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        self.1.put_pixel(x, y, self.0);
    }

    fn into_image(self) -> Self::Image {
        self.1
    }
}

// Example implementation of Canvas for LumaA<u8>
impl Canvas for (LumaA<u8>, ImageBuffer<LumaA<u8>, Vec<u8>>) {
    type Pixel = LumaA<u8>;
    type Image = ImageBuffer<LumaA<u8>, Vec<u8>>;

    // Implement the required methods
    fn new(width: u32, height: u32, dark_pixel: Self::Pixel, light_pixel: Self::Pixel) -> Self {
        (dark_pixel, ImageBuffer::from_pixel(width, height, light_pixel))
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        self.1.put_pixel(x, y, self.0);
    }

    fn into_image(self) -> Self::Image {
        self.1
    }
}

#[cfg(test)]
mod render_tests {
    use crate::render::Renderer;
    use crate::types::Color;
    use image::{Luma, Rgba};

    #[test]
    fn test_render_luma8_unsized() {
        let image = Renderer::<Luma<u8>>::new(
            &[
                Color::Light,
                Color::Dark,
                Color::Dark,
                //
                Color::Dark,
                Color::Light,
                Color::Light,
                //
                Color::Light,
                Color::Dark,
                Color::Light,
            ],
            3,
            1,
        )
        .module_dimensions(1, 1)
        .build();

        #[rustfmt::skip]
        let expected = [
            255, 255, 255, 255, 255,
            255, 255,   0,   0, 255,
            255,   0, 255, 255, 255,
            255, 255,   0, 255, 255,
            255, 255, 255, 255, 255,
        ];
        assert_eq!(image.into_raw(), expected);
    }

    #[test]
    fn test_render_rgba_unsized() {
        let image = Renderer::<Rgba<u8>>::new(&[Color::Light, Color::Dark, Color::Dark, Color::Dark], 2, 1)
            .module_dimensions(1, 1)
            .build();

        #[rustfmt::skip]
        let expected: &[u8] = &[
            255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
            255,255,255,255, 255,255,255,255,   0,  0,  0,255, 255,255,255,255,
            255,255,255,255,   0,  0,  0,255,   0,  0,  0,255, 255,255,255,255,
            255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
        ];

        assert_eq!(image.into_raw(), expected);
    }

    #[test]
    fn test_render_resized_min() {
        let image = Renderer::<Luma<u8>>::new(&[Color::Dark, Color::Light, Color::Light, Color::Dark], 2, 1)
            .min_dimensions(10, 10)
            .build();

        #[rustfmt::skip]
        let expected: &[u8] = &[
            255,255,255, 255,255,255, 255,255,255, 255,255,255,
            255,255,255, 255,255,255, 255,255,255, 255,255,255,
            255,255,255, 255,255,255, 255,255,255, 255,255,255,

            255,255,255,   0,  0,  0, 255,255,255, 255,255,255,
            255,255,255,   0,  0,  0, 255,255,255, 255,255,255,
            255,255,255,   0,  0,  0, 255,255,255, 255,255,255,

            255,255,255, 255,255,255,   0,  0,  0, 255,255,255,
            255,255,255, 255,255,255,   0,  0,  0, 255,255,255,
            255,255,255, 255,255,255,   0,  0,  0, 255,255,255,

            255,255,255, 255,255,255, 255,255,255, 255,255,255,
            255,255,255, 255,255,255, 255,255,255, 255,255,255,
            255,255,255, 255,255,255, 255,255,255, 255,255,255,
        ];

        assert_eq!(image.dimensions(), (12, 12));
        assert_eq!(image.into_raw(), expected);
    }

    #[test]
    fn test_render_resized_max() {
        let image = Renderer::<Luma<u8>>::new(&[Color::Dark, Color::Light, Color::Light, Color::Dark], 2, 1)
            .max_dimensions(10, 5)
            .build();

        #[rustfmt::skip]
        let expected: &[u8] = &[
            255,255, 255,255, 255,255, 255,255,

            255,255,   0,  0, 255,255, 255,255,

            255,255, 255,255,   0,  0, 255,255,

            255,255, 255,255, 255,255, 255,255,
        ];

        assert_eq!(image.dimensions(), (8, 4));
        assert_eq!(image.into_raw(), expected);
    }
}
