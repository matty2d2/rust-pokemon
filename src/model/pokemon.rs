//! Pokemon group models

use super::resource::{
    ApiResource, Description, Name, NamedApiResource,
    VerboseEffect,
};

/// [Ability official documentation](https://pokeapi.co/docs/v2#ability)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Ability {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The generation this ability originated in.
    pub generation: Option<NamedApiResource>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// The effect of this ability listed in different languages.
    pub effect_entries: Option<Vec<VerboseEffect>>,
    /// The flavor text of this ability listed in different languages.
    pub flavor_text_entries: Option<Vec<AbilityFlavorText>>,
    /// A list of Pokémon that could potentially have this ability.
    pub pokemon: Option<Vec<AbilityPokemon>>,
}

/// [AbilityFlavorText official documentation](https://pokeapi.co/docs/v2#abilityflavortext)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct AbilityFlavorText {
    /// The localized name for an API resource in a specific language.
    pub flavor_text: Option<String>,
    /// The language this text resource is in.
    pub language: Option<NamedApiResource>,
    /// The version group that uses this flavor text.
    pub version_group: Option<NamedApiResource>,
}

/// [AbilityPokemon official documentation](https://pokeapi.co/docs/v2#abilitypokemon)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct AbilityPokemon {
    /// Whether or not this a hidden ability for the referenced Pokémon.
    pub is_hidden: Option<bool>,
    /// Pokémon have 3 ability 'slots' which hold references to possible abilities they could have.
    /// This is the slot of this ability for the referenced pokemon.
    pub slot: Option<i64>,
    /// The Pokémon this ability could belong to.
    pub pokemon: Option<NamedApiResource>,
}

/// [Characteristic official documentation](https://pokeapi.co/docs/v2#characteristic)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Characteristic {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The remainder of the highest stat/IV divided by 5.
    pub gene_modulo: Option<i64>,
    /// The possible values of the highest stat that would result in a Pokémon
    /// receiving this characteristic when divided by 5.
    pub possible_values: Option<Vec<i64>>,
    /// The description of this resource listed in different languages.
    pub descriptions: Option<Vec<Description>>,
    /// The highest stat referenced by this characteristic.
    pub highest_stat: Option<NamedApiResource>,
}

/// [Gender official documentation](https://pokeapi.co/docs/v2#gender)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Gender {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of Pokémon species that can be this gender and how likely it is that they will be.
    pub pokemon_species_details: Option<Vec<PokemonSpeciesGender>>,
    /// A list of Pokémon species that required this gender in order for a Pokémon to evolve into them.
    pub required_for_evolution: Option<Vec<NamedApiResource>>,
}

/// [PokemonSpeciesGender official documentation](https://pokeapi.co/docs/v2#pokemonspeciesgender)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonSpeciesGender {
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub rate: Option<i64>,
    /// A Pokémon species that can be the referenced gender.
    pub pokemon_species: Option<NamedApiResource>,
}

/// [GrowthRate official documentation](https://pokeapi.co/docs/v2#growthrate)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GrowthRate {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The formula used to calculate the rate at which the Pokémon species gains level.
    pub formula: Option<String>,
    /// The descriptions of this characteristic listed in different languages.
    pub descriptions: Option<Vec<Description>>,
    /// A list of levels and the amount of experienced needed to atain them based on this growth rate.
    pub levels: Option<Vec<GrowthRateExperienceLevel>>,
    /// A list of Pokémon species that gain levels at this growth rate.
    pub pokemon_species: Option<Vec<NamedApiResource>>,
}

/// [GrowthRateExperienceLevel official documentation](https://pokeapi.co/docs/v2#growthrateexperiencelevel)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GrowthRateExperienceLevel {
    /// The level gained.
    pub level: Option<i64>,
    /// The amount of experience required to reach the referenced level.
    pub experience: Option<i64>,
}

