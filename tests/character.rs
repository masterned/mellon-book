use std::error::Error;

use mellon_book::dc20::{background, AttributesBuilder, LanguageFluency};
use mellon_book::dc20::{Ancestry, Attribute, CharacterBuilder, Class, Defense, Skill};

#[test]
fn _built_character_should_have_name() -> Result<(), Box<dyn Error>> {
    let test_character = CharacterBuilder::new()
        .player_name("Test Player")
        .character_name("Test Name")
        .class(Class::new("Warrior"))
        .ancestry(Ancestry::new("Human"))
        .background(
            background::Builder::new("Soldier")
                .add_skill(Skill::new("Athletics"))
                .add_trade(Skill::new("Blacksmithing"))
                .add_language_fluency(LanguageFluency::common())
                .build()?,
        )
        .attributes(
            AttributesBuilder::new()
                .prime(Attribute::new().with_base_score(3).with_save_proficiency())
                .might(Attribute::default())
                .agility(Attribute::default())
                .charisma(Attribute::default())
                .intelligence(Attribute::default())
                .build()?,
        )
        .physical_defense(Defense {
            name: "Physical".into(),
            score: 10,
            reduction: 0,
        })
        .mystical_defense(Defense {
            name: "Mystical".into(),
            score: 10,
            reduction: 0,
        })
        .build()?;

    assert_eq!(test_character.character_name(), "Test Name");

    Ok(())
}
