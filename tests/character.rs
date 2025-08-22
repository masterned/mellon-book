use std::error::Error;

use mellon_book::dc20::*;
use uuid::Uuid;

#[test]
fn _built_character_should_have_name() -> Result<(), Box<dyn Error>> {
    let unkillable = AncestryTrait {
        uuid: Uuid::new_v4(),
        name: "Unkillable".into(),
        cost: 0,
        description: "Gives advantage on death saves".into(),
    };
    let human = AncestryInstanceBuilder::from(AncestryEntry {
        uuid: Uuid::new_v4(),
        name: "Human".into(),
        description: "Versatile but milktoast".into(),
        available_traits: vec![unkillable.clone()],
    })
    .add_ancestry_trait(unkillable)?
    .build()?;

    let test_character = Character::builder()
        .player_name("Test Player")
        .character_name("Test Name")
        .class(ClassEntry::new("Champion"))
        .ancestry(Origin::PureBred(human))
        .background(
            Background::builder()
                .name("Soldier")?
                .skill(Skill::new("Athletics"))
                .trade(Skill::new("Blacksmithing"))
                .language_fluency(LanguageFluency::common())
                .build()?,
        )
        .attributes(
            Attributes::builder()
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
