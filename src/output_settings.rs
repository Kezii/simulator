use crate::theme::BinaryColorTheme;
use embedded_graphics::prelude::*;

/// Output settings.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OutputSettings {
    /// Pixel scale, allowing for non-square pixels.
    pub scale: Size,
    /// Spacing between pixels.
    pub pixel_spacing: u32,
    /// Binary color theme.
    pub theme: BinaryColorTheme,
}

impl OutputSettings {
    /// Translates a output coordinate to the corresponding display coordinate.
    #[cfg(feature = "with-sdl")]
    pub(crate) const fn output_to_display(&self, output_point: Point) -> Point {
        output_point.component_div(self.pixel_pitch())
    }

    pub(crate) const fn pixel_pitch(&self) -> Point {
        Point::new(
            (self.scale.width + self.pixel_spacing) as i32,
            (self.scale.height + self.pixel_spacing) as i32,
        )
    }
}

impl Default for OutputSettings {
    fn default() -> Self {
        OutputSettingsBuilder::new().build()
    }
}

/// Output settings builder.
#[derive(Default)]
pub struct OutputSettingsBuilder {
    scale: Option<Size>,
    pixel_spacing: Option<u32>,
    theme: BinaryColorTheme,
}

impl OutputSettingsBuilder {
    /// Creates new output settings builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the pixel scale.
    ///
    /// A scale of `2` or higher is useful for viewing the simulator on high DPI displays.
    ///
    /// # Panics
    ///
    /// Panics if the scale is set to `0`.
    pub fn scale(mut self, scale: u32) -> Self {
        assert!(scale > 0, "scale must be > 0");

        self.scale = Some(Size::new(scale, scale));

        self
    }

    /// Sets a non-square pixel scale.
    ///
    /// This is useful for simulating a display with a non-square pixel aspect ratio.
    ///
    /// # Panics
    ///
    /// Panics if `width` or `height` is `0`.
    pub fn scale_non_square(mut self, scale: Size) -> Self {
        assert!(scale.width > 0, "width must be > 0");
        assert!(scale.height > 0, "height must be > 0");

        self.scale = Some(scale);

        self
    }

    /// Sets the binary color theme.
    ///
    /// The binary color theme defines the mapping between the two display colors
    /// and the output. The variants provided by the [`BinaryColorTheme`] enum
    /// simulate the color scheme of commonly used display types.
    ///
    /// Most binary color displays are relatively small individual pixels
    /// are hard to recognize on higher resolution screens. Because of this
    /// some scaling is automatically applied to the output when a theme is
    /// set and no scaling was specified explicitly.
    ///
    /// Note that a theme should only be set when an monochrome display is used.
    /// Setting a theme when using a color display will cause an corrupted output.
    ///
    pub fn theme(mut self, theme: BinaryColorTheme) -> Self {
        self.theme = theme;

        self.scale.get_or_insert(Size::new_equal(3));
        self.pixel_spacing.get_or_insert(1);

        self
    }

    /// Sets the gap between pixels.
    ///
    /// Most lower resolution displays have visible gaps between individual pixels.
    /// This effect can be simulated by setting the pixel spacing to a value greater
    /// than `0`.
    pub fn pixel_spacing(mut self, pixel_spacing: u32) -> Self {
        self.pixel_spacing = Some(pixel_spacing);

        self
    }

    /// Builds the output settings.
    pub fn build(self) -> OutputSettings {
        OutputSettings {
            scale: self.scale.unwrap_or(Size::new_equal(1)),
            pixel_spacing: self.pixel_spacing.unwrap_or(0),
            theme: self.theme,
        }
    }
}
