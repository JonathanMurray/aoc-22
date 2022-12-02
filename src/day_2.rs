fn parse_opponent_move(opponent: char) -> Shape {
    match opponent {
        'A' => Shape::rock(),
        'B' => Shape::papers(),
        'C' => Shape::scissors(),
        _ => panic!("Invalid opponent move"),
    }
}

const VICTORY_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOSS_SCORE: u32 = 0;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Shape(u8);

impl Shape {
    const NUM_SHAPES: u8 = 3;

    fn rock() -> Self {
        Self(0)
    }

    fn papers() -> Self {
        Self(1)
    }

    fn scissors() -> Self {
        Self(2)
    }

    fn base_score(&self) -> u32 {
        (self.0 + 1) as u32
    }

    fn wins_against(&self) -> Shape {
        Shape((self.0 + Self::NUM_SHAPES - 1) % Self::NUM_SHAPES)
    }

    fn loses_against(&self) -> Shape {
        Shape((self.0 + 1) % Self::NUM_SHAPES)
    }

    fn compete_against(&self, opponent: Self) -> u32 {
        if self.wins_against() == opponent {
            VICTORY_SCORE
        } else if self.loses_against() == opponent {
            LOSS_SCORE
        } else {
            DRAW_SCORE
        }
    }
}

#[allow(dead_code)]
pub fn puzzle_2_predict_score_from_recommended_moves(
    input: impl Iterator<Item = impl Into<String>>,
) -> u32 {
    let mut total_score = 0;
    for line in input {
        let line = line.into();
        let chars = line.chars().into_iter().collect::<Vec<char>>();
        if let [opponent, ' ', you] = chars[..] {
            let opponent = parse_opponent_move(opponent);
            let you = match you {
                'X' => Shape::rock(),
                'Y' => Shape::papers(),
                'Z' => Shape::scissors(),
                _ => panic!("Invalid response"),
            };
            total_score += you.base_score() + you.compete_against(opponent);
        } else {
            panic!("Malformed line: {}", line);
        }
    }
    total_score
}

pub fn puzzle_2_predict_score_from_recommended_outcome(
    input: impl Iterator<Item = impl Into<String>>,
) -> u32 {
    let mut total_score = 0;
    for line in input {
        let line = line.into();
        let chars = line.chars().into_iter().collect::<Vec<char>>();
        if let [opponent, ' ', outcome] = chars[..] {
            let opponent = parse_opponent_move(opponent);
            let (your_move, outcome_score) = match outcome {
                'X' => (opponent.wins_against(), LOSS_SCORE),
                'Y' => (opponent, DRAW_SCORE),
                'Z' => (opponent.loses_against(), VICTORY_SCORE),
                _ => panic!("Invalid outcome"),
            };
            total_score += your_move.base_score() + outcome_score;
        } else {
            panic!("Malformed line: {}", line);
        }
    }
    total_score
}
