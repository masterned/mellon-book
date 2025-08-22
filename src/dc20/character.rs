use turann::Builder;
use uuid::Uuid;

use super::{Attributes, Background, ClassEntry, Origin};

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Character {
    #[builder(default = Uuid::new_v4)]
    id: Uuid,
    player_name: String,
    character_name: String,
    class: ClassEntry,
    ancestry: Origin,
    background: Background,
    #[builder(default)]
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
    use std::error::Error;

    use crate::dc20::*;

    use super::*;

    #[test]
    #[ignore = "API changed"]
    fn _all_fields_present_to_build_character() -> Result<(), Box<dyn Error>> {
        let result = Character::builder().build();
        assert_eq!(
            result,
            Err(CharacterBuilderError::missing_fields(&[
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

        let result = Character::builder().player_name("John Doe").build();
        assert_eq!(
            result,
            Err(CharacterBuilderError::missing_fields(&[
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

        let result = Character::builder()
            .player_name("John Doe")
            .ancestry(human.clone())
            .build();
        assert_eq!(
            result,
            Err(CharacterBuilderError::missing_fields(&[
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

        let attributes = Attributes::builder()
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

        let mut character = Character::builder()
            .player_name("John Doe")
            .character_name("Johannas Doeworth")
            .class(champion.clone())
            .ancestry(human.clone())
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
