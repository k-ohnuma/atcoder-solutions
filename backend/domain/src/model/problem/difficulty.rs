#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Difficulty(i32);

impl Difficulty {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn value(self) -> i32 {
        self.0
    }

    pub fn band(self) -> DifficultyBand {
        match self.0 {
            ..400 => DifficultyBand::Gray,
            400..800 => DifficultyBand::Brown,
            800..1200 => DifficultyBand::Green,
            1200..1600 => DifficultyBand::Cyan,
            1600..2000 => DifficultyBand::Blue,
            2000..2400 => DifficultyBand::Yellow,
            2400..2800 => DifficultyBand::Orange,
            _ => DifficultyBand::Red,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DifficultyBand {
    Gray,
    Brown,
    Green,
    Cyan,
    Blue,
    Yellow,
    Orange,
    Red,
}

impl DifficultyBand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gray => "gray",
            Self::Brown => "brown",
            Self::Green => "green",
            Self::Cyan => "cyan",
            Self::Blue => "blue",
            Self::Yellow => "yellow",
            Self::Orange => "orange",
            Self::Red => "red",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Difficulty, DifficultyBand};

    #[test]
    fn classifies_boundary_values() {
        let cases = [
            (-1, DifficultyBand::Gray),
            (399, DifficultyBand::Gray),
            (400, DifficultyBand::Brown),
            (799, DifficultyBand::Brown),
            (800, DifficultyBand::Green),
            (1199, DifficultyBand::Green),
            (1200, DifficultyBand::Cyan),
            (1599, DifficultyBand::Cyan),
            (1600, DifficultyBand::Blue),
            (1999, DifficultyBand::Blue),
            (2000, DifficultyBand::Yellow),
            (2399, DifficultyBand::Yellow),
            (2400, DifficultyBand::Orange),
            (2799, DifficultyBand::Orange),
            (2800, DifficultyBand::Red),
        ];

        for (value, expected) in cases {
            assert_eq!(Difficulty::new(value).band(), expected);
        }
    }

    #[test]
    fn exposes_numeric_value_and_band_name() {
        let difficulty = Difficulty::new(1279);

        assert_eq!(difficulty.value(), 1279);
        assert_eq!(difficulty.band().as_str(), "cyan");
    }
}
