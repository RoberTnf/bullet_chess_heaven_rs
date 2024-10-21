use crate::board::position::BoardPosition;
use bevy::utils::HashSet;

pub enum MovementType {
    WhitePawn,
    BlackPawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Default)]
pub struct MovementResponse {
    pub valid_moves: Vec<BoardPosition>,
    pub valid_attacks: Vec<BoardPosition>,
}

impl MovementType {
    pub fn get_valid_moves(
        &self,
        position: &BoardPosition,
        other_pieces_positions: &HashSet<BoardPosition>,
        enemies_positions: &HashSet<BoardPosition>,
    ) -> MovementResponse {
        let mut valid_moves = Vec::new();
        let mut valid_attacks = Vec::new();

        match self {
            MovementType::WhitePawn => {
                self.pawn_moves(
                    position,
                    other_pieces_positions,
                    enemies_positions,
                    1,
                    &mut valid_moves,
                    &mut valid_attacks,
                );
            }
            MovementType::BlackPawn => {
                self.pawn_moves(
                    position,
                    other_pieces_positions,
                    enemies_positions,
                    -1,
                    &mut valid_moves,
                    &mut valid_attacks,
                );
            }
            MovementType::Knight => {
                self.knight_moves(
                    position,
                    other_pieces_positions,
                    enemies_positions,
                    &mut valid_moves,
                    &mut valid_attacks,
                );
            }
            MovementType::Bishop => {
                self.diagonal_moves(
                    position,
                    other_pieces_positions,
                    enemies_positions,
                    &mut valid_moves,
                    &mut valid_attacks,
                );
            }
            MovementType::Rook => {
                self.straight_moves(
                    position,
                    other_pieces_positions,
                    enemies_positions,
                    &mut valid_moves,
                    &mut valid_attacks,
                );
            }
            MovementType::Queen => {
                self.diagonal_moves(
                    position,
                    other_pieces_positions,
                    enemies_positions,
                    &mut valid_moves,
                    &mut valid_attacks,
                );
                self.straight_moves(
                    position,
                    other_pieces_positions,
                    enemies_positions,
                    &mut valid_moves,
                    &mut valid_attacks,
                );
            }
            MovementType::King => {
                self.king_moves(
                    position,
                    other_pieces_positions,
                    enemies_positions,
                    &mut valid_moves,
                    &mut valid_attacks,
                );
            }
        }

        MovementResponse {
            valid_moves,
            valid_attacks,
        }
    }

    fn pawn_moves(
        &self,
        position: &BoardPosition,
        other_pieces_positions: &HashSet<BoardPosition>,
        enemies_positions: &HashSet<BoardPosition>,
        direction: i32,
        valid_moves: &mut Vec<BoardPosition>,
        valid_attacks: &mut Vec<BoardPosition>,
    ) {
        if let Some(possible_move) = BoardPosition::new(position.x, position.y + direction) {
            if !other_pieces_positions.contains(&possible_move) {
                valid_moves.push(possible_move);
            }
        }

        let possible_attacks = vec![
            BoardPosition::new(position.x - 1, position.y + direction),
            BoardPosition::new(position.x + 1, position.y + direction),
        ];
        for attack in possible_attacks {
            if let Some(attack) = attack {
                if enemies_positions.contains(&attack) {
                    valid_attacks.push(attack);
                }
            }
        }
    }

    fn knight_moves(
        &self,
        position: &BoardPosition,
        other_pieces_positions: &HashSet<BoardPosition>,
        enemies_positions: &HashSet<BoardPosition>,
        valid_moves: &mut Vec<BoardPosition>,
        valid_attacks: &mut Vec<BoardPosition>,
    ) {
        let possible_moves = vec![
            BoardPosition::new(position.x + 1, position.y + 2),
            BoardPosition::new(position.x + 2, position.y + 1),
            BoardPosition::new(position.x + 2, position.y - 1),
            BoardPosition::new(position.x + 1, position.y - 2),
            BoardPosition::new(position.x - 1, position.y - 2),
            BoardPosition::new(position.x - 2, position.y - 1),
            BoardPosition::new(position.x - 2, position.y + 1),
            BoardPosition::new(position.x - 1, position.y + 2),
        ];

        for move_pos in possible_moves {
            if let Some(move_pos) = move_pos {
                if enemies_positions.contains(&move_pos) {
                    valid_attacks.push(move_pos);
                } else if !other_pieces_positions.contains(&move_pos) {
                    valid_moves.push(move_pos);
                }
            }
        }
    }

