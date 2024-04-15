#[cfg(feature = "wasm")]
pub mod wasm;

#[derive(Default)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub fn from_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(
            (255.0 * r) as u8,
            (255.0 * g) as u8,
            (255.0 * b) as u8,
            (255.0 * a) as u8,
        )
    }
}

impl From<Rgba> for u32 {
    fn from(rgba: Rgba) -> Self {
        let r: u32 = rgba.0.into();
        let g: u32 = rgba.1.into();
        let b: u32 = rgba.2.into();
        let a: u32 = rgba.3.into();
        r << 24 | g << 16 | b << 8 | a
    }
}

pub trait FloatToRgba {
    fn convert(&self, f: f32) -> Rgba;
    fn convert_many(&self, input: &[f32], output: &mut Vec<Rgba>) {
        output.clear();
        input.iter().for_each(|i: &f32| {
            let converted: Rgba = self.convert(*i);
            output.push(converted);
        });
    }
}

impl<F> FloatToRgba for F
where
    F: Fn(f32) -> Rgba,
{
    fn convert(&self, f: f32) -> Rgba {
        self(f)
    }
}

pub trait Normalize {
    /// Converts an original float number to a "normalized" float number:
    /// - "normalized" lower bound: 0.0
    /// - "normalized" upper bound: 1.0
    fn normalize(&self, original: f32) -> f32;
}

/// Converts a "normalized" float number to an Rgba value.
pub struct NormalizedConverter<F, N> {
    pub normalizer: N,
    pub converter: F,
}

impl<F, N> FloatToRgba for NormalizedConverter<F, N>
where
    F: FloatToRgba,
    N: Normalize,
{
    fn convert(&self, f: f32) -> Rgba {
        let normalized: f32 = self.normalizer.normalize(f);
        self.converter.convert(normalized)
    }
}

/// Simple example converter.
///
/// | normalized value | r    | g    | b    | r(f), g(f), b(f)         |
/// |:----------------:|:----:|:----:|:----:|:------------------------:|
/// | 0.000            | 1.00 | 0.00 | 0.00 | g = 2f, r = 1.0 - g      |
/// | 0.125            | 0.75 | 0.25 | 0.00 |                          |
/// | 0.250            | 0.50 | 0.50 | 0.00 |                          |
/// | 0.375            | 0.25 | 0.75 | 0.00 |                          |
/// | 0.500            | 0.00 | 1.00 | 0.00 | b = 2(f-0.5), g= 1.0 - b |
/// | 0.625            | 0.00 | 0.75 | 0.25 |                          |
/// | 0.750            | 0.00 | 0.50 | 0.50 |                          |
/// | 0.875            | 0.00 | 0.25 | 0.75 |                          |
/// | 1.000            | 0.00 | 0.00 | 1.00 |                          |
pub fn float2rgba_simple(f: f32) -> Rgba {
    let g05: f32 = 2.0 * f;
    let r05: f32 = 1.0 - g05;
    let b05: f32 = 0.0;
    let a05: f32 = 1.0;
    let rgba05: Rgba = Rgba::from_f(r05, g05, b05, a05);

    let f51: f32 = f - 0.5;
    let b51: f32 = 2.0 * f51;
    let g51: f32 = 1.0 - b51;
    let r51: f32 = 0.0;
    let a51: f32 = 1.0;
    let rgba51: Rgba = Rgba::from_f(r51, g51, b51, a51);

    let oolb: bool = f < 0.0;
    let ooub: bool = 1.0 < f;
    let oob: bool = oolb || ooub;

    let over_half: bool = 0.5 < f;

    let rgba: Rgba = if over_half { rgba51 } else { rgba05 };

    match oob {
        true => Rgba::default(),
        false => rgba,
    }
}

#[cfg(test)]
mod test_float2rgba_simple {
    use super::{float2rgba_simple, Rgba};

    #[test]
    fn out_of_bound() {
        let f: f32 = -1.0;
        let rgba: Rgba = float2rgba_simple(f);
        assert_eq!(rgba.0, 0);
        assert_eq!(rgba.1, 0);
        assert_eq!(rgba.2, 0);
        assert_eq!(rgba.3, 0);
    }

    #[test]
    fn red() {
        let f: f32 = 0.0;
        let rgba: Rgba = float2rgba_simple(f);
        assert_eq!(rgba.0, 255);
        assert_eq!(rgba.1, 0);
        assert_eq!(rgba.2, 0);
        assert_eq!(rgba.3, 255);
    }

    #[test]
    fn green() {
        let f: f32 = 0.5;
        let rgba: Rgba = float2rgba_simple(f);
        assert_eq!(rgba.0, 0);
        assert_eq!(rgba.1, 255);
        assert_eq!(rgba.2, 0);
        assert_eq!(rgba.3, 255);
    }

    #[test]
    fn blue() {
        let f: f32 = 1.0;
        let rgba: Rgba = float2rgba_simple(f);
        assert_eq!(rgba.0, 0);
        assert_eq!(rgba.1, 0);
        assert_eq!(rgba.2, 255);
        assert_eq!(rgba.3, 255);
    }
}
