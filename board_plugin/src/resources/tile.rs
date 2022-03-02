#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tile {
    /// Is a Bomb
    Bomb,
    /// Is a bomb neighbor
    BombNeighbor(u8),
    /// Empty tile
    Empty,
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbor(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
}