    fn diagonal_moves(
        &self,
        position: &BoardPosition,
        other_pieces_positions: &HashSet<BoardPosition>,
        enemies_positions: &HashSet<BoardPosition>,
        valid_moves: &mut Vec<BoardPosition>,
        valid_attacks: &mut Vec<BoardPosition>,
    ) {
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.line_moves(
            position,
            other_pieces_positions,
            enemies_positions,
            valid_moves,
            valid_attacks,
            &directions,
        );
    }

    fn straight_moves(
        &self,
        position: &BoardPosition,
        other_pieces_positions: &HashSet<BoardPosition>,
        enemies_positions: &HashSet<BoardPosition>,
        valid_moves: &mut Vec<BoardPosition>,
        valid_attacks: &mut Vec<BoardPosition>,
    ) {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        self.line_moves(
            position,
            other_pieces_positions,
            enemies_positions,
            valid_moves,
            valid_attacks,
            &directions,
        );
    }

    fn line_moves(
        &self,
        position: &BoardPosition,
        other_pieces_positions: &HashSet<BoardPosition>,
        enemies_positions: &HashSet<BoardPosition>,
        valid_moves: &mut Vec<BoardPosition>,
        valid_attacks: &mut Vec<BoardPosition>,
        directions: &[(i32, i32)],
    ) {
        for &(dx, dy) in directions {
            let mut x = position.x + dx;
            let mut y = position.y + dy;

            while let Some(new_pos) = BoardPosition::new(x, y) {
                if enemies_positions.contains(&new_pos) {
                    valid_attacks.push(new_pos);
                    break;
                } else if other_pieces_positions.contains(&new_pos) {
                    break;
                } else {
                    valid_moves.push(new_pos);
                }
                x += dx;
                y += dy;
            }
        }
    }

    fn king_moves(
        &self,
        position: &BoardPosition,
        other_pieces_positions: &HashSet<BoardPosition>,
        enemies_positions: &HashSet<BoardPosition>,
        valid_moves: &mut Vec<BoardPosition>,
        valid_attacks: &mut Vec<BoardPosition>,
    ) {
        let directions = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        for &(dx, dy) in &directions {
            if let Some(new_pos) = BoardPosition::new(position.x + dx, position.y + dy) {
                if enemies_positions.contains(&new_pos) {
                    valid_attacks.push(new_pos);
                } else if !other_pieces_positions.contains(&new_pos) {
                    valid_moves.push(new_pos);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::common::MovementTypes;

    use super::*;

    #[test]
    fn test_valid_moves_for_king() {
        // Setup a mock board state
        // Test king movement in various scenarios
        let king_position = BoardPosition::new(4, 4).unwrap();
        let other_positions = HashSet::from_iter(vec![
            BoardPosition::new(3, 3).unwrap(),
            BoardPosition::new(5, 5).unwrap(),
        ]);
        let enemies_positions = HashSet::from_iter(vec![BoardPosition::new(5, 5).unwrap()]);
        let movement_types = MovementTypes(vec![MovementType::King]);
        let valid_moves = movement_types.0[0].get_valid_moves(
            &king_position,
            &other_positions,
            &enemies_positions,
        );
        assert_eq!(valid_moves.valid_moves.len(), 6);
        assert_eq!(valid_moves.valid_attacks.len(), 1);
    }

    #[test]
    fn test_valid_moves_for_pawn() {
        // Test pawn movement in various scenarios
        let pawn_position = BoardPosition::new(4, 4).unwrap();
        let other_positions = HashSet::from_iter(vec![
            BoardPosition::new(4, 7).unwrap(),
            BoardPosition::new(5, 5).unwrap(),
        ]);
        let enemies_positions = HashSet::from_iter(vec![BoardPosition::new(5, 5).unwrap()]);
        let movement_types = MovementTypes(vec![MovementType::WhitePawn]);
        let valid_moves = movement_types.0[0].get_valid_moves(
            &pawn_position,
            &other_positions,
            &enemies_positions,
        );
        assert_eq!(valid_moves.valid_moves.len(), 1);
        assert_eq!(valid_moves.valid_attacks.len(), 1);
        assert_eq!(
            valid_moves.valid_moves[0],
            BoardPosition::new(4, 5).unwrap()
        );
        assert_eq!(
            valid_moves.valid_attacks[0],
            BoardPosition::new(5, 5).unwrap()
        );
    }

    #[test]
    fn test_collision_detection() {
        // Setup a board with multiple pieces
        // Ensure pieces can't move through each other
    }
}
