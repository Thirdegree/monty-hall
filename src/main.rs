use std::thread;

use rand::{seq::IteratorRandom, thread_rng, Rng};

#[derive(Clone)]
enum Door {
    Empty,
    Goat,
}

struct Game {
    doors: Vec<Door>,
    goat: usize,
}

impl Game {
    fn new<R: Rng + Sized>(n_doors: usize, rng: &mut R) -> Self {
        assert!(n_doors > 1);
        let goat = (0..n_doors).choose(rng).unwrap();
        let mut doors = vec![Door::Empty; n_doors];
        doors[goat] = Door::Goat;
        Game { doors, goat }
    }

    fn guess<R: Rng + Sized>(&self, mut guess: usize, accept_other: bool, rng: &mut R) -> bool {
        if accept_other {
            guess = if guess == self.goat {
                // if they got it right, pick a random other door to leave closed
                (0..guess)
                    .chain(guess + 1..self.doors.len())
                    .choose(rng)
                    .unwrap()
            } else {
                // we can't open the goat door, so that has to be the one they get as "other"
                self.goat
            };
        }
        matches!(self.doors[guess], Door::Goat)
    }
}

fn main() {
    let games = u32::MAX;
    let n_threads = 11;
    let chunk_size = games / n_threads;
    let mut handles = Vec::new();
    let num_doors = 3;
    for _ in 0..n_threads {
        handles.push(thread::spawn(move || {
            let mut wins = 0;
            let mut rng = thread_rng();
            for _ in 0..chunk_size {
                let game = Game::new(num_doors, &mut rng);
                if game.guess((0..num_doors).choose(&mut rng).unwrap(), true, &mut rng) {
                    wins += 1;
                }
            }
            wins
        }));
    }
    let wins: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("Win %: {:.3}%", (wins as f32 / games as f32) * 100.);
}
