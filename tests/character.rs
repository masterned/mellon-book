use std::error::Error;

use mellon_book::{dc20::*, player::Player};

#[test]
fn _built_character_should_have_name() -> Result<(), Box<dyn Error>> {
    let player = Player::builder().name("Test Player")?.build()?;

    let human = Ancestry::builder().name("Human").build()?;

    let test_character = Character::builder()
        .player(player)
        .character_name("Test Name")
        .class(ClassEntry::new("Champion"))
        .ancestry(human)
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
        .build()?;

    assert_eq!(test_character.character_name(), "Test Name");

    Ok(())
}
