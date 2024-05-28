use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};
use std::io::BufReader;
use std::io::stdin;
use std::fs::File;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
struct FlashCard {
    question: String,
    answer: String
}

#[derive(Serialize, Deserialize)]
struct Deck {
    title: String,
    cards: Vec<FlashCard>
}

impl Deck {
    fn quiz(&mut self) {
        self.shuffle_deck();

        for card in &self.cards {
            let possible_answers: Vec<FlashCard> = self.get_possible_answer_cards(3, card);

            println!("{}", card.question);
            println!("\n");
            println!("A - {}", possible_answers[0].answer);
            println!("B - {}", possible_answers[1].answer);
            println!("C - {}", possible_answers[2].answer);
            println!("D - {}", possible_answers[3].answer);

            let mut user_input: String = String::new();
            stdin().read_line(&mut user_input).expect("Failed to read line.");

            match user_input.trim().to_lowercase().as_str() {
                "a" => check_answer(&card, &possible_answers[0]),
                "b" => check_answer(&card, &possible_answers[1]),
                "c" => check_answer(&card, &possible_answers[2]),
                "d" => check_answer(&card, &possible_answers[3]),
                _ => println!("{}is not a valid input.", user_input)
            }
        }
    }

    fn get_possible_answer_cards (&self, choose_num: usize, correct_card: &FlashCard) -> Vec<FlashCard> {
        let mut rngen = thread_rng();
        let mut random_cards = Vec::new();

        while (random_cards.len() as usize) < choose_num {
            let random_index = rngen.gen_range(0..self.cards.len());
            let random_card: FlashCard = self.cards[random_index].clone();

            if random_card != *correct_card {
                random_cards.push(random_card);
            } else {/* Ignore */}
        }

        random_cards.push(correct_card.clone());
        random_cards.shuffle(&mut rngen);
        random_cards
    }

    fn shuffle_deck(&mut self) {
        let mut rngen = thread_rng();
        self.cards.shuffle(&mut rngen);
    }
}

fn check_answer(correct_answer: &FlashCard, user_answer: &FlashCard) {
    if user_answer == correct_answer {
        println!("Correct!");
    } else {
        println!("Incorrect. The correct answer is {}", correct_answer.answer);
    }
}

fn main() {
    let file = File::open("decks.json").expect("What made you think that was a good idea?");
    let reader = BufReader::new(file);

    let mut deserialized: Deck = serde_json::from_reader(reader).unwrap();
    deserialized.quiz();
}
