use std::error::Error;

use mellon_book::dc20::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut perception = Skill::new("Perception");
    perception.set_mastery(Mastery::Novice);

    let mut engineering = Skill::new("Engineering");
    engineering.set_mastery(Mastery::Novice);

    let mut common = Language::new("Common");
    common.set_fluency(Fluency::Fluent);

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
            skills: vec![perception],
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
        .add_trade(engineering)
        .add_language(common)
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
