//! Pokemon group models

use super::resource::{
    ApiResource, Description, Effect, FlavorText, GenerationGameIndex, Name, NamedApiResource,
    VerboseEffect, VersionEncounterDetail, VersionGameIndex,
};

/// [Ability official documentation](https://pokeapi.co/docs/v2#ability)
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
    /// The list of previous effects this ability has had across version groups.
    pub effect_changes: Option<Vec<AbilityEffectChange>>,
    /// The flavor text of this ability listed in different languages.
    pub flavor_text_entries: Option<Vec<AbilityFlavorText>>,
    /// A list of Pokémon that could potentially have this ability.
    pub pokemon: Option<Vec<AbilityPokemon>>,
}

/// [AbilityEffectChange official documentation](https://pokeapi.co/docs/v2#abilityeffectchange)
pub struct AbilityEffectChange {
    /// The previous effect of this ability listed in different languages.
    pub effect_entries: Option<Vec<Effect>>,
    /// The version group in which the previous effect of this ability originated.
    pub version_group: Option<NamedApiResource>,
}

/// [AbilityFlavorText official documentation](https://pokeapi.co/docs/v2#abilityflavortext)
pub struct AbilityFlavorText {
    /// The localized name for an API resource in a specific language.
    pub flavor_text: Option<String>,
    /// The language this text resource is in.
    pub language: Option<NamedApiResource>,
    /// The version group that uses this flavor text.
    pub version_group: Option<NamedApiResource>,
}

/// [AbilityPokemon official documentation](https://pokeapi.co/docs/v2#abilitypokemon)
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

/// [EggGroup official documentation](https://pokeapi.co/docs/v2#egggroup)
pub struct EggGroup {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of all Pokémon species that are members of this egg group.
    pub pokemon_species: Option<Vec<NamedApiResource>>,
}

/// [Gender official documentation](https://pokeapi.co/docs/v2#gender)
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
pub struct PokemonSpeciesGender {
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub rate: Option<i64>,
    /// A Pokémon species that can be the referenced gender.
    pub pokemon_species: Option<NamedApiResource>,
}

/// [GrowthRate official documentation](https://pokeapi.co/docs/v2#growthrate)
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
pub struct GrowthRateExperienceLevel {
    /// The level gained.
    pub level: Option<i64>,
    /// The amount of experience required to reach the referenced level.
    pub experience: Option<i64>,
}

