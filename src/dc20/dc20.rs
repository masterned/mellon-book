use core::fmt;
use std::error::Error;
use uuid::Uuid;

use crate::utils::SwapResult;

#[derive(Clone, Debug, Default)]
pub struct CharacterBuilder {
    pub player_name: Option<String>,
    pub character_name: Option<String>,
    pub class: Option<Class>,
    pub ancestry: Option<Ancestry>,
    pub background: Option<Background>,
    pub level: Level,
    pub stats: Option<Vec<Stat>>,
    pub trades: Vec<Skill>,
    pub languages: Option<Vec<Language>>,
    pub physical_defense: Option<Defense>,
    pub mystical_defense: Option<Defense>,
}

impl CharacterBuilder {
    #[must_use]
    pub fn new() -> Self {
        CharacterBuilder::default()
    }

    #[must_use]
    pub fn player_name(mut self, name: &str) -> Self {
        if name.is_empty() {
            return self;
        }

        let _ = self.player_name.insert(name.to_string());

        self
    }

    #[must_use]
    pub fn character_name(mut self, name: &str) -> Self {
        if name.is_empty() {
            return self;
        }

        let _ = self.character_name.insert(name.to_string());

        self
    }

    #[must_use]
    pub fn class(mut self, class: Class) -> Self {
        let _ = self.class.insert(class);

        self
    }

    #[must_use]
    pub fn ancestry(mut self, ancestry: Ancestry) -> Self {
        let _ = self.ancestry.insert(ancestry);

        self
    }

    #[must_use]
    pub fn background(mut self, background: Background) -> Self {
        let _ = self.background.insert(background);

        self
    }

    #[must_use]
    pub fn add_stat(mut self, stat: Stat) -> Self {
        self.stats.get_or_insert_default().push(stat);

        self
    }

    #[must_use]
    pub fn add_trade(mut self, trade: Skill) -> Self {
        self.trades.push(trade);

        self
    }

    #[must_use]
    pub fn add_language(mut self, language: Language) -> Self {
        self.languages.get_or_insert_default().push(language);

        self
    }

    #[must_use]
    pub fn physical_defense(mut self, physical_defense: Defense) -> Self {
        let _ = self.physical_defense.insert(physical_defense);

        self
    }

    #[must_use]
    pub fn mystical_defense(mut self, mystical_defense: Defense) -> Self {
        let _ = self.mystical_defense.insert(mystical_defense);

        self
    }

    pub fn build(self) -> Result<Character, CharacterBuildError> {
        self.try_into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    id: Uuid,
    player_name: String,
    character_name: String,
    class: Class,
    ancestry: Ancestry,
    background: Background,
    level: Level,
    stats: Vec<Stat>,
    trades: Vec<Skill>,
    languages: Vec<Language>,
    physical_defense: Defense,
    mystical_defense: Defense,
}

impl Character {
    #[must_use]
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    #[must_use]
    pub fn player_name(&self) -> &str {
        &self.player_name
    }

    #[must_use]
    pub fn character_name(&self) -> &str {
        &self.character_name
    }

    #[must_use]
    pub fn class(&self) -> &Class {
        &self.class
    }

    #[must_use]
    pub fn ancestry(&self) -> &Ancestry {
        &self.ancestry
    }

    #[must_use]
    pub fn background(&self) -> &Background {
        &self.background
    }

    #[must_use]
    pub fn level(&self) -> &Level {
        &self.level
    }

    #[must_use]
    pub fn stats(&self) -> &[Stat] {
        &self.stats
    }

    #[must_use]
    pub fn trades(&self) -> &[Skill] {
        &self.trades
    }

    #[must_use]
    pub fn languages(&self) -> &[Language] {
        &self.languages
    }

    #[must_use]
    pub fn physical_defense(&self) -> &Defense {
        &self.physical_defense
    }

