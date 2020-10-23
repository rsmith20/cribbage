mod card;

fn main() {
    //test();
    loop {
        cribbage::play_game();
    }
}

mod cribbage {

    pub struct Player {
        is_cpu: bool,
        name: String,
        points: u8,
    }

    pub struct Game {
        player1: Player,
        player2: Player,
        player1_next_dealer: bool,
        player1_started_deal: bool,
        deck: super::card::deck::Deck,
    }

    impl Game {
        fn new() -> Self {
            use std::io;
            println!("Enter first player's name: ");
            let mut player1_name = String::new();
            match io::stdin().read_line(&mut player1_name) {
                Ok(_n) => (),
                Err(error) => panic!("error: {}", error),
            }

            println!("First player's name is: {}", player1_name.trim());

            println!("Enter second player's name: ");
            let mut player2_name = String::new();
            match io::stdin().read_line(&mut player2_name) {
                Ok(_n) => (),
                Err(error) => panic!("error: {}", error),
            }
            Game {
                player1: Player {
                    is_cpu: false,
                    name: player1_name,
                    points: 0,
                },
                player2: Player {
                    is_cpu: false,
                    name: player2_name,
                    points: 0,
                },
                player1_next_dealer: true,
                player1_started_deal: true,
                deck: super::card::deck::new(),
            }
        }
        fn deal(&mut self) {
        	self.deck.shuffle();
        	
        }
    }

    pub fn play_game() {
        let mut game = Game::new();
        //setup::select_first_player();
        loop {
        	game.deal();
            game_play::deal();
            game_play::choose_crib();

            //All of these could trigger end of game
            game_play::cut();
            game_play::play_hand();
            game_play::score_pone();
            game_play::score_dealer();
            game_play::score_crib();
        }
    }

    mod setup {

        pub fn select_first_player() {}
    }

    mod game_play {

        pub fn deal() {}

        pub fn choose_crib() {}

        pub fn cut() {}
        pub fn play_hand() {}
        pub fn score_hand() {}
        pub fn score_pone() {}
        pub fn score_dealer() {}
        pub fn score_crib() {}
    }
}

fn test() {
    let card = card::card::Card {
        suit: card::card::Suit::Hearts,
        value: card::card::Value::Ten,
    };
    println!("{}", card.to_string());

    let mut deck = card::deck::new();
    println!("This is the deck: {:?}.", deck);
    deck.shuffle();
    println!("This is the shuffled deck.");
    deck.pretty_print();
}