/// [Nature official documentation](https://pokeapi.co/docs/v2#nature)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Nature {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The stat decreased by 10% in Pokémon with this nature.
    pub decreased_stat: Option<NamedApiResource>,
    /// The stat increased by 10% in Pokémon with this nature.
    pub increased_stat: Option<NamedApiResource>,
    /// The flavor hated by Pokémon with this nature.
    pub hates_flavor: Option<NamedApiResource>,
    /// The flavor liked by Pokémon with this nature.
    pub likes_flavor: Option<NamedApiResource>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [Pokemon official documentation](https://pokeapi.co/docs/v2#pokemon)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Pokemon {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The base experience gained for defeating this Pokémon.
    pub base_experience: Option<i64>,
    /// A list of abilities this Pokémon could potentially have.
    pub abilities: Option<Vec<PokemonAbility>>,
    /// A list of items this Pokémon may be holding when encountered.
    pub held_items: Option<Vec<PokemonHeldItem>>,
    /// A list of moves along with learn methods and level details pertaining to specific version groups.
    pub moves: Option<Vec<PokemonMove>>,
    /// A set of sprites used to depict this Pokémon in the game.
    /// A visual representation of the various sprites can be found at [PokeAPI/sprites](https://github.com/PokeAPI/sprites).
    pub sprites: Option<PokemonSprites>,
    /// The species this Pokémon belongs to.
    pub species: Option<NamedApiResource>,
    /// A list of base stat values for this Pokémon.
    pub stats: Option<Vec<PokemonStat>>,
    /// A list of details showing types this Pokémon has.
    pub types: Option<Vec<PokemonType>>,
}

/// [PokemonAbility official documentation](https://pokeapi.co/docs/v2#pokemonability)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonAbility {
    /// Whether or not this is a hidden ability.
    pub is_hidden: Option<bool>,
    /// The slot this ability occupies in this Pokémon species.
    pub slot: Option<i64>,
    /// The ability the Pokémon may have.
    pub ability: Option<NamedApiResource>,
}

/// [PokemonType official documentation](https://pokeapi.co/docs/v2#pokemontype)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonType {
    /// The order the Pokémon's types are listed in.
    pub slot: Option<i64>,
    /// The type the referenced Pokémon has.
    #[serde(rename = "type")]
    pub type_: Option<NamedApiResource>,
}

/// [PokemonHeldItem official documentation](https://pokeapi.co/docs/v2#pokemonhelditem)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonHeldItem {
    /// The item the referenced Pokémon holds.
    pub item: Option<NamedApiResource>,
}

/// [PokemonMove official documentation](https://pokeapi.co/docs/v2#pokemonmove)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonMove {
    /// The move the Pokémon can learn.
    #[serde(rename = "move")]
    pub move_: Option<NamedApiResource>,
}

/// [PokemonStat official documentation](https://pokeapi.co/docs/v2#pokemonstat)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonStat {
    /// The stat the Pokémon has.
    pub stat: Option<NamedApiResource>,
    /// The effort points (EV) the Pokémon has in the stat.
    pub effort: Option<i64>,
    /// The base value of the stat.
    pub base_stat: Option<i64>,
}

/// [PokemonSprites official documentation](https://pokeapi.co/docs/v2#pokemonsprites)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonSprites {
    /// The default depiction of this Pokémon from the front in battle.
    pub front_default: Option<String>,
    /// The shiny depiction of this Pokémon from the front in battle.
    pub front_shiny: Option<String>,
    /// The female depiction of this Pokémon from the front in battle.
    pub front_female: Option<String>,
    /// The shiny female depiction of this Pokémon from the front in battle.
    pub front_shiny_female: Option<String>,
    /// The default depiction of this Pokémon from the back in battle.
    pub back_default: Option<String>,
    /// The shiny depiction of this Pokémon from the back in battle.
    pub back_shiny: Option<String>,
    /// The female depiction of this Pokémon from the back in battle.
    pub back_female: Option<String>,
    /// The shiny female depiction of this Pokémon from the back in battle.
    pub back_shiny_female: Option<String>,
}

/// [PokemonSpecies official documentation](https://pokeapi.co/docs/v2#pokemonspecies)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonSpecies {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The order in which species should be sorted. Based on National Dex order,
    /// except families are grouped together and sorted by stage.
    pub order: Option<i64>,
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub gender_rate: Option<i64>,
    /// The base capture rate; up to 255. The higher the number, the easier the catch.
    pub capture_rate: Option<i64>,
    /// The happiness when caught by a normal Pokéball; up to 255. The higher the number, the happier the Pokémon.
    pub base_hapiness: Option<i64>,
    /// Whether or not this is a baby Pokémon.
    pub is_baby: Option<bool>,
    /// Whether or not this is a legendary Pokémon.
    pub is_legendary: Option<bool>,
    /// Whether or not this is a mythical Pokémon.
    pub is_mythical: Option<bool>,
    /// Initial hatch counter: one must walk 255 × (hatch_counter + 1) steps before this Pokémon's egg hatches,
    /// unless utilizing bonuses like Flame Body's.
    pub hatch_counter: Option<i64>,
    /// Whether or not this Pokémon has visual gender differences.
    pub has_gender_differences: Option<bool>,
    /// Whether or not this Pokémon has multiple forms and can switch between them.
    pub forms_switchable: Option<bool>,
    /// The rate at which this Pokémon species gains levels.
    pub growth_rate: Option<NamedApiResource>,
    /// A list of egg groups this Pokémon species is a member of.
    pub egg_groups: Option<Vec<NamedApiResource>>,
    /// The color of this Pokémon for Pokédex search.
    pub color: Option<NamedApiResource>,
    /// The shape of this Pokémon for Pokédex search.
    pub shape: Option<NamedApiResource>,
    /// The Pokémon species that evolves into this Pokemon_species.
    pub evolves_from_species: Option<NamedApiResource>,
    /// The evolution chain this Pokémon species is a member of.
    pub evolution_chain: Option<ApiResource>,
    /// The habitat this Pokémon species can be encountered in.
    pub habitat: Option<NamedApiResource>,
    /// The generation this Pokémon species was introduced in.
    pub generation: Option<NamedApiResource>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// Descriptions of different forms Pokémon take on within the Pokémon species.
    pub form_descriptions: Option<Vec<Description>>,
}

