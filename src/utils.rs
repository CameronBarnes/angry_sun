use derive_more::derive::IsVariant;
use serde::Deserialize;

#[must_use]
pub fn format_number(mut number: f32) -> String {
    let suffix = if number >= 1_000_000_000. {
        number /= 1_000_000_000.;
        Some("B")
    } else if number >= 1_000_000. {
        number /= 1_000_000.;
        Some("M")
    } else if number >= 10_000. {
        number /= 1000.;
        Some("k")
    } else {
        None
    };

    // 0.001 is the error margin
    if (number - number.floor()).abs() < 0.001 {
        format!("{number}{}", suffix.unwrap_or(""))
    } else {
        format!("{number:.2}{}", suffix.unwrap_or(""))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, IsVariant)]
pub enum Value {
    Abs(f32),
    Percentage(f32),
}

impl Value {
    pub fn scale(&mut self, scale: f32) {
        if let Self::Percentage(val) = self {
            *self = Self::Abs(*val * scale);
        }
    }
}