/// [Nature official documentation](https://pokeapi.co/docs/v2#nature)
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
    /// A list of Pokéathlon stats this nature effects and how much it effects them.
    pub pokeathlon_stat_changes: Option<Vec<NatureStatChange>>,
    /// A list of battle styles and how likely a Pokémon with this nature is to use them in the Battle Palace or Battle Tent.
    pub move_battle_style_preferences: Option<Vec<MoveBattleStylePreference>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [NatureStatChange official documentation](https://pokeapi.co/docs/v2#naturestatchange)
pub struct NatureStatChange {
    /// The amount of change.
    pub max_change: Option<i64>,
    /// The stat being affected.
    pub pokeathlon_stat: Option<NamedApiResource>,
}

/// [MoveBattleStylePreference official documentation](https://pokeapi.co/docs/v2#movebattlestylepreference)
pub struct MoveBattleStylePreference {
    /// Chance of using the move, in percent, if HP is under one half.
    pub low_hp_preference: Option<i64>,
    /// Chance of using the move, in percent, if HP is over one half.
    pub high_hp_preference: Option<i64>,
    /// The move battle style.
    pub move_battle_style: Option<NamedApiResource>,
}

/// [PokeathlonStat official documentation](https://pokeapi.co/docs/v2#pokeathlonstat)
pub struct PokeathlonStat {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A detail of natures which affect this Pokéathlon stat positively or negatively.
    pub affecting_natures: Option<NaturePokeathlonStatAffectSets>,
}

/// [NaturePokeathlonStatAffectSets official documentation](https://pokeapi.co/docs/v2#naturepokeathlonstataffectsets)
pub struct NaturePokeathlonStatAffectSets {
    /// A list of natures and how they change the referenced Pokéathlon stat.
    pub increase: Option<Vec<NaturePokeathlonStatAffect>>,
    /// A list of natures and how they change the referenced Pokéathlon stat.
    pub decrease: Option<Vec<NaturePokeathlonStatAffect>>,
}


/// [Pokemon official documentation](https://pokeapi.co/docs/v2#pokemon)
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
pub struct PokemonAbility {
    /// Whether or not this is a hidden ability.
    pub is_hidden: Option<bool>,
    /// The slot this ability occupies in this Pokémon species.
    pub slot: Option<i64>,
    /// The ability the Pokémon may have.
    pub ability: Option<NamedApiResource>,
}

/// [PokemonType official documentation](https://pokeapi.co/docs/v2#pokemontype)
pub struct PokemonType {
    /// The order the Pokémon's types are listed in.
    pub slot: Option<i64>,
    /// The type the referenced Pokémon has.
    #[serde(rename = "type")]
    pub type_: Option<NamedApiResource>,
}

/// [PokemonTypePast official documentation](https://pokeapi.co/docs/v2#pokemontypepast)
pub struct PokemonTypePast {
    /// The last generation in which the referenced pokémon had the listed types.
    pub generation: Option<NamedApiResource>,
    /// The types the referenced pokémon had up to and including the listed generation.
    pub types: Option<Vec<PokemonType>>,
}

/// [PokemonHeldItem official documentation](https://pokeapi.co/docs/v2#pokemonhelditem)
pub struct PokemonHeldItem {
    /// The item the referenced Pokémon holds.
    pub item: Option<NamedApiResource>,
    /// The details of the different versions in which the item is held.
    pub version_details: Option<Vec<PokemonHeldItemVersion>>,
}

/// [PokemonHeldItemVersion official documentation](https://pokeapi.co/docs/v2#pokemonhelditemversion)
pub struct PokemonHeldItemVersion {
    /// The version in which the item is held.
    pub version: Option<NamedApiResource>,
    /// How often the item is held.
    pub rarity: Option<i64>,
}
/// [PokemonMove official documentation](https://pokeapi.co/docs/v2#pokemonmove)
pub struct PokemonMove {
    /// The move the Pokémon can learn.
    #[serde(rename = "move")]
    pub move_: Option<NamedApiResource>,
    /// The details of the version in which the Pokémon can learn the move.
    pub version_group_details: Option<Vec<PokemonMoveVersion>>,
}

/// [PokemonMoveVersion official documentation](https://pokeapi.co/docs/v2#pokemonmoveversion)
pub struct PokemonMoveVersion {
    /// The method by which the move is learned.
    pub move_learn_method: Option<NamedApiResource>,
    /// The version group in which the move is learned.
    pub version_group: Option<NamedApiResource>,
    /// The minimum level to learn the move.
    pub level_learned_at: Option<i64>,
}

/// [PokemonStat official documentation](https://pokeapi.co/docs/v2#pokemonstat)
pub struct PokemonStat {
    /// The stat the Pokémon has.
    pub stat: Option<NamedApiResource>,
    /// The effort points (EV) the Pokémon has in the stat.
    pub effort: Option<i64>,
    /// The base value of the stat.
    pub base_stat: Option<i64>,
}

/// [PokemonSprites official documentation](https://pokeapi.co/docs/v2#pokemonsprites)
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
    /// Other sprites.
    pub other: Option<OtherSprites>,
    /// Sprites per version.
    pub versions: Option<VersionsSprites>,
}


/// References the official artwork of a Pokémon.
pub struct OfficialArtworkSprites {
    /// The default despiction of this Pokémon form the official artwork.
    pub front_default: Option<String>,
}

