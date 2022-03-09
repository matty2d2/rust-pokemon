
extern crate pokemon_game;

use pokemon_game::pokemon::base::PokemonBase;
use pokemon_game::pokemon::base::sprite::PokemonSprite;
use pokemon_game::pokemon::stats::Stats;
use pokemon_game::pokemon::Pokemon;
use pokemon_game::pokemon_type::PokemonType;
use pokemon_game::pokemon_type::damage_relations::DamageRelations;

use reqwest;
// use std::collections::HashMap;

#[tokio::main]
async fn main() {
    
    let pikachu = PokemonBase {
        id: 25,
        name: String::from("Pikachu"),
        sprite: PokemonSprite {
            back_default: String::from("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/back/25.png"),
            back_female: String::from("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/back/female/25.png"),
            back_shiny: String::from("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/back/shiny/25.png"),
            back_shiny_female: String::from("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/back/shiny/female/25.png"),
            front_default: String::from("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/25.png"),
            front_female: String::from("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/female/25.png"),
            front_shiny: String::from("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/shiny/25.png"),
            front_shiny_female: String::from("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/shiny/female/25.png"),
        },
        stats: Stats {
            hp: 108,
            attack: 55,
            defence: 40,
            special_attack: 50,
            special_defence: 50,
            speed: 90,
        },
        experience: 112,
        type_1: PokemonType {
            id: 1,
            name: String::from("Electric"),
            damage_relations: DamageRelations {
                double_damage_from: None,
                double_damage_to: None,
                half_damage_from: None,
                half_damage_to: None,
                no_damage_from: None,
                no_damage_to: None,
            }
        },
        type_2: None,
    };

    let my_pikachu = Pokemon {
        nickname: String::from("Bolt"),
        level: 78,
        base: pikachu,
        iv: Stats {
            hp: 24,
            attack: 12,
            defence: 30,
            special_attack: 16,
            special_defence: 23,
            speed: 5,
        },
        ev: Stats {
            hp: 74,
            attack: 190,
            defence: 44,
            special_attack: 22,
            special_defence: 67,
            speed: 90,
        },
    };

    println!("Pokemon: {:?}", my_pikachu.base.name);
    println!("Type: {:?}", my_pikachu.base.type_1.name);
    println!("Nickname: {:?}", my_pikachu.nickname);
    println!("Level: {:?}", my_pikachu.level);
    println!("HP: {:?}", my_pikachu.hp());
    println!("Attack: {:?}", my_pikachu.attack());
    println!("Defence: {:?}", my_pikachu.defence());
    println!("Special attack: {:?}", my_pikachu.special_attack());
    println!("Special defence: {:?}", my_pikachu.special_defence());
    println!("Speed: {:?}", my_pikachu.speed());
    println!("Experience: {:?}", my_pikachu.base.experience);

    // let conn = Connection::open("cats.db")?;

    // conn.execute(
    //     "create table if not exists cat_colors (
    //          id integer primary key,
    //          name text not null unique
    //      )",
    //     NO_PARAMS,
    // )?;
    // conn.execute(
    //     "create table if not exists cats (
    //          id integer primary key,
    //          name text not null,
    //          color_id integer not null references cat_colors(id)
    //      )",
    //     NO_PARAMS,
    // )?;
    // Ok(())

}
