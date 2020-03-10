extern crate rand;

use rand::Rng;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
enum Suit {
    Nil,
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    fn iter() -> Iter<'static, Suit> {
        use Suit::*;
        static SUITS: [Suit; 4] = [Hearts, Diamonds, Clubs, Spades];
        SUITS.iter()
    }
}

#[derive(Debug, Clone, Copy)]
enum Number {
    Nil,
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

impl Number {
    fn iter() -> Iter<'static, Number> {
        use Number::*;
        static NUMBERS: [Number; 12] = [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Jack, Queen, King,
        ];
        NUMBERS.iter()
    }
}

#[derive(Debug)]
struct Solitaire {
    display: Display,
    deck: Vec<Card>,
    piles: Vec<Vec<Card>>,
    foundations: Vec<Vec<Card>>,
}

impl Solitaire {
    fn new(display: Display) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for suit in Suit::iter() {
            for number in Number::iter() {
                cards.push(Card(suit.clone(), number.clone()));
            }
        }

        Self {
            display,
            deck: cards,
            piles: Vec::new(),
            foundations: Vec::new(),
        }
    }

    fn shuffle(&mut self) {
        let mut pools: Vec<Vec<Card>> = Vec::new();
        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen_range(6, 13);
        for _i in 0..n {
            pools.push(Vec::new());
        }

        for i in 0..self.deck.len() {
            let pool = rng.gen_range(0, n);
            pools[pool as usize].push(self.deck[i]);
        }

        self.deck = Vec::new();

        for pool in pools {
            for card in pool {
                self.deck.push(card);
            }
        }
    }

    fn deal_piles(&mut self) {
        for i in 0..7 {
            let mut pile: Vec<Card> = Vec::new();
            for _j in 0..i + 1 {
                pile.push(self.deck.pop().unwrap());
            }
            self.piles.push(pile);
        }
    }

    fn print(&mut self) {
        let deck_space = Card(Suit::Nil, Number::Nil);
        deck_space.print_card(&mut self.display, 1, 1);
        let piles_start_x = 1;
        let piles_start_y = 5;
        for i in 0..self.piles.len() {
            let x: u32 = piles_start_x + (i as u32 * 5);
            for j in 0..self.piles[i].len() {
                self.piles[i][j].print_card(&mut self.display, x, piles_start_y + (j as u32 * 2))
            }
        }
        self.display.print();
    }
}

#[test]
fn solitaire_test() {
    let mut display = Display::new(80, 20);
    let mut solitaire = Solitaire::new(display);
    solitaire.shuffle();
    solitaire.deal_piles();
    solitaire.print();
    assert!(false);
}

#[derive(Debug, Clone, Copy)]
struct Card(Suit, Number);

impl Card {
    fn get_printable_number(&self, number: &Number) -> char {
        use Number::*;
        match number {
            Nil => ' ',
            Ace => 'A',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
        }
    }

    fn get_printable_suit(&self, suit: &Suit) -> char {
        use Suit::*;
        match suit {
            Nil => ' ',
            Hearts => '♥',
            Diamonds => '♦',
            Clubs => '♣',
            Spades => '♠',
        }
    }

    fn draw_card(&self) -> Vec<char> {
        let Card(suit, number) = self;
        let mut buffer: Vec<char> = Vec::new();
        buffer.push('╔');
        buffer.push('═');
        buffer.push('═');
        buffer.push('╗');
        buffer.push('║');
        buffer.push(self.get_printable_number(number));
        buffer.push(self.get_printable_suit(suit));
        buffer.push('║');
        buffer.push('╚');
        buffer.push('═');
        buffer.push('═');
        buffer.push('╝');
        return buffer;
    }

    fn print_card(&self, display: &mut Display, x: u32, y: u32) {
        let drawable_card = self.draw_card();
        let (width, height) = display.dimensions;
        let first_x = x;
        let mut x = x;
        let mut y = y;
        for i in 0..drawable_card.len() {
            display.buffer[((y * width) + x) as usize] = drawable_card[i];
            x += 1;
            if (i + 1) % 4 == 0 && i != 0 {
                y += 1;
                x = first_x;
            }
        }
    }
}

#[derive(Debug)]
struct Display {
    buffer: Vec<char>,
    dimensions: (u32, u32),
}

impl Display {
    fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: vec!['*'; (width * height) as usize],
            dimensions: (width, height),
        }
    }

    fn print(&self) {
        let (width, height) = self.dimensions;

        let mut y_count = 0;
        let mut x_count = 0;

        for c in &self.buffer {
            print!("{}", c);
            x_count += 1;
            if x_count % width == 0 {
                y_count += 1;
                if y_count > height {
                    panic!("this shouldn't happen");
                }
                println!("");
            }
        }
        println!("");
    }
}

#[test]
fn draw_card_test() {
    let mut display = Display::new(10, 10);
    let card = Card(Suit::Clubs, Number::Five);
    display.print();
    card.print_card(&mut display, 0, 0);
    display.print();
    assert!(false);
}
