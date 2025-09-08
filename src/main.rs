use std::str::FromStr;

use mellon_book::{dc20::*, player::Player};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;

    let player_id = Uuid::from_str("01991836ac9f75898eff73915fd87018").unwrap();

    let (secs, nsecs) = player_id.get_timestamp().unwrap().to_unix();
    let timestamp = chrono::DateTime::from_timestamp(secs as i64, nsecs);
    println!("{timestamp:#?}");

    let spencer = match Player::get_player_by_uuid(&pool, player_id).await {
        Ok(player) => player,
        Err(error) => {
            eprintln!("{error:#?}");

            println!("inserting player into table");

            let player = Player::builder()
                .id(player_id)
                .name("Spencer Dent")?
                .build()?;

            let id = player.clone().save(&pool).await?;

            println!("new player id: {id}");

            player
        }
    };

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

    let psy_trait = AncestryTrait {
        uuid: Uuid::new_v4(),
        cost: 5,
        ..Default::default()
    };

    let psyborn = AncestryInstanceBuilder::from(AncestryEntry {
        uuid: Uuid::new_v4(),
        name: "Psyborn".into(),
        description: "Totally not a mindflayer... promise...".into(),
        available_traits: vec![psy_trait.clone()],
    })
    .add_ancestry_trait(psy_trait)?
    .build()?;

    let character = CharacterBuilder::default()
        .player(spencer)
        .character_name("Cygnus")
        .class(ClassEntry {
            combat_style: vec![CombatStyle::default_spellcasting()],
            available_subclasses: vec![
                SubclassEntry::new("Angelic"),
                SubclassEntry::new("Draconic"),
                SubclassEntry::new("Paragon"),
            ],
            ..ClassEntry::new("Sorcerer")
        })
        .ancestry(Origin::HybridBred(human, psyborn))
        .background(
            Background::builder()
                .name("Bounty Hunter")?
                .trade(Skill::new("Engineering").with_mastery(Mastery::Novice))
                .skill(Skill::new("Perception").with_mastery(Mastery::Novice))
                .language_fluency(LanguageFluency::common())
                .build()?,
        )
        .attributes(
            Attributes::builder()
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
        .build()?;

    println!("{character:#?}");

    println!("PD: {:#?}", character.precision_defense());
    println!("AD: {:#?}", character.area_defense());

    Ok(())
}
