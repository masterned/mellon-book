use core::fmt;
use std::error::Error;

use uuid::Uuid;

use crate::utils::{FieldAggregator, SwapResult};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Item {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeaponType {
    Melee,
    Ranged,
}

impl WeaponType {
    pub fn compatible_with_style(self, style: WeaponStyle) -> bool {
        match self {
            WeaponType::Melee => !matches!(style, WeaponStyle::Bow | WeaponStyle::Crossbow),
            WeaponType::Ranged => matches!(style, WeaponStyle::Bow | WeaponStyle::Crossbow),
        }
    }

    pub fn compatible_with_property(self, property: WeaponProperty) -> bool {
        match self {
            WeaponType::Melee => property.is_melee_property(),
            WeaponType::Ranged => property.is_ranged_property(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeaponStyle {
    Axe,
    Bow,
    Chained,
    Crossbow,
    Fist,
    Hammer,
    Pick,
    Spear,
    Staff,
    Sword,
    Whip,
}

impl WeaponStyle {
    pub fn compatible_with_type(self, weapon_type: WeaponType) -> bool {
        weapon_type.compatible_with_style(self)
    }

    pub fn default_damage_type(self) -> Option<DamageType> {
        match self {
            WeaponStyle::Axe | WeaponStyle::Sword | WeaponStyle::Whip => Some(DamageType::Slashing),
            WeaponStyle::Bow | WeaponStyle::Crossbow | WeaponStyle::Pick | WeaponStyle::Spear => {
                Some(DamageType::Piercing)
            }
            WeaponStyle::Chained | WeaponStyle::Hammer | WeaponStyle::Staff => {
                Some(DamageType::Bludgeoning)
            }
            WeaponStyle::Fist => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeaponProperty {
    Ammo,
    Concealable,
    Guard,
    Heavy,
    Impact,
    LongRanged,
    MultiFaceted(WeaponStyle),
    Reach,
    Reload,
    Silent,
    Toss,
    Thrown,
    TwoHanded,
    Unwieldy,
    Versatile,
    Returning,
    Capture,
}

impl WeaponProperty {
    pub fn get_cost(self) -> isize {
        match self {
            WeaponProperty::Ammo => 0,
            WeaponProperty::Concealable => 1,
            WeaponProperty::Guard => 1,
            WeaponProperty::Heavy => 2,
            WeaponProperty::Impact => 1,
            WeaponProperty::LongRanged => 1,
            WeaponProperty::MultiFaceted(_) => 1,
            WeaponProperty::Reach => 1,
            WeaponProperty::Reload => 0,
            WeaponProperty::Silent => 1,
            WeaponProperty::Toss => 1,
            WeaponProperty::Thrown => 1,
            WeaponProperty::TwoHanded => -1,
            WeaponProperty::Unwieldy => -1,
            WeaponProperty::Versatile => 1,
            WeaponProperty::Returning => 1,
            WeaponProperty::Capture => 0,
        }
    }

    pub fn is_melee_property(self) -> bool {
        matches!(
            self,
            Self::Concealable
                | Self::Guard
                | Self::Heavy
                | Self::Impact
                | Self::MultiFaceted(_)
                | Self::Reach
                | Self::Silent
                | Self::Toss
                | Self::Thrown
                | Self::TwoHanded
                | Self::Unwieldy
                | Self::Versatile
        )
    }

    pub fn is_ranged_property(self) -> bool {
        matches!(
            self,
            Self::Ammo
                | Self::Heavy
                | Self::Impact
                | Self::LongRanged
                | Self::Reload
                | Self::TwoHanded
                | Self::Unwieldy
        )
    }

    pub fn compatible_with_weapon_type(self, weapon_type: WeaponType) -> bool {
        weapon_type.compatible_with_property(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WeaponBuildError {
    MissingField(Vec<String>),
    IncompatibleStyle(WeaponStyle, WeaponType),
    IncompatibleProperty(WeaponProperty, WeaponType),
    DuplicateProperty(WeaponProperty),
    MissingProperty(WeaponProperty),
    PropertyRequiresStyle(WeaponProperty, WeaponStyle),
}

impl fmt::Display for WeaponBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to build Weapon: {}",
            match self {
                WeaponBuildError::MissingField(fields) =>
                    format!("missing field(s): `{}`", fields.join("`, `")),
                WeaponBuildError::IncompatibleStyle(style, weapon_type) =>
                    format!("{style:?} style incompatible with a {weapon_type:?} weapon"),
                WeaponBuildError::IncompatibleProperty(property, weapon_type) =>
                    format!("{property:?} property incompatible with a {weapon_type:?} weapon"),
                WeaponBuildError::DuplicateProperty(weapon_property) =>
                    format!("contains duplicated property `{weapon_property:?}`"),
                WeaponBuildError::MissingProperty(weapon_property) =>
                    format!("property `{weapon_property:?}` not present"),
                WeaponBuildError::PropertyRequiresStyle(weapon_property, weapon_style) =>
                    format!("`{weapon_property:?}` property requires `{weapon_style:?}` style"),
            }
        )
    }
}

impl Error for WeaponBuildError {}

impl TryFrom<FieldAggregator> for WeaponBuildError {
    type Error = ();

    fn try_from(value: FieldAggregator) -> Result<Self, ()> {
        value
            .0
            .map(|fields: Vec<&'static str>| {
                WeaponBuildError::MissingField(
                    fields
                        .iter()
                        .map(|field| field.parse().expect("I have no clue how you got here..."))
                        .collect(),
                )
            })
            .ok_or(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeaponBuilder {
    pub weapon_type: Option<WeaponType>,
    pub style: Option<WeaponStyle>,
    pub damage_type: Option<DamageType>,
    pub properties: Option<Vec<WeaponProperty>>,
    pub max_points: usize,
}

impl WeaponBuilder {
    pub fn new() -> Self {
        Self {
            max_points: 2,
            ..Default::default()
        }
    }

    pub fn set_weapon_type(&mut self, weapon_type: WeaponType) -> Result<(), WeaponBuildError> {
        if let Some(style) = self.style {
            if !style.compatible_with_type(weapon_type) {
                return Err(WeaponBuildError::IncompatibleStyle(style, weapon_type));
            }
        }

        let _ = self.weapon_type.insert(weapon_type);

        Ok(())
    }

    pub fn with_weapon_type(mut self, weapon_type: WeaponType) -> Result<Self, WeaponBuildError> {
        self.set_weapon_type(weapon_type)?;

        Ok(self)
    }

    pub fn with_melee(mut self) -> Result<Self, WeaponBuildError> {
        self.set_weapon_type(WeaponType::Melee)?;

        Ok(self)
    }

    pub fn new_melee() -> Self {
        Self::new().with_melee().unwrap()
    }

    pub fn with_ranged(mut self) -> Result<Self, WeaponBuildError> {
        self.set_weapon_type(WeaponType::Ranged)?;

        Ok(self)
    }

    pub fn new_ranged() -> Self {
        Self::new()
            .with_ranged()
            .unwrap()
            .with_properties(&[
                WeaponProperty::Ammo,
                WeaponProperty::TwoHanded,
                WeaponProperty::Unwieldy,
            ])
            .unwrap()
    }

    pub fn set_style(&mut self, style: WeaponStyle) -> Result<(), WeaponBuildError> {
        if let Some(weapon_type) = self.weapon_type {
            if !weapon_type.compatible_with_style(style) {
                return Err(WeaponBuildError::IncompatibleStyle(style, weapon_type));
            }
        }

        if self.damage_type.is_none() {
            self.damage_type = style.default_damage_type();
        }

        let _ = self.style.insert(style);

        Ok(())
    }

    pub fn with_style(mut self, style: WeaponStyle) -> Result<Self, WeaponBuildError> {
        self.set_style(style)?;

        Ok(self)
    }

    pub fn set_damage_type(&mut self, damage_type: DamageType) {
        let _ = self.damage_type.insert(damage_type);
    }

    pub fn with_damage_type(mut self, damage_type: DamageType) -> Self {
        self.set_damage_type(damage_type);

        self
    }

    pub fn add_property(&mut self, property: WeaponProperty) -> Result<(), WeaponBuildError> {
        if self
            .properties
            .as_ref()
            .is_some_and(|ps| ps.contains(&property))
        {
            return Err(WeaponBuildError::DuplicateProperty(property));
        }

        if let Some(weapon_type) = self.weapon_type {
            if !weapon_type.compatible_with_property(property) {
                return Err(WeaponBuildError::IncompatibleProperty(
                    property,
                    weapon_type,
                ));
            }
        }

        self.properties.get_or_insert_default().push(property);

        Ok(())
    }

    pub fn with_property(mut self, property: WeaponProperty) -> Result<Self, WeaponBuildError> {
        self.add_property(property)?;

        Ok(self)
    }

    pub fn with_properties(
        mut self,
        properties: &[WeaponProperty],
    ) -> Result<Self, WeaponBuildError> {
        for &property in properties {
            self.add_property(property)?;
        }

        Ok(self)
    }

    pub fn remove_property(&mut self, property: WeaponProperty) -> Result<(), WeaponBuildError> {
        if let Some(properties) = self.properties.as_mut() {
            let index = properties
                .iter()
                .position(|&p| p == property)
                .ok_or(WeaponBuildError::MissingProperty(property))?;
            properties.remove(index);

            Ok(())
        } else {
            Err(WeaponBuildError::MissingField(vec!["properties".into()]))
        }
    }

    pub fn without_property(mut self, property: WeaponProperty) -> Result<Self, WeaponBuildError> {
        self.remove_property(property)?;

        Ok(self)
    }

    pub fn build(self) -> Result<Weapon, WeaponBuildError> {
        let mut fa = FieldAggregator::new();

        fa.field_check(&self.weapon_type, "weapon_type");
        fa.field_check(&self.style, "style");
        fa.field_check(&self.damage_type, "damage_type");

        WeaponBuildError::try_from(fa).swap()?;

        let weapon_type = self.weapon_type.unwrap();
        let style = self.style.unwrap();

        weapon_type
            .compatible_with_style(style)
            .then_some(Weapon {
                uuid: Uuid::new_v4(),
                weapon_type,
                style,
                damage_type: self.damage_type.unwrap(),
                properties: self.properties.unwrap_or_default(),
            })
            .ok_or(WeaponBuildError::IncompatibleStyle(style, weapon_type))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Weapon {
    pub uuid: Uuid,
    pub weapon_type: WeaponType,
    pub style: WeaponStyle,
    pub damage_type: DamageType,
    pub properties: Vec<WeaponProperty>,
}

impl TryFrom<WeaponBuilder> for Weapon {
    type Error = WeaponBuildError;

    fn try_from(value: WeaponBuilder) -> Result<Self, Self::Error> {
        value.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _weapon_type_and_style_must_be_compatible() -> Result<(), Box<dyn Error>> {
        assert!(WeaponType::Melee.compatible_with_style(WeaponStyle::Axe));
        assert!(!WeaponType::Ranged.compatible_with_style(WeaponStyle::Axe));

        let mut builder = WeaponBuilder::new().with_style(WeaponStyle::Bow)?;
        builder.weapon_type = Some(WeaponType::Melee);

        assert_eq!(
            builder.build(),
            Err(WeaponBuildError::IncompatibleStyle(
                WeaponStyle::Bow,
                WeaponType::Melee
            ))
        );

        Ok(())
    }

    #[test]
    fn _cannot_add_duplicate_property() -> Result<(), Box<dyn Error>> {
        let builder = WeaponBuilder::new().with_property(WeaponProperty::Ammo)?;

        assert_eq!(
            builder.with_property(WeaponProperty::Ammo),
            Err(WeaponBuildError::DuplicateProperty(WeaponProperty::Ammo))
        );

        Ok(())
    }

    #[test]
    fn _cannot_remove_property_if_properties_is_none() -> Result<(), Box<dyn Error>> {
        let mut builder = WeaponBuilder::new();

        assert_eq!(
            builder.remove_property(WeaponProperty::Ammo),
            Err(WeaponBuildError::MissingField(vec!["properties".into()]))
        );

        Ok(())
    }

    #[test]
    fn _cannot_remove_property_if_property_not_present() -> Result<(), Box<dyn Error>> {
        let mut builder = WeaponBuilder::new_melee().with_property(WeaponProperty::Impact)?;

        assert_eq!(
            builder.remove_property(WeaponProperty::Ammo),
            Err(WeaponBuildError::MissingProperty(WeaponProperty::Ammo))
        );

        Ok(())
    }

    #[test]
    fn _ranged_weapons_start_with_ammo_two_handed_and_unwieldy_properties() {
        let ranged_weapon_builder = WeaponBuilder::new_ranged();

        assert_eq!(
            ranged_weapon_builder.properties,
            vec![
                WeaponProperty::Ammo,
                WeaponProperty::TwoHanded,
                WeaponProperty::Unwieldy
            ]
            .into()
        );
    }

    #[test]
    fn _default_damage_type_based_on_style() {
        let sword = WeaponBuilder::new_melee()
            .with_style(WeaponStyle::Sword)
            .unwrap();

        assert_eq!(sword.damage_type, Some(DamageType::Slashing));

        let hammer = WeaponBuilder::new_melee()
            .with_style(WeaponStyle::Hammer)
            .unwrap();

        assert_eq!(hammer.damage_type, Some(DamageType::Bludgeoning));

        let pick = WeaponBuilder::new_melee()
            .with_style(WeaponStyle::Pick)
            .unwrap();

        assert_eq!(pick.damage_type, Some(DamageType::Piercing));
    }

    #[test]
    fn _do_not_override_damage_type_with_default_if_damage_type_already_set(
    ) -> Result<(), Box<dyn Error>> {
        let builder = WeaponBuilder::new_melee().with_damage_type(DamageType::Bludgeoning);
        let sword = builder.with_style(WeaponStyle::Sword)?;

        assert_eq!(sword.damage_type, Some(DamageType::Bludgeoning));

        Ok(())
    }

    #[test]
    fn _enforces_all_required_fields_to_build_weapon() -> Result<(), Box<dyn Error>> {
        let builder = WeaponBuilder::new();

        assert_eq!(
            builder.clone().build(),
            Err(WeaponBuildError::MissingField(vec![
                "weapon_type".into(),
                "style".into(),
                "damage_type".into(),
            ]))
        );

        let builder = builder.with_melee()?;

        assert_eq!(
            builder.clone().build(),
            Err(WeaponBuildError::MissingField(vec![
                "style".into(),
                "damage_type".into(),
            ]))
        );

        let weapon = builder.with_style(WeaponStyle::Sword)?.build()?;

        assert_eq!(
            Weapon {
                uuid: weapon.uuid,
                weapon_type: WeaponType::Melee,
                style: WeaponStyle::Sword,
                damage_type: DamageType::Slashing,
                properties: vec![]
            },
            weapon
        );

        Ok(())
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _ranged_weapons_have_longer_ranges_than_melee_weapons() {
        todo!()
    }

    #[test]
    fn _fist_weapon_does_not_have_default_style() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            WeaponBuilder::new_melee()
                .with_style(WeaponStyle::Fist)?
                .damage_type,
            None
        );

        Ok(())
    }

    #[test]
    fn _weapon_type_and_property_must_be_compatible() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            WeaponBuilder::new_melee().with_property(WeaponProperty::LongRanged),
            Err(WeaponBuildError::IncompatibleProperty(
                WeaponProperty::LongRanged,
                WeaponType::Melee
            ))
        );

        assert_eq!(
            WeaponBuilder::new_ranged().with_property(WeaponProperty::Reach),
            Err(WeaponBuildError::IncompatibleProperty(
                WeaponProperty::Reach,
                WeaponType::Ranged
            ))
        );

        assert_eq!(
            WeaponBuilder::new_melee().with_property(WeaponProperty::Heavy),
            Ok(WeaponBuilder {
                weapon_type: Some(WeaponType::Melee),
                properties: Some(vec![WeaponProperty::Heavy]),
                ..WeaponBuilder::new()
            })
        );

        assert_eq!(
            WeaponBuilder::new()
                .with_weapon_type(WeaponType::Ranged)?
                .with_property(WeaponProperty::Heavy),
            Ok(WeaponBuilder {
                weapon_type: Some(WeaponType::Ranged),
                properties: Some(vec![WeaponProperty::Heavy]),
                ..WeaponBuilder::new()
            })
        );

        Ok(())
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _properties_requiring_other_properties_must_be_enforced() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _enforce_style_requirements_for_properties() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _properties_with_exclusion_rules_most_be_enforced() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _enforce_weapon_type_required_properties() {
        todo!()
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _cannot_exceed_available_points_on_weapon_build() {
        todo!()
    }
}
