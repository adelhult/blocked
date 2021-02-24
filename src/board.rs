use std::fmt;

pub type Tile = (u32, u32);

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Piece {
    size: u32,
    location: Tile,
    direction: Direction,
    marked: bool,
}

impl Piece {
    /// Create a new piece
    pub fn new(location: Tile, size: u32, direction: Direction) -> Self {
        Piece {
            marked: false,
            location,
            direction,
            size,
        }
    }
    /// Create a new marked piece
    pub fn marked(location: Tile, size: u32, direction: Direction) -> Self {
        Piece {
            marked: true,
            location,
            direction,
            size,
        }
    }

    pub fn occupies(&self) -> Vec<Tile> {
        let (x, y) = self.location;
        (0..self.size)
            .into_iter()
            .map(|i| match self.direction {
                Direction::Horizontal => (x + i, y),
                Direction::Vertical => (x, y + i),
            })
            .collect()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Move {
    Left(Tile, u32),
    Right(Tile, u32),
    Up(Tile, u32),
    Down(Tile, u32),
}

impl Move {
    /// Get the tile from which the move
    /// occurs
    pub fn get_tile(&self) -> Tile {
        match *self {
            Move::Left(t, _) => t,
            Move::Right(t, _) => t,
            Move::Up(t, _) => t,
            Move::Down(t, _) => t,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (tile, dir, steps) = match self {
            Move::Left(t, steps)    => (t, "left", steps),
            Move::Right(t, steps)   => (t, "right", steps),
            Move::Up(t, steps)      => (t, "up", steps),
            Move::Down(t, steps)    => (t, "down", steps),
        };

        write!(f, "Move ({},{}) {} by {} steps", tile.0, tile.1, dir, steps)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub goal: Tile,
    pub is_won: bool,
    pieces: Vec<Piece>,
    pub occupied_tiles: Vec<Tile>,
}

impl Board {
    /// Create a new board
    pub fn new(width: u32, height: u32, goal: Tile, pieces: Vec<Piece>) -> Self {
        let occupied_tiles = Board::occupied_tiles(&pieces);
        let is_won = pieces
            .iter()
            .find(|p| p.marked && p.occupies().contains(&goal))
            .is_some();

        Board {
            occupied_tiles,
            is_won,
            width,
            height,
            pieces,
            goal,
        }
    }

    /// Find all the possible moves
    pub fn all_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        for piece in &self.pieces {
            let (x, y) = piece.location;

            match piece.direction {
                Direction::Horizontal => {
                    let (start, end) = (x, x + piece.size - 1);

                    for i in 1.. {
                        if !self.empty_tile((end + i, y)) {
                            break;
                        }
                        moves.push(Move::Right((x, y), i));
                    }

                    for i in 1.. {
                        if start < i || !self.empty_tile((start - i, y)) {
                            break;
                        }
                        moves.push(Move::Left((x, y), i));
                    }
                }
                Direction::Vertical => {
                    let (start, end) = (y, y + piece.size - 1);

                    for i in 1.. {
                        if start < i || !self.empty_tile((x, start - i)) {
                            break;
                        }
                        moves.push(Move::Up((x, y), i));
                    }

                    for i in 1.. {
                        if !self.empty_tile((x, end + i)) {
                            break;
                        }
                        moves.push(Move::Down((x, y), i));
                    }
                }
            }
        }
        moves
    }

    /// Get tuples of future boards and the move to get there
    pub fn future_boards(&self) -> Vec<(Board, Move)> {
        self.all_moves()
            .into_iter()
            .map(|next| (self.play(&next), next))
            .collect()
    }

    /// Given a move, generate a new board
    pub fn play(&self, mov: &Move) -> Board {
        let pieces = self
            .pieces
            .clone()
            .into_iter()
            .map(|mut p| {
                if p.location == mov.get_tile() {
                    // if the current piece is the one that the move
                    // concerns, then move it:
                    let (x, y) = p.location;
                    match mov {
                        Move::Left(_, steps) => p.location = (x - steps, y),
                        Move::Right(_, steps) => p.location = (x + steps, y),
                        Move::Up(_, steps) => p.location = (x, y - steps),
                        Move::Down(_, steps) => p.location = (x, y + steps),
                    }
                }
                p
            })
            .collect();

        Board::new(self.width, self.height, self.goal, pieces)
    }

    /// Given a move, reverse the action and return that board.
    pub fn undo(&self, mov: &Move) -> Board {
        let reverse_move = match *mov {
            Move::Left((x, y), steps) => Move::Right((x - steps, y), steps),
            Move::Right((x, y), steps) => Move::Left((x + steps, y), steps),
            Move::Up((x, y), steps) => Move::Down((x, y - steps), steps),
            Move::Down((x, y), steps) => Move::Up((x, y + steps), steps),
        };

        self.play(&reverse_move)
    }

    /// Given a list of pieces, find all the occupied tiles
    /// This functions is used when initing new boards.
    pub fn occupied_tiles(pieces: &Vec<Piece>) -> Vec<Tile> {
        pieces.iter().flat_map(|p| p.occupies()).collect()
    }

    /// Check if a tile is free
    pub fn empty_tile(&self, t: Tile) -> bool {
        self.tile_exists(t) && !self.occupied_tiles.contains(&t)
    }

    /// Check if a tile exists on the board,
    /// i.e has a lower value than the width/height and, greater or equal to 0
    pub fn tile_exists(&self, (x, y): Tile) -> bool {
        x < self.width && y < self.height
    }
}