/// Sprites of a Pokémon, per generation.
pub struct VersionsSprites {
    /// Sprites for the first generation.
    #[serde(rename = "generation-i")]
    pub generation_i: Option<GenerationISprites>,
    /// Sprites for the second generation.
    #[serde(rename = "generation-i")]
    pub generation_ii: Option<GenerationIISprites>,
    /// Sprites for the third generation.
    #[serde(rename = "generation-iii")]
    pub generation_iii: Option<GenerationIIISprites>,
    /// Sprites for the fourth generation.
    #[serde(rename = "generation-iv")]
    pub generation_iv: Option<GenerationIVSprites>,
    /// Sprites for the fifth generation.
    #[serde(rename = "generation-v")]
    pub generation_v: Option<GenerationVSprites>,
    /// Sprites for the sixth generation.
    #[serde(rename = "generation-vi")]
    pub generation_vi: Option<GenerationVISprites>,
    /// Sprites for the seventh generation.
    #[serde(rename = "generation-vii")]
    pub generation_vii: Option<GenerationVIISprites>,
    /// Sprites for the eighth generation.
    #[serde(rename = "generation-viii")]
    pub generation_viii: Option<GenerationVIIISprites>,
}

/// Sprites for the first generation.
pub struct GenerationISprites {
    /// Sprites for Pokémon Red & Pokémon Blue.
    #[serde(rename = "red-blue")]
    pub red_blue: Option<RedBlueSprites>,
    /// Sprites for Pokémon Yellow.
    pub yellow: Option<YellowSprites>,
}

/// Sprites for Pokémon Red & Pokémon Blue.
pub struct RedBlueSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default back sprite of a Pokémon, in gray.
    pub back_gray: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default front sprite of a Pokémon, in gray.
    pub front_gray: Option<String>,
}

/// Sprites for Pokémon Yellow.
pub struct YellowSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default back sprite of a Pokémon, in gray.
    pub back_gray: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default front sprite of a Pokémon, in gray.
    pub front_gray: Option<String>,
}

/// Sprites for the second generation.
pub struct GenerationIISprites {
    /// Sprites for Pokémon Crystal.
    pub crystal: Option<CrystalSprites>,
    /// Sprites for Pokémon Gold.
    pub gold: Option<GoldSprites>,
    /// Sprites for Pokémon Silver.
    pub silver: Option<SilverSprites>,
}

/// Sprites for Pokémon Crystal.
pub struct CrystalSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for Pokémon Gold.
pub struct GoldSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for Pokémon Silver.
pub struct SilverSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for the third generation.
pub struct GenerationIIISprites {
    /// Sprites for Pokémon Emerald.
    pub emerald: Option<EmeraldSprites>,
    /// Sprites for Pokémon FireRed & Pokémon LeafGreen.
    #[serde(rename = "firered-leafgreen")]
    pub firered_leafgreen: Option<FireredLeafgreenSprites>,
    /// Sprites for Pokémon Ruby & Pokémon Sapphire.
    #[serde(rename = "ruby-sapphire")]
    pub ruby_sapphire: Option<RubySapphireSprites>,
}

/// Sprites for Pokémon Emerald.
pub struct EmeraldSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for Pokémon FireRed & Pokémon LeafGreen.
pub struct FireredLeafgreenSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// The icons sprites of a Pokémon.
pub struct IconsSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
}

/// Sprites for Pokémon UltraSun & Pokémon UltraMoon.
pub struct UltrasunUltramoonSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// Sprites for the eighth generation.
pub struct GenerationVIIISprites {
    /// The icons sprites of a Pokémon.
    pub icons: Option<IconsSprites>,
}

/// [LocationAreaEncounter official documentation](https://pokeapi.co/docs/v2#locationareaencounter)
pub struct LocationAreaEncounter {
    /// The location area the referenced Pokémon can be encountered in.
    pub location_area: Option<NamedApiResource>,
    /// A list of versions and encounters with the referenced Pokémon that might happen.
    pub version_details: Option<Vec<VersionEncounterDetail>>,
}