/// [Stat official documentation](https://pokeapi.co/docs/v2#stat)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Stat {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// ID the games use for this stat.
    pub game_index: Option<i64>,
    /// Whether this stat only exists within a battle.
    pub is_battle_only: Option<bool>,
    /// A detail of moves which affect this stat positively or negatively.
    pub affecting_moves: Option<MoveStatAffectSets>,
    /// A detail of natures which affect this stat positively or negatively.
    pub affecting_natures: Option<NatureStatAffectSets>,
    /// A list of characteristics that are set on a Pokémon when its highest base stat is this stat.
    pub characteristics: Option<Vec<ApiResource>>,
    /// The class of damage this stat is directly related to.
    pub move_damage_class: Option<NamedApiResource>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [MoveStatAffectSets official documentation](https://pokeapi.co/docs/v2#movestataffectsets)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveStatAffectSets {
    /// A list of moves and how they change the referenced stat.
    pub increase: Option<Vec<MoveStatAffect>>,
    /// A list of moves and how they change the referenced stat.
    pub decrease: Option<Vec<MoveStatAffect>>,
}

/// [MoveStatAffect official documentation](https://pokeapi.co/docs/v2#movestataffect)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveStatAffect {
    /// The maximum amount of change to the referenced stat.
    pub change: Option<i64>,
    /// The move causing the change.
    #[serde(rename = "move")]
    pub move_: Option<NamedApiResource>,
}

/// [NatureStatAffectSets official documentation](https://pokeapi.co/docs/v2#naturestataffectsets)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct NatureStatAffectSets {
    /// A list of natures and how they change the referenced stat.
    pub increase: Option<Vec<NamedApiResource>>,
    /// A list of nature sand how they change the referenced stat.
    pub decrease: Option<Vec<NamedApiResource>>,
}

/// [Type official documentation](https://pokeapi.co/docs/v2#type)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Type {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A detail of how effective this type is toward others and vice versa.
    pub damage_relations: Option<TypeRelations>,
    /// The generation this type was introduced in.
    pub generation: Option<NamedApiResource>,
    /// The class of damage inflicted by this type.
    pub move_damage_class: Option<NamedApiResource>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of details of Pokémon that have this type.
    pub pokemon: Option<Vec<TypePokemon>>,
    /// A list of moves that have this type.
    pub moves: Option<Vec<NamedApiResource>>,
}

/// [TypePokemon official documentation](https://pokeapi.co/docs/v2#typepokemon)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TypePokemon {
    /// The order the Pokémon's types are listed in.
    pub slot: Option<i64>,
    /// The Pokémon that has the referenced type.
    pub pokemon: Option<NamedApiResource>,
}

/// [TypeRelations official documentation](https://pokeapi.co/docs/v2#typerelations)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TypeRelations {
    /// A list of types this type has no effect on.
    pub no_damage_to: Option<Vec<NamedApiResource>>,
    /// A list of types this type is not very effect against.
    pub half_damage_to: Option<Vec<NamedApiResource>>,
    /// A list of types this type is very effect against.
    pub double_damage_to: Option<Vec<NamedApiResource>>,
    /// A list of types that have no effect on this type.
    pub no_damage_from: Option<Vec<NamedApiResource>>,
    /// A list of types that are not very effective against this type.
    pub half_damage_from: Option<Vec<NamedApiResource>>,
    /// A list of types that are very effective against this type.
    pub double_damage_from: Option<Vec<NamedApiResource>>,
}
