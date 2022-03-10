//! Moves group models

use super::{
    resource::{
        ApiResource, Description, MachineVersionDetail, Name, NamedApiResource, VerboseEffect,
    },
};

/// [Move official documentation](https://pokeapi.co/docs/v2#move)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Move {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The percent value of how likely this move is to be successful.
    pub accuracy: Option<i64>,
    /// The percent value of how likely it is this moves effect will happen.
    pub effect_chance: Option<i64>,
    /// Power points. The number of times this move can be used.
    pub pp: Option<i64>,
    /// A value between -8 and 8. Sets the order in which moves are executed during battle.
    /// See [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Priority) for greater detail.
    pub priority: Option<i64>,
    /// The base power of this move with a value of 0 if it does not have a base power.
    pub power: Option<i64>,
    /// The type of appeal this move gives a Pokémon when used in a contest.
    pub contest_type: Option<NamedApiResource>,
    /// The effect the move has when used in a contest.
    pub contest_effect: Option<ApiResource>,
    /// The type of damage the move inflicts on the target, e.g. physical.
    pub damage_class: Option<NamedApiResource>,
    /// The effect of this move listed in different languages.
    pub effect_entries: Option<Vec<VerboseEffect>>,
    /// List of Pokemon that can learn the move
    pub learned_by_pokemon: Option<Vec<NamedApiResource>>,
    /// The flavor text of this move listed in different languages.
    pub flavor_text_entries: Option<Vec<MoveFlavorText>>,
    /// The generation in which this move was introduced.
    pub generation: Option<NamedApiResource>,
    /// A list of the machines that teach this move.
    pub machines: Option<Vec<MachineVersionDetail>>,
    /// Metadata about this move.
    pub meta: Option<MoveMetaData>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of stats this moves effects and how much it effects them.
    pub stat_changes: Option<Vec<MoveStatChange>>,
    /// The effect the move has when used in a super contest.
    pub super_contest_effect: Option<ApiResource>,
    /// The type of target that will receive the effects of the attack.
    pub target: Option<NamedApiResource>,
    /// The elemental type of this move.
    #[serde(rename = "type")]
    pub type_: Option<NamedApiResource>,
}

/// [MoveFlavorText official documentation](https://pokeapi.co/docs/v2#moveflavortext)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveFlavorText {
    /// The localized flavor text for an api resource in a specific language.
    pub flavor_text: Option<String>,
    /// The language this name is in.
    pub language: Option<NamedApiResource>,
    /// The version group that uses this flavor text.
    pub version_group: Option<NamedApiResource>,
}

/// [MoveMetaData official documentation](https://pokeapi.co/docs/v2#movemetadata)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveMetaData {
    /// The status ailment this move inflicts on its target.
    pub ailment: Option<NamedApiResource>,
    /// The category of move this move falls under, e.g. damage or ailment.
    pub category: Option<NamedApiResource>,
    /// The minimum number of times this move hits. Null if it always only hits once.
    pub min_hits: Option<i64>,
    /// The maximum number of times this move hits. Null if it always only hits once.
    pub max_hits: Option<i64>,
    /// The minimum number of turns this move continues to take effect. Null if it always only lasts one turn.
    pub min_turns: Option<i64>,
    /// The maximum number of turns this move continues to take effect. Null if it always only lasts one turn.
    pub max_turns: Option<i64>,
    /// HP drain (if positive) or Recoil damage (if negative), in percent of damage done.
    pub drain: Option<i64>,
    /// The amount of hp gained by the attacking Pokemon, in percent of it's maximum HP.
    pub healing: Option<i64>,
    /// Critical hit rate bonus.
    pub crit_rate: Option<i64>,
    /// The likelihood this attack will cause an ailment.
    pub ailment_chance: Option<i64>,
    /// The likelihood this attack will cause the target Pokémon to flinch.
    pub flinch_chance: Option<i64>,
    /// The likelihood this attack will cause a stat change in the target Pokémon.
    pub stat_chance: Option<i64>,
}

/// [MoveStatChange official documentation](https://pokeapi.co/docs/v2#movestatchange)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveStatChange {
    /// The amount of change.
    pub change: Option<i64>,
    /// The stat being affected.
    pub stat: Option<NamedApiResource>,
}

/// [MoveAilment official documentation](https://pokeapi.co/docs/v2#moveailment)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveAilment {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of moves that cause this ailment.
    pub moves: Option<Vec<NamedApiResource>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [MoveDamageClass official documentation](https://pokeapi.co/docs/v2#movedamageclass)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveDamageClass {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The description of this resource listed in different languages.
    pub descriptions: Option<Vec<Description>>,
    /// A list of moves that fall into this damage class.
    pub moves: Option<Vec<NamedApiResource>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [MoveLearnMethod official documentation](https://pokeapi.co/docs/v2#movelearnmethod)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveLearnMethod {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The description of this resource listed in different languages.
    pub descriptions: Option<Vec<Description>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of version groups where moves can be learned through this method.
    pub version_groups: Option<Vec<NamedApiResource>>,
}

/// [MoveTarget official documentation](https://pokeapi.co/docs/v2#movetarget)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveTarget {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The description of this resource listed in different languages.
    pub descriptions: Option<Vec<Description>>,
    /// A list of moves that that are directed at this target.
    pub moves: Option<Vec<Name>>,
    /// The name of this resource listed in different languages.ƒ
    pub names: Option<Vec<Name>>,
}