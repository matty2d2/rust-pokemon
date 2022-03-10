//! Pokemon endpoints group

/// Abilities provide passive effects for Pokémon in battle or in the overworld.
/// Pokémon have multiple possible abilities but can have only one ability at a time.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Ability) for greater detail.
pub mod ability {
    crate::endpoint!(crate::model::pokemon::Ability; for "ability");
}

/// Characteristics indicate which stat contains a Pokémon's highest IV.
/// A Pokémon's Characteristic is determined by the remainder of its highest IV divided by 5 (gene_modulo).
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Characteristic) for greater detail.
pub mod characteristic {
    crate::endpoint!(crate::model::pokemon::Characteristic; for "characteristic");
}


/// Genders were introduced in Generation II for the purposes of breeding Pokémon but can also result
/// in visual differences or even different evolutionary lines.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Gender) for greater detail.
pub mod gender {
    crate::endpoint!(crate::model::pokemon::Gender; for "gender");
}

/// Growth rates are the speed with which Pokémon gain levels through experience.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Experience) for greater detail.
pub mod growth_rate {
    crate::endpoint!(crate::model::pokemon::GrowthRate; for "growth-rate");
}

/// Natures influence how a Pokémon's stats grow. See [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Nature) for greater detail.
pub mod nature {
    crate::endpoint!(crate::model::pokemon::Nature; for "nature");
}

/// Pokémon are the creatures that inhabit the world of the Pokémon games.
/// They can be caught using Pokéballs and trained by battling with other Pokémon.
/// Each Pokémon belongs to a specific species but may take on a variant which makes it differ
/// from other Pokémon of the same species, such as base stats, available abilities and typings.
/// See [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Pokémon_(species)) for greater detail.
pub mod pokemon {
    crate::endpoint!(crate::model::pokemon::Pokemon; for "pokemon");
}

/// Stats determine certain aspects of battles. Each Pokémon has a value for each stat which
/// grows as they gain levels and can be altered momentarily by effects in battles.
pub mod stat {
    crate::endpoint!(crate::model::pokemon::Stat; for "stat");
}

/// Types are properties for Pokémon and their moves. Each type has three properties:
/// which types of Pokémon it is super effective against, which types of Pokémon it is not very effective against,
/// and which types of Pokémon it is completely ineffective against.
pub mod type_ {
    crate::endpoint!(crate::model::pokemon::Type; for "type");
}