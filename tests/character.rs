use std::error::Error;

use mellon_book::{dc20::*, player::Player};

#[test]
fn _built_character_should_have_name() -> Result<(), Box<dyn Error>> {
    let player = Player::builder().name("Test Player")?.build()?;

    let test_character = Character::builder()
        .player(player)
        .character_name("Test Name")
        .background(Background::builder().name("Soldier")?.build()?)
        .build()?;

    assert_eq!(test_character.character_name(), "Test Name");

    Ok(())
}
