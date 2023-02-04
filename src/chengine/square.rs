use std::{fmt, ops};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    pub fn new(square: &str) -> Option<Square> {
        let mut str_iter = square.chars();
        let range_check = |x| {
            if x < 8 {
                Some(x as u8)
            } else {
                None
            }
        };
        Some(Square {
            x: range_check(str_iter.next()? as u32 - 97)?,
            y: range_check(str_iter.next()? as u32 - 49)?,
        })
    }
    pub fn disp(&self) -> String {
        format!(
            "{}{}",
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][self.x as usize],
            self.y + 1
        )
    }
}

impl std::convert::Into<Square> for (u8, u8) {
    fn into(self) -> Square {
        Square {
            x: self.0,
            y: self.1,
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][self.x as usize],
            self.y + 1
        )
    }
}

impl ops::Add<(i8, i8)> for Square {
    type Output = Option<Self>;

    fn add(self, other: (i8, i8)) -> Option<Self> {
        let result = (self.x as i8 + other.0, self.y as i8 + other.1);
        if 0 <= result.0 && result.0 < 8 && 0 <= result.1 && result.1 < 8 {
            Some(Self {
                x: result.0 as u8,
                y: result.1 as u8,
            })
        } else {
            None
        }
    }
}