/// [PokemonColor official documentation](https://pokeapi.co/docs/v2#pokemoncolor)
pub struct PokemonColor {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of the Pokémon species that have this color.
    pub pokemon_species: Option<Vec<NamedApiResource>>,
}

/// [PokemonForm official documentation](https://pokeapi.co/docs/v2#pokemonform)
pub struct PokemonForm {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The order in which forms should be sorted within all forms. Multiple forms may have equal order,
    /// in which case they should fall back on sorting by name.
    pub order: Option<i64>,
    /// The order in which forms should be sorted within a species' forms.
    pub form_order: Option<i64>,
    /// True for exactly one form used as the default for each Pokémon.
    pub is_default: Option<bool>,
    /// Whether or not this form can only happen during battle.
    pub is_battle_only: Option<bool>,
    /// Whether or not this form requires mega evolution.
    pub is_mega: Option<bool>,
    /// The name of this form.
    pub form_name: Option<String>,
    /// The Pokémon that can take on this form.
    pub pokemon: Option<NamedApiResource>,
    /// A list of details showing types this Pokémon form has.
    pub types: Option<Vec<PokemonFormType>>,
    /// A set of sprites used to depict this Pokémon form in the game.
    pub sprites: Option<PokemonFormSprites>,
    /// The version group this Pokémon form was introduced in.
    pub version_group: Option<NamedApiResource>,
    /// The form specific full name of this Pokémon form, or empty if the form does not have a specific name.
    pub names: Option<Vec<Name>>,
    /// The form specific form name of this Pokémon form, or empty if the form does not have a specific name.
    pub form_names: Option<Vec<Name>>,
}

/// [PokemonFormType official documentation](https://pokeapi.co/docs/v2#pokemonformtype)
pub struct PokemonFormType {
    /// The order the Pokémon's types are listed in.
    pub slot: Option<i64>,
    /// The type the referenced Form has.
    #[serde(rename = "type")]
    pub type_: Option<NamedApiResource>,
}

/// [PokemonFormSprites official documentation](https://pokeapi.co/docs/v2#pokemonformsprites)
pub struct PokemonFormSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default female back sprite of a Pokémon.
    pub back_female: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The shiny female back sprite of a Pokémon.
    pub back_shiny_female: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// [PokemonHabitat official documentation](https://pokeapi.co/docs/v2#pokemonhabitat)
pub struct PokemonHabitat {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of the Pokémon species that can be found in this habitat.
    pub pokemon_species: Option<Vec<NamedApiResource>>,
}

/// [PokemonShape official documentation](https://pokeapi.co/docs/v2#pokemonshape)
pub struct PokemonShape {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The "scientific" name of this Pokémon shape listed in different languages.
    pub awesome_names: Option<Vec<AwesomeName>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of the Pokémon species that have this shape.
    pub pokemon_species: Option<Vec<NamedApiResource>>,
}

/// [AwesomeName official documentation](https://pokeapi.co/docs/v2#awesomename)
pub struct AwesomeName {
    /// The localized "scientific" name for an API resource in a specific language.
    pub awesome_name: Option<String>,
    /// The language this "scientific" name is in.
    pub language: Option<NamedApiResource>,
}

/// [PokemonSpecies official documentation](https://pokeapi.co/docs/v2#pokemonspecies)
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
    /// A list of Pokedexes and the indexes reserved within them for this Pokémon species.
    pub pokemon_numbers: Option<Vec<PokemonSpeciesDexEntry>>,
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
    /// A list of encounters that can be had with this Pokémon species in pal park.
    pub pal_park_encounters: Option<Vec<PalParkEncounterArea>>,
    /// A list of flavor text entries for this Pokémon species.
    pub flavor_text_entries: Option<Vec<FlavorText>>,
    /// Descriptions of different forms Pokémon take on within the Pokémon species.
    pub form_descriptions: Option<Vec<Description>>,
    /// The genus of this Pokémon species listed in multiple languages.
    pub genera: Option<Vec<Genus>>,
    /// A list of the Pokémon that exist within this Pokémon species.
    pub varieties: Option<Vec<PokemonSpeciesVariety>>,
}

