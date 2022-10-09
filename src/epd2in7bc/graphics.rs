use crate::epd2in7bc::{DEFAULT_BACKGROUND_COLOR, HEIGHT, WIDTH};
use crate::graphics::{DisplayRotation, DisplayColorRendering, TriDisplay};
use crate::prelude::TriColor;
use embedded_graphics_core::prelude::*;

/// Full size buffer for use with the 2in7B EPD
///
/// Can also be manuall constructed:
/// `buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value(); WIDTH * HEIGHT / 8]`
pub struct Display2in7bc {
    buffer: [u8; 2 * (WIDTH as usize * HEIGHT as usize / 8)],
    rotation: DisplayRotation,
}

impl Default for Display2in7bc {
    fn default() -> Self {
        Display2in7bc {
            buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value();
                2 * (WIDTH as usize * HEIGHT as usize / 8)],
            rotation: DisplayRotation::default(),
        }
    }
}

impl DrawTarget for Display2in7bc {
    type Color = TriColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.draw_helper_tri(WIDTH, HEIGHT, pixel, DisplayColorRendering::Positive)?;
        }
        Ok(())
    }
}

impl OriginDimensions for Display2in7bc {
    fn size(&self) -> Size {
        match self.rotation {
            DisplayRotation::Rotate0 => Size::new(WIDTH, HEIGHT),
            DisplayRotation::Rotate180 => Size::new(WIDTH, HEIGHT),
            DisplayRotation::Rotate270 => Size::new(HEIGHT, WIDTH),
            DisplayRotation::Rotate90 => Size::new(HEIGHT, WIDTH),
        } 
    }
}

impl TriDisplay for Display2in7bc {
    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn get_mut_buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }

    fn rotation(&self) -> DisplayRotation {
        self.rotation
    }

    fn chromatic_offset(&self) -> usize {
        (WIDTH as usize * HEIGHT as usize / 8) as usize
    }

    fn bw_buffer(&self) -> &[u8] {
        &self.buffer[0..self.chromatic_offset()]
    }

    fn chromatic_buffer(&self) -> &[u8] {
        &self.buffer[self.chromatic_offset()..]
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Black;
    use crate::color::Color;
    use crate::epd2in7bc;
    use crate::epd2in7bc::{HEIGHT, WIDTH};
    use crate::graphics::{Display, DisplayRotation};
    use embedded_graphics::{
        prelude::*,
        primitives::{Line, PrimitiveStyle},
    };

    // test buffer length
    #[test]
    fn graphics_size() {
        let display = Display2in7bc::default();
        assert_eq!(display.buffer().len(), 5808*2);
    }

    // test default background color on all bytes
    #[test]
    fn graphics_default() {
        let display = Display2in7bc::default();
        for &byte in display.buffer() {
            assert_eq!(byte, epd2in7bc::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_0() {
        let mut display = Display2in7bc::default();
        let _ = Line::new(Point::new(0, 0), Point::new(7, 0))
            .into_styled(PrimitiveStyle::with_stroke(Black, 1))
            .draw(&mut display);

        let buffer = display.buffer();

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, epd2in7bc::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_90() {
        let mut display = Display2in7bc::default();
        display.set_rotation(DisplayRotation::Rotate90);
        let _ = Line::new(
            Point::new(0, WIDTH as i32 - 8),
            Point::new(0, WIDTH as i32 - 1),
        )
        .into_styled(PrimitiveStyle::with_stroke(Black, 1))
        .draw(&mut display);

        let buffer = display.buffer();

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, epd2in7bc::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_180() {
        let mut display = Display2in7bc::default();
        display.set_rotation(DisplayRotation::Rotate180);

        let _ = Line::new(
            Point::new(WIDTH as i32 - 8, HEIGHT as i32 - 1),
            Point::new(WIDTH as i32 - 1, HEIGHT as i32 - 1),
        )
        .into_styled(PrimitiveStyle::with_stroke(Black, 1))
        .draw(&mut display);

        let buffer = display.buffer();

        extern crate std;
        std::println!("{:?}", buffer);

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, epd2in7bc::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_270() {
        let mut display = Display2in7bc::default();
        display.set_rotation(DisplayRotation::Rotate270);
        let _ = Line::new(
            Point::new(HEIGHT as i32 - 1, 0),
            Point::new(HEIGHT as i32 - 1, 7),
        )
        .into_styled(PrimitiveStyle::with_stroke(Black, 1))
        .draw(&mut display);

        let buffer = display.buffer();

        extern crate std;
        std::println!("{:?}", buffer);

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, epd2in7bc::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }
}
*/