
extern crate pokemon_game;

// use reqwest;
use pokemon_game::model::pokemon::Pokemon;

#[tokio::main]
async fn main() {
    let mypoke: Pokemon = match pokemon_game::pokemon::pokemon::get_by_id(25).await {
        Ok(poke) => poke,
        Err(why) => panic!("{:?}", why),
    };

    println!("Pokemon: {:?}", mypoke.name);
    println!("id: {:?}", mypoke.id);
    println!("Type: {:?}", mypoke.types);
    println!("Base stats: {:?}", mypoke.stats);
}

// let my_pikachu = Pokemon {
//     nickname: String::from("Bolt"),
//     level: 78,
//     base: pikachu,
//     iv: Stats {
//         hp: 24,
//         attack: 12,
//         defence: 30,
//         special_attack: 16,
//         special_defence: 23,
//         speed: 5,
//     },
//     ev: Stats {
//         hp: 74,
//         attack: 190,
//         defence: 44,
//         special_attack: 22,
//         special_defence: 67,
//         speed: 90,
//     },
// };
