const EMPTY_SQUARE: &str = "rgb(255, 255, 255)";
const PIECE_0_COLOR: &str = "rgb(79, 174, 234)";
const PIECE_1_COLOR: &str = "rgb(79, 174, 91)";
const PIECE_2_COLOR: &str = "rgb(254, 255, 84)";
const PIECE_3_COLOR: &str = "rgb(234, 53, 35)";
const PIECE_4_COLOR: &str = "rgb(184, 95, 40)";
const PIECE_5_COLOR: &str = "rgb(246, 195, 67)";
const PIECE_6_COLOR: &str = "rgb(94, 129, 63)";
const PIECE_7_COLOR: &str = "rgb(147, 168, 215)";

#[derive(Debug)]
pub struct AllPieces(Vec<Piece>);

impl AllPieces {
    /// lots of hard-coded values for each piece and all variations (rotated, flipped)
    pub fn create() -> AllPieces {
        let piece0 = Piece {
            available: true,
            color: PIECE_0_COLOR,
            variations: vec![
                PieceVariation {
                    x_len: 2,
                    y_len: 3,
                    piece: vec![true, true, true, true, false, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 3,
                    piece: vec![true, true, true, true, true, false],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 3,
                    piece: vec![false, true, true, true, true, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 3,
                    piece: vec![true, false, true, true, true, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 2,
                    piece: vec![false, true, true, true, true, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 2,
                    piece: vec![true, true, false, true, true, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 2,
                    piece: vec![true, true, true, false, true, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 2,
                    piece: vec![true, true, true, true, true, false],
                },
            ],
        };
        let piece1 = Piece {
            available: true,
            color: PIECE_1_COLOR,
            variations: vec![
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![true, true, true, true, false, true, false, false],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![true, true, true, true, false, false, true, false],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![false, true, false, false, true, true, true, true],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![false, false, true, false, true, true, true, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![false, true, true, true, false, true, false, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![false, true, false, true, true, true, false, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![true, false, true, true, true, false, true, false],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![true, false, true, false, true, true, true, false],
                },
            ],
        };
        let piece2 = Piece {
            available: true,
            color: PIECE_2_COLOR,
            variations: vec![
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![false, false, true, true, true, true, true, false],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![true, true, false, false, false, true, true, true],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![false, true, true, true, true, true, false, false],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![true, true, true, false, false, false, true, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![false, true, true, true, true, false, true, false],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![true, false, true, true, false, true, false, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![false, true, false, true, true, true, true, false],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![true, false, true, false, true, true, false, true],
                },
            ],
        };
        let piece3 = Piece {
            available: true,
            color: PIECE_3_COLOR,
            variations: vec![
                PieceVariation {
                    x_len: 3,
                    y_len: 2,
                    piece: vec![true, true, true, true, true, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 3,
                    piece: vec![true, true, true, true, true, true],
                },
            ],
        };
        let piece4 = Piece {
            available: true,
            color: PIECE_4_COLOR,
            variations: vec![
                PieceVariation {
                    x_len: 3,
                    y_len: 3,
                    piece: vec![true, true, false, false, true, false, false, true, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 3,
                    piece: vec![false, true, true, false, true, false, true, true, false],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 3,
                    piece: vec![true, false, false, true, true, true, false, false, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 3,
                    piece: vec![false, false, true, true, true, true, true, false, false],
                },
            ],
        };
        let piece5 = Piece {
            available: true,
            color: PIECE_5_COLOR,
            variations: vec![
                PieceVariation {
                    x_len: 3,
                    y_len: 2,
                    piece: vec![true, true, true, true, false, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 2,
                    piece: vec![true, false, true, true, true, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 3,
                    piece: vec![true, true, false, true, true, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 3,
                    piece: vec![true, true, true, false, true, true],
                },
            ],
        };
        let piece6 = Piece {
            available: true,
            color: PIECE_6_COLOR,
            variations: vec![
                PieceVariation {
                    x_len: 3,
                    y_len: 3,
                    piece: vec![true, false, false, true, false, false, true, true, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 3,
                    piece: vec![true, true, true, true, false, false, true, false, false],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 3,
                    piece: vec![true, true, true, false, false, true, false, false, true],
                },
                PieceVariation {
                    x_len: 3,
                    y_len: 3,
                    piece: vec![false, false, true, false, false, true, true, true, true],
                },
            ],
        };
        let piece7 = Piece {
            available: true,
            color: PIECE_7_COLOR,
            variations: vec![
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![true, false, false, false, true, true, true, true],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![false, false, false, true, true, true, true, true],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![true, true, true, true, true, false, false, false],
                },
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![true, true, true, true, false, false, false, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![true, true, true, false, true, false, true, false],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![true, true, false, true, false, true, false, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![true, false, true, false, true, false, true, true],
                },
                PieceVariation {
                    x_len: 2,
                    y_len: 4,
                    piece: vec![false, true, false, true, false, true, true, true],
                },
                // duplicate variation
                PieceVariation {
                    x_len: 4,
                    y_len: 2,
                    piece: vec![true, false, false, false, true, true, true, true],
                },
            ],
        };
        /*
        note that piece 7 has a duplicate variation, this is to check if this can prevent
        a cascade of SolvingDirections::GoBackHistory in the event piece 7, variation 7 *needs*
        to be set somewhere later, from the SolvingDirections::GoBackHistory call, if there's
        no next variation for variation 7/7, then SolvingDirections::GoBackHistory is called again.
        I'm not sure why it works, but adding this duplicate variation solves the October 6th
        problem for some reason.
        */

        AllPieces(vec![
            piece0, piece1, piece2, piece3, piece4, piece5, piece6, piece7,
        ])
    }

    pub fn get_piece_by_id(&self, id: PieceId) -> Result<&PieceVariation, &'static str> {
        if id.piece_number as usize + 1 > self.0.len() {
            return Err("not a valid piece number");
        }

        let piece = &self.0[id.piece_number as usize];

        if id.variation_number + 1 > piece.variations.len() {
            return Err("not a valid variation number");
        }

        Ok(&piece.variations[id.variation_number as usize])
    }

    pub fn get_next_piece_id_or_variation_by_id(&self, id: &PieceId) -> Option<PieceId> {
        let this_piece_id = id.piece_number;
        let this_variation_id = id.variation_number;

        let next_piece_id = this_piece_id + 1;
        let next_variation_id = this_variation_id + 1;

        if self.0[this_piece_id].variations.len() > next_variation_id {
            Some(PieceId {
                piece_number: this_piece_id,
                variation_number: next_variation_id,
            })
        } else if self.0.len() > next_piece_id {
            Some(PieceId {
                piece_number: next_piece_id,
                variation_number: 0,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Piece {
    pub color: &'static str,
    pub variations: Vec<PieceVariation>,
    pub available: bool,
}

#[derive(Debug)]
pub struct PieceVariation {
    pub x_len: usize,
    pub y_len: usize,
    pub piece: Vec<bool>,
}

impl std::fmt::Display for PieceVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut return_string: String = "".to_string();
        for (i, sec) in self.piece.iter().enumerate() {
            return_string += if *sec { "■" } else { "☐" };

            if i != 0 && (i + 1) % self.x_len as usize == 0 {
                return_string += "\n";
            }
        }

        write!(f, "{}", return_string)
    }
}

impl PieceVariation {
    pub fn get_index_from_coordinates(&self, coordinates: Coordinates) -> Option<usize> {
        let index = (coordinates.y * self.x_len) + coordinates.x;

        if index > self.x_len * self.y_len {
            None
        } else {
            Some(index)
        }
    }

    pub fn get_coordinates_from_index(&self, index: usize) -> Option<Coordinates> {
        if index > self.x_len * self.y_len {
            None
        } else {
            Some(Coordinates {
                x: index % self.x_len,
                y: index / self.x_len,
            })
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PieceId {
    pub piece_number: usize,
    pub variation_number: usize,
}

#[derive(Debug)]
pub struct HistoryItem {
    location: usize,
    piece_id: PieceId,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl std::ops::Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum BoardSquare {
    Forbidden,
    Empty,
    Filled(PieceId),
}

impl BoardSquare {
    fn is_writable(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Filled(_) => false,
            Self::Forbidden => false,
        }
    }
}

#[derive(Debug)]
enum SolvingDirections {
    SetPiece,
    NextVariation,
    NextSquare,
    GoBackHistory,
}

pub struct Board {
    pub with_color: [&'static str; 49],
    history: Vec<HistoryItem>,
    debug_board: [BoardSquare; 49],
    pub all_pieces: AllPieces,
    pub counter: i64,
}

impl Board {
    pub fn new(month: i32, day: i32) -> Result<Board, &'static str> {
        let mut forbidden_indexes: Vec<usize> = vec![6, 13, 45, 46, 47, 48];

        if (1..=12).contains(&month) {
            // the board has two blocked out boxes at the end, so jan-june are -1 but the next 6
            // don't need to have anything subtracted
            if month <= 6 {
                forbidden_indexes.push(month as usize - 1);
            } else {
                forbidden_indexes.push(month as usize)
            }
        } else {
            return Err("bad date");
        }

        if (1..=31).contains(&day) {
            forbidden_indexes.push(day as usize + 13);
        } else {
            return Err("bad date");
        }

        let mut new_board = Board {
            with_color: [EMPTY_SQUARE; 49],
            history: vec![],
            debug_board: [BoardSquare::Empty; 49],
            all_pieces: AllPieces::create(),
            counter: 0,
        };

        for forbidden_index in forbidden_indexes {
            new_board.debug_board[forbidden_index] = BoardSquare::Forbidden;
        }

        Ok(new_board)
    }

    pub fn get_index_from_coordinates(coordinates: Coordinates) -> Option<usize> {
        if coordinates.y > 6 || coordinates.x > 6 {
            return None;
        }
        let index = (coordinates.y * 7) + coordinates.x;
        if index > 48 {
            None
        } else {
            Some(index)
        }
    }

    pub fn get_coordinates_from_index(index: usize) -> Option<Coordinates> {
        if index > 48 {
            None
        } else {
            Some(Coordinates {
                x: index % 7,
                y: index / 7,
            })
        }
    }

    // gets the affected indices from Board for the piece variant
    // returns none if the true values go out of bounds of the puzzle
    // doesn't check if the index is filled already
    pub fn get_affected_indices(
        &self,
        at_coordinate: &Coordinates,
        piece_id: &PieceId,
    ) -> Option<Vec<usize>> {
        // get this variant
        let this_variant =
            &self.all_pieces.0[piece_id.piece_number].variations[piece_id.variation_number];

        // get a list
        let mut affected_list: Vec<usize> = vec![];

        // get the variant's true values and get their corresponding coordinates from the board
        for (index, piece_box) in this_variant.piece.iter().enumerate() {
            if *piece_box {
                // get the coordinates of the true values
                if let Some(coordinates) = this_variant.get_coordinates_from_index(index) {
                    if !self.all_pieces.0[piece_id.piece_number].available {
                        // this piece isn't available so just return none
                        return None;
                    }
                    // add to the at_coordinate part to find the part on the board this will go
                    let affected_coordinate = at_coordinate.clone() + coordinates;

                    // add this to the list. Unwrap is okay because it'll already be checked for out of bounds.
                    if let Some(index) = Board::get_index_from_coordinates(affected_coordinate) {
                        affected_list.push(index);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }
        Some(affected_list)
    }

    // check the indices. If all are empty and not restricted, write to them. Return true if written, false if unable to
    pub fn check_and_write_piece(
        &mut self,
        piece_id: &PieceId,
        affected_indices: Vec<usize>,
    ) -> bool {
        // cannot set the piece
        if affected_indices
            .iter()
            .any(|index| !self.debug_board[*index].is_writable())
        {
            false
        } else {
            // can set the piece
            for index in affected_indices {
                self.debug_board[index] = BoardSquare::Filled(*piece_id);
                self.with_color[index] = self.all_pieces.0[piece_id.piece_number].color;
            }
            true
        }
    }

    // remove a piece using usize as an argument. just iterate through the set pieces and remove
    // all with the usize being piece id.
    pub fn remove_piece(&mut self, piece_id: &PieceId) {
        // make a list of indexes to make empty
        let mut make_empty_list = vec![];

        for (index, square) in self.debug_board.iter().enumerate() {
            match square {
                BoardSquare::Filled(square_piece_id) => {
                    if square_piece_id == piece_id {
                        make_empty_list.push(index);
                    }
                }
                _ => {}
            }
        }

        // go over list and make empty
        for index in make_empty_list {
            self.set_square_to_empty(index);
        }

        self.all_pieces.0[piece_id.piece_number as usize].available = true;
    }

    // check if all pieces are set
    pub fn check_if_all_pieces_set(&self) -> bool {
        self.debug_board
            .iter()
            .any(|square| *square == BoardSquare::Empty)
    }

    pub fn set_square_to_empty(&mut self, index: usize) {
        self.debug_board[index] = BoardSquare::Empty;
        self.with_color[index] = EMPTY_SQUARE;
    }

    pub fn solve(&mut self) {
        // start at 0 index and 0, 0 piece
        let mut at_index: usize = 0;
        let mut at_piece_id = PieceId {
            piece_number: 0,
            variation_number: 0,
        };
        let mut current_cmd = SolvingDirections::SetPiece;

        loop {
            match current_cmd {
                SolvingDirections::SetPiece => {
                    if at_index < 48 {
                        // check if the current piece can be set (can be unwrapped safely because checked at_index < 48 above)
                        let at_coordinates = Self::get_coordinates_from_index(at_index).unwrap();

                        // check if I can set the current piece at the current coordinates
                        if let Some(affected_coordinates) =
                            self.get_affected_indices(&at_coordinates, &at_piece_id)
                        {
                            if !self.all_pieces.0[at_piece_id.piece_number as usize].available {
                                panic!("tried to set an unavailable piece");
                            }

                            if !self.check_and_write_piece(&at_piece_id, affected_coordinates) {
                                // cannot set the piece, so try the next variation
                                current_cmd = SolvingDirections::NextVariation;
                            } else {
                                // set the piece and move on
                                self.all_pieces.0[at_piece_id.piece_number as usize].available =
                                    false;
                                self.history.push(HistoryItem {
                                    piece_id: at_piece_id.clone(),
                                    location: at_index.clone(),
                                });
                                // set the new piece then go to the next spot
                                current_cmd = SolvingDirections::NextSquare;

                                self.counter += 1;

                                // set the piece, so check if the puzzle is finished
                                if !self.check_if_all_pieces_set() {
                                    println!("{}", self);
                                    break;
                                }
                            }
                        } else {
                            // cannot set so try the next variation
                            current_cmd = SolvingDirections::NextVariation;
                        }
                    } else {
                        // shouldn't get here
                        current_cmd = SolvingDirections::GoBackHistory;
                    }
                }
                SolvingDirections::NextSquare => {
                    at_index += 1;
                    at_piece_id = PieceId {
                        piece_number: 0,
                        variation_number: 0,
                    };

                    if at_index <= 48 {
                        current_cmd = SolvingDirections::SetPiece;
                    } else {
                        current_cmd = SolvingDirections::GoBackHistory;
                    }
                }
                SolvingDirections::NextVariation => {
                    if let Some(next_piece) = self
                        .all_pieces
                        .get_next_piece_id_or_variation_by_id(&at_piece_id)
                    {
                        at_piece_id = next_piece;
                        // got the next variation, now check if it's allowed
                        // if it's not, don't change the command, so it'll continue to check
                        if self.all_pieces.0[next_piece.piece_number as usize].available {
                            current_cmd = SolvingDirections::SetPiece;
                        }
                    } else {
                        // no next variation so just try the next square
                        current_cmd = SolvingDirections::NextSquare;
                    }
                }
                SolvingDirections::GoBackHistory => {
                    // get last history item and continue from there
                    if let Some(last_history) = self.history.pop() {
                        self.remove_piece(&last_history.piece_id);
                        at_index = last_history.location;
                        if let Some(is_next_variation) = self
                            .all_pieces
                            .get_next_piece_id_or_variation_by_id(&PieceId {
                                piece_number: last_history.piece_id.piece_number,
                                variation_number: last_history.piece_id.variation_number,
                            })
                        {
                            at_piece_id = is_next_variation;
                            current_cmd = SolvingDirections::SetPiece;
                        } else {
                            current_cmd = SolvingDirections::GoBackHistory;
                        }
                    } else {
                        // for some reason there's no history item, so try this again with another piece starting off.
                        // the only reason I can think of is that the last round had exhausted every possibility,
                        // which makes no sense. But I think that so far the logic works 99% of the time, so since so
                        // far this date is an outlier I'll just try this weird other measure.
                        at_piece_id = PieceId {
                            piece_number: 7,
                            variation_number: 7,
                        };
                        at_index = 0;
                        current_cmd = SolvingDirections::SetPiece;

                        println!(
                            "had to start over again with a different piece at count {}",
                            self.counter
                        );
                        println!("{}", self);
                        println!("history items: {:?}", self.history);
                        for (index, piece) in self.all_pieces.0.iter().enumerate() {
                            println!("piece number {}'s availability: {}", index, piece.available);
                        }

                        panic!();
                    }
                }
            }

            if self.counter >= i64::MAX {
                break;
            }
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut return_string: String = "".to_string();
        for (i, sec) in self.debug_board.iter().enumerate() {
            match sec {
                BoardSquare::Empty => return_string += " ",
                BoardSquare::Forbidden => return_string += "X",
                BoardSquare::Filled(item) => return_string += &format!("{}", item.piece_number),
            }

            if i != 0 && (i + 1) % 7 == 0 {
                return_string += "\n";
            }
        }

        write!(f, "{}", return_string)
    }
}