    #[must_use]
    pub fn mystical_defense(&self) -> &Defense {
        &self.mystical_defense
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CharacterBuildError {
    FieldMissing(Vec<&'static str>),
}

impl Error for CharacterBuildError {}

impl fmt::Display for CharacterBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cannot build Character: {}",
            match self {
                CharacterBuildError::FieldMissing(field) => format!(
                    "missing field(s): {}",
                    field
                        .iter()
                        .map(|s| format!("`{s}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            }
        )
    }
}

#[derive(Clone, Debug, Default)]
struct FieldAggregator(Option<Vec<&'static str>>);

impl FieldAggregator {
    fn new() -> Self {
        FieldAggregator::default()
    }

    fn field_check<T>(&mut self, field: &Option<T>, field_name: &'static str) {
        if field.is_none() {
            self.0.get_or_insert_default().push(field_name);
        }
    }
}

impl TryFrom<FieldAggregator> for CharacterBuildError {
    type Error = ();

    fn try_from(value: FieldAggregator) -> Result<Self, Self::Error> {
        value.0.map(CharacterBuildError::FieldMissing).ok_or(())
    }
}

impl TryFrom<CharacterBuilder> for Character {
    type Error = CharacterBuildError;

    fn try_from(value: CharacterBuilder) -> Result<Self, Self::Error> {
        let CharacterBuilder { trades, .. } = value;

        let mut aggregator = FieldAggregator::new();

        aggregator.field_check(&value.player_name, "Player Name");
        aggregator.field_check(&value.character_name, "Character Name");
        aggregator.field_check(&value.class, "Class");
        aggregator.field_check(&value.ancestry, "Ancestry");
        aggregator.field_check(&value.background, "Background");
        aggregator.field_check(&value.stats, "Stats");
        aggregator.field_check(&value.languages, "Languages");
        aggregator.field_check(&value.physical_defense, "Physical Defense");
        aggregator.field_check(&value.mystical_defense, "Mystical Defense");

        CharacterBuildError::try_from(aggregator).swap()?;

        Ok(Character {
            id: Uuid::new_v4(),
            player_name: value.player_name.unwrap(),
            character_name: value.character_name.unwrap(),
            class: value.class.unwrap(),
            ancestry: value.ancestry.unwrap(),
            background: value.background.unwrap(),
            level: value.level,
            stats: value.stats.unwrap(),
            trades,
            languages: value.languages.unwrap(),
            physical_defense: value.physical_defense.unwrap(),
            mystical_defense: value.mystical_defense.unwrap(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub uuid: Uuid,
    pub name: String,
    subclass: Option<Subclass>,
}

impl Class {
    pub fn new(name: impl Into<String>) -> Self {
        Class {
            uuid: Uuid::new_v4(),
            name: name.into(),
            subclass: None,
        }
    }

    #[must_use]
    pub fn add_subclass(mut self, subclass: Subclass) -> Self {
        let _ = self.subclass.insert(subclass);

        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Subclass {
    pub uuid: Uuid,
    pub name: String,
}

impl Subclass {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ancestry {
    pub uuid: Uuid,
    pub name: String,
}

impl Ancestry {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Background {
    pub uuid: Uuid,
    pub name: String,
}

impl Background {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Level(usize);

impl Level {
    #[must_use]
    pub fn calc_combat_mastery(&self) -> usize {
        self.0.div_ceil(2)
    }
}

impl Default for Level {
    fn default() -> Self {
        Level(1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mastery {
    Novice = 2,
    Adept = 4,
    Expert = 6,
    Master = 8,
    GrandMaster = 10,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stat {
    pub name: String,
    pub score: isize,
    pub save_proficiency: bool,
    pub skills: Vec<Skill>,
}

impl Stat {
    #[must_use]
    pub fn calc_save(&self, level: Level) -> isize {
        self.score
            .checked_add_unsigned(level.calc_combat_mastery())
            .unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Skill {
    pub uuid: Uuid,
    pub name: String,
    pub mastery: Option<Mastery>,
}

impl Skill {
    #[must_use]
    pub fn calc_score(&self, stat: &Stat) -> isize {
        if let Some(mastery) = self.mastery {
            mastery as isize + stat.score
        } else {
            stat.score
        }
    }
}

impl Skill {
    pub fn new(name: impl Into<String>) -> Self {
        Skill {
            uuid: Uuid::new_v4(),
            name: name.into(),
            mastery: None,
        }
    }

    pub fn set_mastery(&mut self, mastery: Mastery) {
        let _ = self.mastery.insert(mastery);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fluency {
    Limited,
    Fluent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Language {
    pub uuid: Uuid,
    pub name: String,
    pub fluency: Fluency,
}

impl Language {
    pub fn new(name: impl Into<String>) -> Self {
        Language {
            uuid: Uuid::new_v4(),
            name: name.into(),
            fluency: Fluency::Limited,
        }
    }

    pub fn set_fluency(&mut self, fluency: Fluency) {
        self.fluency = fluency;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Defense {
    pub name: String,
    pub score: usize,
    pub reduction: usize,
}

impl Defense {
    #[must_use]
    pub fn heavy(&self) -> usize {
        self.score + 5
    }

    #[must_use]
    pub fn brutal(&self) -> usize {
        self.score + 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _all_fields_present_to_build_character() -> Result<(), Box<dyn Error>> {
        let result = CharacterBuilder::new().build();
        assert_eq!(
            result,
            Err(CharacterBuildError::FieldMissing(vec![
                "Player Name",
                "Character Name",
                "Class",
                "Ancestry",
                "Background",
                "Stats",
                "Languages",
                "Physical Defense",
                "Mystical Defense"
            ]))
        );

        let result = CharacterBuilder::new().player_name("John Doe").build();
        assert_eq!(
            result,
            Err(CharacterBuildError::FieldMissing(vec![
                "Character Name",
                "Class",
                "Ancestry",
                "Background",
                "Stats",
                "Languages",
                "Physical Defense",
                "Mystical Defense"
            ]))
        );

        let result = CharacterBuilder::new()
            .player_name("John Doe")
            .ancestry(Ancestry::new("Human"))
            .build();
        assert_eq!(
            result,
            Err(CharacterBuildError::FieldMissing(vec![
                "Character Name",
                "Class",
                "Background",
                "Stats",
                "Languages",
                "Physical Defense",
                "Mystical Defense"
            ]))
        );

        let champion = Class::new("Champion");
        let human = Ancestry::new("Human");
        let soldier = Background::new("Soldier");

        let mut perception = Skill::new("Perception");
        perception.set_mastery(Mastery::Novice);

        let mut common = Language::new("Common");
        common.set_fluency(Fluency::Fluent);

        let mut character = CharacterBuilder::new()
            .player_name("John Doe")
            .character_name("Johannas Doeworth")
            .class(champion.clone())
            .ancestry(human.clone())
            .background(soldier.clone())
            .add_stat(Stat {
                name: "Prime".to_string(),
                score: 3,
                save_proficiency: false,
                skills: vec![perception.clone()],
            })
            .add_language(common.clone())
            .physical_defense(Defense {
                name: "Physical Defense".into(),
                score: 10,
                reduction: 0,
            })
            .mystical_defense(Defense {
                name: "Mystical Defense".into(),
                score: 10,
                reduction: 0,
            })
            .build()?;

        let char_id = Uuid::new_v4();
        character.id = char_id;

        assert_eq!(
            character,
            Character {
                id: char_id,
                player_name: "John Doe".to_string(),
                character_name: "Johannas Doeworth".to_string(),
                class: champion,
                ancestry: human,
                background: soldier,
                level: Level::default(),
                stats: vec![Stat {
                    name: "Prime".to_string(),
                    score: 3,
                    save_proficiency: false,
                    skills: vec![perception]
                }],
                trades: vec![],
                languages: vec![common],
                physical_defense: Defense {
                    name: "Physical Defense".to_string(),
                    score: 10,
                    reduction: 0
                },
                mystical_defense: Defense {
                    name: "Mystical Defense".to_string(),
                    score: 10,
                    reduction: 0
                }
            }
        );

        Ok(())
    }

    #[test]
    fn _combat_mastery_half_level_rounded_up() {
        assert_eq!(Level(1).calc_combat_mastery(), 1);
        assert_eq!(Level(2).calc_combat_mastery(), 1);
        assert_eq!(Level(3).calc_combat_mastery(), 2);
        assert_eq!(Level(4).calc_combat_mastery(), 2);
        assert_eq!(Level(5).calc_combat_mastery(), 3);
        assert_eq!(Level(6).calc_combat_mastery(), 3);
        assert_eq!(Level(7).calc_combat_mastery(), 4);
        assert_eq!(Level(8).calc_combat_mastery(), 4);
        assert_eq!(Level(9).calc_combat_mastery(), 5);
        assert_eq!(Level(10).calc_combat_mastery(), 5);
        assert_eq!(Level(11).calc_combat_mastery(), 6);
        assert_eq!(Level(12).calc_combat_mastery(), 6);
        assert_eq!(Level(13).calc_combat_mastery(), 7);
        assert_eq!(Level(14).calc_combat_mastery(), 7);
        assert_eq!(Level(15).calc_combat_mastery(), 8);
        assert_eq!(Level(16).calc_combat_mastery(), 8);
        assert_eq!(Level(17).calc_combat_mastery(), 9);
        assert_eq!(Level(18).calc_combat_mastery(), 9);
        assert_eq!(Level(19).calc_combat_mastery(), 10);
        assert_eq!(Level(20).calc_combat_mastery(), 10);
    }
}
