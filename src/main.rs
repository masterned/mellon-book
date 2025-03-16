use std::error::Error;

use mellon_book::dc20::*;

fn main() -> Result<(), Box<dyn Error>> {
    let character = CharacterBuilder::default()
        .player_name("Spencer")
        .character_name("Cygnus")
        .class(Class::new("Sorcerer"))
        .ancestry(Ancestry::new("Human/Psyborn"))
        .background(
            background::Builder::new("Bounty Hunter")
                .add_trade(Skill::new("Engineering").with_mastery(Mastery::Novice))
                .add_skill(Skill::new("Perception").with_mastery(Mastery::Novice))
                .add_language_fluency(LanguageFluency::common())
                .build()?,
        )
        .attributes(
            AttributesBuilder::new()
                .prime(Attribute {
                    base_score: 4,
                    save_proficiency: false,
                    skills: vec![],
                })
                .might(Attribute {
                    base_score: 0,
                    save_proficiency: false,
                    skills: vec![],
                })
                .agility(Attribute {
                    base_score: 1,
                    save_proficiency: true,
                    skills: vec![],
                })
                .charisma(Attribute::new().with_base_score(0))
                .intelligence(Attribute::new().with_base_score(3).with_save_proficiency())
                .build()?,
        )
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

    println!("{character:#?}");

    Ok(())
}
