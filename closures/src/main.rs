pub mod state;
use state::*;
fn main() {
    println!("Hello, world!");
    let africans = Some(Race::White);
    get_race(africans);
}

fn get_race(race: Option<Race>) {
    let user_race = race.unwrap_or_else(|| Race::Black);
    println!("{:#?}", user_race);
}
