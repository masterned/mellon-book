use std::error::Error;

use mellon_book::dc20::*;

fn main() -> Result<(), Box<dyn Error>> {
    let character = CharacterBuilder::default()
        .player_name("Spencer")
        .character_name("Cygnus")
        .class(Class::new("Sorcerer"))
        .ancestry(Ancestry::new("Human/Psyborn"))
        .background(Background::new("Bounty Hunter"))
        .add_stat(Stat {
            name: "Prime".into(),
            score: 4,
            save_proficiency: false,
            skills: vec![Skill {
                name: "Perception".to_string(),
                mastery: Some(Mastery::Novice),
            }],
        })
        .add_stat(Stat {
            name: "Strength".into(),
            score: 0,
            save_proficiency: false,
            skills: vec![],
        })
        .add_stat(Stat {
            name: "Dexterity".into(),
            score: 1,
            save_proficiency: true,
            skills: vec![],
        })
        .add_trade(Skill {
            name: "Engineering".into(),
            mastery: Some(Mastery::Novice),
        })
        .add_language(Language {
            name: "Common".into(),
            fluency: Fluency::Fluent,
        })
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
