use log::*;
trait PaletteColorize {
    fn get_color(&self, value: f32) -> [u8; 3];
    fn get_color_under_range(&self) -> [u8; 3];
    fn get_color_over_range(&self) -> [u8; 3];
}

pub enum Palettes {
    Default,
    Extended,
}

impl PaletteColorize for Palettes {
    fn get_color(&self, value: f32) -> [u8; 3] {
        match self {
            Palettes::Default => {
                let value = rescale_value_to(value, 0.0, 255.0) as u8;
                [value, value, 50]
            }

            Palettes::Extended => {
                let value = rescale_value_to(value, 0.0, 255.0);
                let whole = value as u8;
                let fract = value.fract();
                if fract < 0.33 {
                    [whole, whole - 1, 50]
                } else if fract < 0.66 {
                    [whole - 1, whole, 50]
                } else {
                    [whole, whole, 50]
                }
            }
        }
    }
    fn get_color_under_range(&self) -> [u8; 3] {
        match self {
            Palettes::Default => [0, 0, 0],
            Palettes::Extended => [0, 0, 0],
        }
    }
    fn get_color_over_range(&self) -> [u8; 3] {
        match self {
            Palettes::Default => [255, 255, 255],
            Palettes::Extended => [255, 255, 255],
        }
    }
}

/// Scale a value from between min and max to between 0 and 1
fn rescale_value_from(value: f32, min: f32, max: f32) -> f32 {
    let old_range = max - min;
    (value - min) / old_range
}

/// Scale a value from between 0 and 1 to between min and max
fn rescale_value_to(value: f32, min: f32, max: f32) -> f32 {
    value.mul_add(max, min)
}

/// Places value on a scale from min to max, and transforms it to an integer scale from 0 to 255. Returns a color using the specified palette.
pub fn scale_tocolor(palette: Palettes, value: f32, min: f32, max: f32) -> [u8; 3] {
    let scaled = rescale_value_from(value, min, max);
    if scaled < 0.0 {
        warn!("Computed invalid color! Value range: {} to {}, Value: {}, Color range: 0-255, Color: {}", min,max,value,scaled);
        palette.get_color_under_range()
    } else if scaled > 1.0 {
        warn!("Computed invalid color! Value range: {} to {}, Value: {}, Color range: 0-255, Color: {}", min,max,value,scaled);
        palette.get_color_over_range()
    } else {
        palette.get_color(scaled)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use proptest::prelude::*;
    #[test]
    fn normalize_goes_up() {
        assert_eq!(
            (0..255)
                .map(|v| v as f32)
                .map(|v| scale_tocolor(Palettes::Default, v, 0.0, 255.0)
                    .first()
                    .cloned()
                    .unwrap())
                .collect::<Vec<_>>(),
            (0..255).map(|v| v as u8).collect::<Vec<_>>()
        );
    }

    #[test]
    fn normalize_max() {
        assert_eq!(
            scale_tocolor(Palettes::Default, 23.02, -29.4, 23.02),
            [255, 255, 50]
        );
    }
    proptest! {
          #![proptest_config(ProptestConfig {
      cases: 10000, .. ProptestConfig::default()
    })]
          #[test]
          fn scale_tocolor_within_bounds(a in proptest::num::f32::ANY,b  in proptest::num::f32::ANY, c in proptest::num::f32::ANY) {
              let min = a.min(b).min(c);
              let mid = a.min(b).max( a.max(b).min(c));
              let max = a.max(b).max(c);
              scale_tocolor(Palettes::Default,mid,min,max);
          }
      }
}
