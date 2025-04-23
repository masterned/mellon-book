use core::fmt;
use std::error::Error;
use uuid::Uuid;

use crate::utils::{FieldAggregator, SwapResult};

use super::{Attributes, Background, ClassEntry, Origin};

#[derive(Clone, Debug, Default)]
pub struct CharacterBuilder {
    pub player_name: Option<String>,
    pub character_name: Option<String>,
    pub class: Option<ClassEntry>,
    pub ancestry: Option<Origin>,
    pub background: Option<Background>,
    pub level: Level,
    pub attributes: Option<Attributes>,
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
    pub fn class(mut self, class: ClassEntry) -> Self {
        let _ = self.class.insert(class);

        self
    }

    #[must_use]
    pub fn origin(mut self, origin: Origin) -> Self {
        let _ = self.ancestry.insert(origin);

        self
    }

    #[must_use]
    pub fn background(mut self, background: Background) -> Self {
        let _ = self.background.insert(background);

        self
    }

    #[must_use]
    pub fn attributes(mut self, attributes: Attributes) -> Self {
        let _ = self.attributes.insert(attributes);

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
    class: ClassEntry,
    ancestry: Origin,
    background: Background,
    level: Level,
    attributes: Attributes,
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
    pub fn class(&self) -> &ClassEntry {
        &self.class
    }

    #[must_use]
    pub fn ancestry(&self) -> &Origin {
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
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
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

impl TryFrom<FieldAggregator> for CharacterBuildError {
    type Error = ();

    fn try_from(value: FieldAggregator) -> Result<Self, Self::Error> {
        value.0.map(CharacterBuildError::FieldMissing).ok_or(())
    }
}

impl TryFrom<CharacterBuilder> for Character {
    type Error = CharacterBuildError;

    fn try_from(value: CharacterBuilder) -> Result<Self, Self::Error> {
        let mut aggregator = FieldAggregator::new();

        aggregator.field_check(&value.player_name, "Player Name");
        aggregator.field_check(&value.character_name, "Character Name");
        aggregator.field_check(&value.class, "Class");
        aggregator.field_check(&value.ancestry, "Ancestry");
        aggregator.field_check(&value.background, "Background");
        aggregator.field_check(&value.attributes, "Attributes");
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
            attributes: value.attributes.unwrap(),
            physical_defense: value.physical_defense.unwrap(),
            mystical_defense: value.mystical_defense.unwrap(),
        })
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
    use crate::dc20::{
        AncestryInstance, Attribute, AttributesBuilder, LanguageFluency, Mastery, Skill,
    };

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
                "Attributes",
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
                "Attributes",
                "Physical Defense",
                "Mystical Defense"
            ]))
        );

        let human = Origin::PureBred(AncestryInstance {
            name: "Human".into(),
            ..Default::default()
        });

        let result = CharacterBuilder::new()
            .player_name("John Doe")
            .origin(human.clone())
            .build();
        assert_eq!(
            result,
            Err(CharacterBuildError::FieldMissing(vec![
                "Character Name",
                "Class",
                "Background",
                "Attributes",
                "Physical Defense",
                "Mystical Defense"
            ]))
        );

        let champion = ClassEntry::new("Champion");

        let soldier = Background::builder()
            .name("Soldier")?
            .skill(Skill::new("Athletics"))
            .trade(Skill::new("Blacksmithing"))
            .language_fluency(LanguageFluency::common())
            .build()?;

        let attributes = AttributesBuilder::new()
            .prime(
                Attribute::new()
                    .with_base_score(3)
                    .with_skill(Skill::new("Perception").with_mastery(Mastery::Novice)),
            )
            .might(Attribute::default())
            .agility(Attribute::default())
            .charisma(Attribute::default())
            .intelligence(Attribute::default())
            .build()?;

        let mut character = CharacterBuilder::new()
            .player_name("John Doe")
            .character_name("Johannas Doeworth")
            .class(champion.clone())
            .origin(human.clone())
            .background(soldier.clone())
            .attributes(attributes.clone())
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
                attributes,
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
