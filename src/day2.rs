use self::Hand::*;
use self::HandResult::*;
 

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors
}

pub trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

#[derive(Debug)]
pub enum HandResult {
    Win,
    Draw,
    Lose
}

pub fn play(opp: Hand, you: Hand) -> HandResult {
    let (opp_beats, you_beat) = (opp.beats(), you.beats());

    match (opp_beats, you_beat) {
        _ if you_beat == opp => Win,
        _ if opp_beats == you => Lose,
        _ => Draw
    }
}

pub fn compute_score(you: &Hand, game: HandResult) -> usize {
    let handval = match you {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    };

    handval + match game {
        Win => 6,
        Draw => 3,
        Lose => 0
    }
}

fn parse_to_hand(letter: &str) -> Result<Hand, String> {
    match letter {
        "A" | "X" => Ok(Rock),
        "B" | "Y" => Ok(Paper),
        "C" | "Z" => Ok(Scissors),
        _ => Err(format!("Invalid Input"))

    }
}

fn parse_to_outcome(letter: &str, opp: Hand) -> Result<Hand, String> {
    match letter {
        "X" => Ok(opp.beats()),
        "Z" => Ok(opp.beats().beats()),
        "Y" => Ok(opp),
        _ => Err(format!("Invalid Input"))

    }
}

pub fn process_line(line: &str, outcome_flag: bool) -> (Hand, Hand) {

    let mut splitline = line.split(' ');
    let opponent = parse_to_hand(splitline.next().unwrap()).unwrap();

    if outcome_flag {
        let letter = splitline.next().unwrap();
        let you = parse_to_outcome(letter, opponent).unwrap();
        (opponent, you)
    }

    else {
        let you = parse_to_hand(splitline.next().unwrap()).unwrap();
        (opponent, you)
    }
}

pub fn get_scores(data: String, flag: bool) -> usize {
    data.lines()
        .map(|line| process_line(line, flag))
            .map(|(opp, you)| compute_score(&you, play(opp, you)))
                .sum()
}