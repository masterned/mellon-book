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

    let spencer = Player::builder()
        .id(player_id)
        .name("Spencer Dent")?
        .build()?;
    spencer.clone().save(&pool).await?;

    let human_uuid = Uuid::from_str("0199366d-d88f-7944-b173-c75f6cd2c5c3")?;
    let human = Ancestry::builder().id(human_uuid).name("Human").build()?;
    human.clone().save(&pool).await?;

    let undying = AncestryTrait::builder()
        .name("Undying")
        .description("You have ADV on Saves against the Doomed Condition")
        .cost(0 as i8)
        .build()?;

    let awareness = Skill::load(&pool, Uuid::from_u128(0x01993a736a8577e183451a57d7c324de)).await?;

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
        .ancestry(human)
        .ancestry_trait(undying)
        .background(
            Background::builder()
                .name("Bounty Hunter")?
                .skill(awareness)
                .trade(
                    Trade::builder()
                        .name("Engineering")
                        .attribute_id(uuid::Uuid::from_u128(0x01993b8556b4774aa4a333bd7f76469e))
                        .build()?,
                )
                .language_fluency(LanguageFluency::common())
                .build()?,
        )
        .attributes(
            Attributes::builder()
                .prime(AttributeLevel {
                    base_score: 4,
                    save_proficiency: false,
                    skills: vec![],
                })
                .might(AttributeLevel {
                    base_score: 0,
                    save_proficiency: false,
                    skills: vec![],
                })
                .agility(AttributeLevel {
                    base_score: 1,
                    save_proficiency: true,
                    skills: vec![],
                })
                .charisma(AttributeLevel::new().with_base_score(0))
                .intelligence(
                    AttributeLevel::new()
                        .with_base_score(3)
                        .with_save_proficiency(),
                )
                .build()?,
        )
        .build()?;

    println!("{character:#?}");

    println!("PD: {:#?}", character.precision_defense());
    println!("AD: {:#?}", character.area_defense());

    Ok(())
}
