use std::str::FromStr;

use mellon_book::{dc20::*, player::Player};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;

    let player_id = uuid::uuid!("01991836-ac9f-7589-8eff-73915fd87018");

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
        .cost(0_i8)
        .build()?;

    let background =
        Background::load(&pool, Uuid::from_u128(0x01993ea09d21764d9a0b98bb22b619ca)).await?;

    let character = Character::builder()
        .id(Uuid::from_u128(0x166ae11a3d404c618d390415e0cae6bb))
        .player(spencer)
        .character_name("Cygnus")
        .ancestry_trait(undying)
        .background(background)
        .build()?;

    println!("{character:#?}");

    let level = character.load_level(&pool, 3).await?;
    println!("{level:#?}");

    let attributes = level.load_base_attributes(&pool).await?;

    println!("{attributes:#?}");

    println!(
        "PD: {:#?}",
        attributes.precision_defense(level.calc_combat_mastery())
    );
    println!(
        "AD: {:#?}",
        attributes.area_defense(level.calc_combat_mastery())
    );

    let ancestries = level.load_ancestries(&pool).await?;
    println!("ancestry(ies): {:#?}", ancestries);

    let classes = level.load_classes(&pool).await?;
    println!("classes: {classes:#?}");

    for class in classes {
        let available_subclasses = class.load_sublasses(&pool).await?;
        println!("available subclasses: {available_subclasses:#?}");
    }

    let subclasses = level.load_sublasses(&pool).await?;
    println!("subclasses: {subclasses:#?}");

    let background = character.background();

    println!(
        "{} Languages: {:#?}",
        background.name,
        background.load_languages(&pool).await?
    );

    println!(
        "{} Trades: {:#?}",
        background.name,
        background.load_trades(&pool).await?
    );

    let skills = background.load_skills(&pool).await?;
    println!("{} Skills: {:#?}", background.name, skills);

    for skill in skills {
        let attr = skill.load_attribute(&pool).await?;
        println!("{attr:#?}");
    }

    Ok(())
}