/// [Genus official documentation](https://pokeapi.co/docs/v2#genus)
pub struct Genus {
    /// The localized genus for the referenced Pokémon species.
    pub genus: Option<String>,
    /// The language this genus is in.
    pub language: Option<NamedApiResource>,
}

/// [PokemonSpeciesDexEntry official documentation](https://pokeapi.co/docs/v2#pokemonspeciesdexentry)
pub struct PokemonSpeciesDexEntry {
    /// The index number within the Pokédex.
    pub entry_number: Option<i64>,
    /// The Pokédex the referenced Pokémon species can be found in.
    pub pokedex: Option<NamedApiResource>,
}

/// [PalParkEncounterArea official documentation](https://pokeapi.co/docs/v2#palparkencounterarea)
pub struct PalParkEncounterArea {
    /// The base score given to the player when the referenced Pokémon is caught during a pal park run.
    pub base_score: Option<i64>,
    /// The base rate for encountering the referenced Pokémon in this pal park area.
    pub rate: Option<i64>,
    /// The pal park area where this encounter happens.
    pub area: Option<NamedApiResource>,
}

/// [PokemonSpeciesVariety official documentation](https://pokeapi.co/docs/v2#pokemonspeciesvariety)
pub struct PokemonSpeciesVariety {
    /// Whether this variety is the default variety.
    pub is_default: Option<bool>,
    /// The Pokémon variety.
    pub pokemon: Option<NamedApiResource>,
}

/// [Stat official documentation](https://pokeapi.co/docs/v2#stat)
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
pub struct MoveStatAffectSets {
    /// A list of moves and how they change the referenced stat.
    pub increase: Option<Vec<MoveStatAffect>>,
    /// A list of moves and how they change the referenced stat.
    pub decrease: Option<Vec<MoveStatAffect>>,
}

/// [MoveStatAffect official documentation](https://pokeapi.co/docs/v2#movestataffect)
pub struct MoveStatAffect {
    /// The maximum amount of change to the referenced stat.
    pub change: Option<i64>,
    /// The move causing the change.
    #[serde(rename = "move")]
    pub move_: Option<NamedApiResource>,
}

/// [NatureStatAffectSets official documentation](https://pokeapi.co/docs/v2#naturestataffectsets)
pub struct NatureStatAffectSets {
    /// A list of natures and how they change the referenced stat.
    pub increase: Option<Vec<NamedApiResource>>,
    /// A list of nature sand how they change the referenced stat.
    pub decrease: Option<Vec<NamedApiResource>>,
}

/// [Type official documentation](https://pokeapi.co/docs/v2#type)
pub struct Type {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A detail of how effective this type is toward others and vice versa.
    pub damage_relations: Option<TypeRelations>,
    /// A list of details of how effective this type was toward others and vice versa in previous generations.
    pub past_damage_relations: Option<Vec<TypeRelationsPast>>,
    /// A list of game indices relevent to this item by generation.
    pub game_indices: Option<Vec<GenerationGameIndex>>,
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
pub struct TypePokemon {
    /// The order the Pokémon's types are listed in.
    pub slot: Option<i64>,
    /// The Pokémon that has the referenced type.
    pub pokemon: Option<NamedApiResource>,
}

/// [TypeRelations official documentation](https://pokeapi.co/docs/v2#typerelations)
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

/// [TypeRelationsPast official documentation](https://pokeapi.co/docs/v2#typerelationspast)
pub struct TypeRelationsPast {
    /// The last generation in which the referenced type had the listed damage relations.
    pub generation: Option<NamedApiResource>,
    /// The damage relations the referenced type had up to and including the listed generation.
    pub damage_relations: Option<TypeRelations>,
}