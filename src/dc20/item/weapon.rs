use crate::{
    dc20::Range,
    utils::{FieldAggregator, Logical, SwapResult},
};
use std::{error::Error, fmt};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeaponType {
    Melee,
    Ranged,
}

impl WeaponType {
    #[must_use]
    pub fn compatible_with_style(self, style: WeaponStyle) -> bool {
        match self {
            WeaponType::Melee => !matches!(style, WeaponStyle::Bow | WeaponStyle::Crossbow),
            WeaponType::Ranged => matches!(style, WeaponStyle::Bow | WeaponStyle::Crossbow),
        }
    }

    #[must_use]
    pub fn compatible_with_property(self, property: WeaponProperty) -> bool {
        match self {
            WeaponType::Melee => property.is_melee_property(),
            WeaponType::Ranged => property.is_ranged_property(),
        }
    }

    #[must_use]
    pub fn default_base_range(&self) -> Range {
        match self {
            WeaponType::Melee => Range::Spaces(1),
            WeaponType::Ranged => Range::Spaces(5),
        }
    }
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
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
    #[must_use]
    pub fn compatible_with_type(self, weapon_type: WeaponType) -> bool {
        weapon_type.compatible_with_style(self)
    }

    #[must_use]
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

impl fmt::Display for WeaponStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
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
    #[must_use]
    pub fn get_cost(self) -> isize {
        match self {
            WeaponProperty::TwoHanded | WeaponProperty::Unwieldy => -1,
            WeaponProperty::Ammo | WeaponProperty::Reload | WeaponProperty::Capture => 0,
            WeaponProperty::Concealable
            | WeaponProperty::Guard
            | WeaponProperty::Impact
            | WeaponProperty::LongRanged
            | WeaponProperty::MultiFaceted(_)
            | WeaponProperty::Returning
            | WeaponProperty::Silent
            | WeaponProperty::Toss
            | WeaponProperty::Thrown
            | WeaponProperty::Versatile
            | WeaponProperty::Reach => 1,
            WeaponProperty::Heavy => 2,
        }
    }

    #[must_use]
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
                | Self::Returning
                | Self::Capture
        )
    }

    #[must_use]
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

    #[must_use]
    pub fn compatible_with_weapon_type(self, weapon_type: WeaponType) -> bool {
        weapon_type.compatible_with_property(self)
    }

    #[must_use]
    pub fn get_property_dependency(self) -> Option<Self> {
        match self {
            Self::Heavy => Some(Self::TwoHanded),
            Self::Thrown | Self::Returning => Some(Self::Toss),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_compatible_with_style(self, style: WeaponStyle) -> bool {
        match self {
            Self::Capture => matches!(style, WeaponStyle::Chained | WeaponStyle::Whip),
            Self::MultiFaceted(s) => s != style,
            _ => true,
        }
    }

    #[must_use]
    pub fn get_style_dependency(&self) -> Option<Logical<WeaponStyle>> {
        match self {
            Self::Capture => {
                Some(Logical::Unit(WeaponStyle::Chained).or(Logical::Unit(WeaponStyle::Whip)))
            }
            _ => None,
        }
    }
}

impl fmt::Display for WeaponProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<WeaponStyle> for Logical<WeaponStyle> {
    fn from(value: WeaponStyle) -> Self {
        Logical::Unit(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WeaponBuildError {
    MissingField(Vec<String>),
    IncompatibleStyle(WeaponStyle, WeaponType),
    IncompatibleProperty(WeaponProperty, WeaponType),
    DuplicateProperty(WeaponProperty),
    MissingProperty(Vec<WeaponProperty>),
    MissingStyleDependencies(Logical<WeaponStyle>),
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
                    format!("{style} style incompatible with a {weapon_type} weapon"),
                WeaponBuildError::IncompatibleProperty(property, weapon_type) =>
                    format!("{property} property incompatible with a {weapon_type} weapon"),
                WeaponBuildError::DuplicateProperty(weapon_property) =>
                    format!("contains duplicated property `{weapon_property}`"),
                WeaponBuildError::MissingProperty(properties) => format!(
                    "`{}` property(ies) required",
                    properties
                        .iter()
                        .map(WeaponProperty::to_string)
                        .collect::<Vec<_>>()
                        .join("`, `")
                ),
                WeaponBuildError::MissingStyleDependencies(dependencies) =>
                    format!("missing style dependencies: {dependencies}"),
            }
        )
    }
}

impl Error for WeaponBuildError {}

impl TryFrom<FieldAggregator> for WeaponBuildError {
    type Error = ();

    fn try_from(value: FieldAggregator) -> std::result::Result<Self, ()> {
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

type Result<T> = std::result::Result<T, WeaponBuildError>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeaponBuilder {
    pub weapon_type: Option<WeaponType>,
    pub style: Option<WeaponStyle>,
    pub damage_type: Option<DamageType>,
    pub properties: Option<Vec<WeaponProperty>>,
    pub base_range: Option<Range>,
    pub max_points: usize,
}

impl WeaponBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            max_points: 2,
            ..Default::default()
        }
    }

    pub fn weapon_type(&mut self, weapon_type: WeaponType) -> Result<&mut Self> {
        if let Some(style) = self.style {
            if !style.compatible_with_type(weapon_type) {
                Err(WeaponBuildError::IncompatibleStyle(style, weapon_type))?;
            }
        }

        self.weapon_type_unchecked(weapon_type);

        Ok(self)
    }

    pub fn weapon_type_unchecked(&mut self, weapon_type: WeaponType) -> &mut Self {
        if self.base_range.is_none() {
            self.base_range_unchecked(weapon_type.default_base_range());
        }

        let _ = self.weapon_type.insert(weapon_type);

        self
    }

    #[must_use]
    pub fn new_melee() -> Self {
        Self::new()
            .weapon_type_unchecked(WeaponType::Melee)
            .to_owned()
    }

    #[must_use]
    pub fn new_ranged() -> Self {
        Self::new()
            .weapon_type_unchecked(WeaponType::Ranged)
            .add_properties_unchecked(&[
                WeaponProperty::Ammo,
                WeaponProperty::TwoHanded,
                WeaponProperty::Unwieldy,
            ])
            .to_owned()
    }

    pub fn base_range(&mut self, base_range: Range) -> Result<&mut Self> {
        self.base_range_unchecked(base_range);

        Ok(self)
    }

    pub fn base_range_unchecked(&mut self, base_range: Range) -> &mut Self {
        let _ = self.base_range.insert(base_range);

        self
    }

    pub fn style(&mut self, style: WeaponStyle) -> Result<&mut Self> {
        if let Some(weapon_type) = self.weapon_type {
            if !weapon_type.compatible_with_style(style) {
                return Err(WeaponBuildError::IncompatibleStyle(style, weapon_type));
            }
        }

        self.style_unchecked(style);

        Ok(self)
    }

    pub fn style_unchecked(&mut self, style: WeaponStyle) -> &mut Self {
        if self.damage_type.is_none() {
            if let Some(default_damage_type) = style.default_damage_type() {
                self.damage_type_unchecked(default_damage_type);
            }
        }

        let _ = self.style.insert(style);

        self
    }

    pub fn damage_type(&mut self, damage_type: DamageType) -> Result<&mut Self> {
        self.damage_type_unchecked(damage_type);

        Ok(self)
    }

    pub fn damage_type_unchecked(&mut self, damage_type: DamageType) -> &mut Self {
        let _ = self.damage_type.insert(damage_type);

        self
    }

    pub fn add_property(&mut self, property: WeaponProperty) -> Result<&mut Self> {
        if self
            .properties
            .as_ref()
            .is_some_and(|ps| ps.contains(&property))
        {
            Err(WeaponBuildError::DuplicateProperty(property))?;
        }

        if let Some(weapon_type) = self.weapon_type {
            if !weapon_type.compatible_with_property(property) {
                Err(WeaponBuildError::IncompatibleProperty(
                    property,
                    weapon_type,
                ))?;
            }
        }

        self.add_property_unchecked(property);

        Ok(self)
    }

    pub fn add_property_unchecked(&mut self, property: WeaponProperty) {
        self.properties.get_or_insert_default().push(property);
    }

    pub fn add_properties(&mut self, properties: &[WeaponProperty]) -> Result<&mut Self> {
        for property in properties {
            self.add_property(*property)?;
        }

        Ok(self)
    }

    pub fn add_properties_unchecked(&mut self, properties: &[WeaponProperty]) -> &mut Self {
        for property in properties {
            self.add_property_unchecked(*property);
        }

        self
    }

    pub fn remove_property(&mut self, property: WeaponProperty) -> Result<&mut Self> {
        let properties = self
            .properties
            .as_mut()
            .ok_or(WeaponBuildError::MissingField(vec!["properties".into()]))?;

        let index = properties
            .iter()
            .position(|&p| p == property)
            .ok_or(WeaponBuildError::MissingProperty(vec![property]))?;

        properties.remove(index);

        Ok(self)
    }

    fn missing_dependency_properties(&self) -> Vec<WeaponProperty> {
        self.properties.as_ref().map_or(vec![], |props| {
            props
                .iter()
                .filter_map(|p| p.get_property_dependency())
                .filter(|dep| !self.properties.as_ref().unwrap().contains(dep))
                .collect()
        })
    }

    fn get_style_dependencies(&self) -> Option<Logical<WeaponStyle>> {
        self.properties.as_ref().and_then(|ps| {
            ps.iter()
                .filter_map(WeaponProperty::get_style_dependency)
                .fold(None, |acc: Option<Logical<WeaponStyle>>, style| {
                    if let Some(acc) = acc {
                        Some(acc.and(style))
                    } else {
                        Some(style)
                    }
                })
        })
    }

    fn meets_style_requirements(&self, deps: &Logical<WeaponStyle>) -> bool {
        match deps {
            Logical::Unit(dep) => {
                self.style.is_some_and(|style| style == *dep)
                    || self
                        .properties
                        .as_ref()
                        .is_some_and(|ps| ps.contains(&WeaponProperty::MultiFaceted(*dep)))
            }
            Logical::Or(left, right) => {
                self.meets_style_requirements(left) || self.meets_style_requirements(right)
            }
            Logical::And(left, right) => {
                self.meets_style_requirements(left) && self.meets_style_requirements(right)
            }
        }
    }

    pub fn build(&self) -> Result<Weapon> {
        let mut fa = FieldAggregator::new();

        fa.field_check(&self.weapon_type, "weapon_type");
        fa.field_check(&self.style, "style");
        fa.field_check(&self.damage_type, "damage_type");
        fa.field_check(&self.base_range, "base_range");

        WeaponBuildError::try_from(fa).swap()?;

        let missing_deps = self.missing_dependency_properties();
        if !missing_deps.is_empty() {
            Err(WeaponBuildError::MissingProperty(missing_deps))?;
        }

        let weapon_type = self
            .weapon_type
            .ok_or(WeaponBuildError::MissingField(vec!["weapon_type".into()]))?;
        let style = self
            .style
            .ok_or(WeaponBuildError::MissingField(vec!["style".into()]))?;

        if let Some(style_dependencies) = self.get_style_dependencies() {
            if !self.meets_style_requirements(&style_dependencies) {
                Err(WeaponBuildError::MissingStyleDependencies(
                    style_dependencies,
                ))?;
            }
        }

        weapon_type
            .compatible_with_style(style)
            .then_some(Weapon {
                uuid: Uuid::new_v4(),
                weapon_type,
                style,
                damage_type: self
                    .damage_type
                    .ok_or(WeaponBuildError::MissingField(vec!["damage_type".into()]))?,
                properties: self.properties.clone().unwrap_or_default(),
                base_range: self
                    .base_range
                    .ok_or(WeaponBuildError::MissingField(vec!["base_range".into()]))?,
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
    pub base_range: Range,
}

impl Weapon {
    #[must_use]
    pub fn get_range(&self) -> Range {
        Range::Caster
    }
}

impl TryFrom<WeaponBuilder> for Weapon {
    type Error = WeaponBuildError;

    fn try_from(value: WeaponBuilder) -> std::result::Result<Self, Self::Error> {
        value.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _weapon_type_and_style_must_be_compatible() -> Result<()> {
        assert!(WeaponType::Melee.compatible_with_style(WeaponStyle::Axe));
        assert!(!WeaponType::Ranged.compatible_with_style(WeaponStyle::Axe));

        let mut builder = WeaponBuilder::new().style(WeaponStyle::Bow)?.to_owned();
        builder.weapon_type = Some(WeaponType::Melee);
        builder.base_range = Some(Range::Spaces(1));

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
    fn _cannot_add_duplicate_property() -> Result<()> {
        assert_eq!(
            WeaponBuilder::new()
                .add_property(WeaponProperty::Ammo)?
                .add_property(WeaponProperty::Ammo),
            Err(WeaponBuildError::DuplicateProperty(WeaponProperty::Ammo))
        );

        Ok(())
    }

    #[test]
    fn _cannot_remove_property_if_properties_is_none() {
        assert_eq!(
            WeaponBuilder::new().remove_property(WeaponProperty::Ammo),
            Err(WeaponBuildError::MissingField(vec!["properties".into()]))
        );
    }

    #[test]
    fn _cannot_remove_property_if_property_not_present() -> Result<()> {
        assert_eq!(
            WeaponBuilder::new_melee()
                .add_property(WeaponProperty::Impact)?
                .remove_property(WeaponProperty::Ammo),
            Err(WeaponBuildError::MissingProperty(vec![
                WeaponProperty::Ammo
            ]))
        );

        Ok(())
    }

    #[test]
    fn _ranged_weapons_start_with_ammo_two_handed_and_unwieldy_properties() {
        assert_eq!(
            WeaponBuilder::new_ranged().properties,
            vec![
                WeaponProperty::Ammo,
                WeaponProperty::TwoHanded,
                WeaponProperty::Unwieldy
            ]
            .into()
        );
    }

    #[test]
    fn _default_damage_type_based_on_style() -> Result<()> {
        assert_eq!(
            WeaponBuilder::new_melee()
                .style(WeaponStyle::Sword)?
                .damage_type,
            Some(DamageType::Slashing)
        );

        assert_eq!(
            WeaponBuilder::new_melee()
                .style(WeaponStyle::Hammer)?
                .damage_type,
            Some(DamageType::Bludgeoning)
        );

        assert_eq!(
            WeaponBuilder::new_melee()
                .style(WeaponStyle::Pick)?
                .damage_type,
            Some(DamageType::Piercing)
        );

        Ok(())
    }

    #[test]
    fn _do_not_override_damage_type_with_default_if_damage_type_already_set() -> Result<()> {
        assert_eq!(
            WeaponBuilder::new_melee()
                .damage_type(DamageType::Bludgeoning)?
                .style(WeaponStyle::Sword)?
                .damage_type,
            Some(DamageType::Bludgeoning)
        );

        Ok(())
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _set_weapon_type_based_on_style_if_unset() -> Result<()> {
        todo!()
    }

    #[test]
    fn _enforces_all_required_fields_to_build_weapon() -> Result<()> {
        let mut builder = WeaponBuilder::new();

        assert_eq!(
            builder.clone().build(),
            Err(WeaponBuildError::MissingField(vec![
                "weapon_type".into(),
                "style".into(),
                "damage_type".into(),
                "base_range".into()
            ]))
        );

        let builder = builder.weapon_type(WeaponType::Melee)?;
        dbg!(&builder);
        assert_eq!(
            builder.clone().build(),
            Err(WeaponBuildError::MissingField(vec![
                "style".into(),
                "damage_type".into(),
            ]))
        );

        let weapon = builder.style(WeaponStyle::Sword)?.build()?;

        assert_eq!(
            Weapon {
                uuid: weapon.uuid,
                weapon_type: WeaponType::Melee,
                style: WeaponStyle::Sword,
                damage_type: DamageType::Slashing,
                properties: vec![],
                base_range: Range::Spaces(1)
            },
            weapon
        );

        Ok(())
    }

    #[test]
    fn _derive_default_base_range_from_weapon_type() {
        assert_eq!(WeaponType::Melee.default_base_range(), Range::Spaces(1));
        assert_eq!(
            WeaponBuilder::new_melee().base_range.unwrap(),
            Range::Spaces(1)
        );

        assert_eq!(WeaponType::Ranged.default_base_range(), Range::Spaces(5));
        assert_eq!(
            WeaponBuilder::new_ranged().base_range.unwrap(),
            Range::Spaces(5)
        );
    }

    #[test]
    fn _fist_weapon_does_not_have_default_style() -> Result<()> {
        assert_eq!(
            WeaponBuilder::new_melee()
                .style(WeaponStyle::Fist)?
                .damage_type,
            None
        );

        Ok(())
    }

    #[test]
    fn _weapon_type_and_property_must_be_compatible() -> Result<()> {
        assert_eq!(
            WeaponBuilder::new_melee().add_property(WeaponProperty::LongRanged),
            Err(WeaponBuildError::IncompatibleProperty(
                WeaponProperty::LongRanged,
                WeaponType::Melee
            ))
        );

        assert_eq!(
            WeaponBuilder::new_ranged().add_property(WeaponProperty::Reach),
            Err(WeaponBuildError::IncompatibleProperty(
                WeaponProperty::Reach,
                WeaponType::Ranged
            ))
        );

        assert_eq!(
            WeaponBuilder::new_melee()
                .add_property(WeaponProperty::Heavy)?
                .to_owned(),
            WeaponBuilder {
                weapon_type: Some(WeaponType::Melee),
                properties: Some(vec![WeaponProperty::Heavy]),
                base_range: Some(Range::Spaces(1)),
                ..WeaponBuilder::new()
            }
        );

        assert_eq!(
            WeaponBuilder::new()
                .weapon_type(WeaponType::Ranged)?
                .add_property(WeaponProperty::Heavy)?
                .to_owned(),
            WeaponBuilder {
                weapon_type: Some(WeaponType::Ranged),
                properties: Some(vec![WeaponProperty::Heavy]),
                base_range: Some(Range::Spaces(5)),
                ..WeaponBuilder::new()
            }
        );

        Ok(())
    }

    #[test]
    fn _properties_requiring_other_properties_must_be_enforced() -> Result<()> {
        let mut heavy = WeaponBuilder::new_melee()
            .add_property(WeaponProperty::Heavy)?
            .style(WeaponStyle::Axe)?
            .to_owned();

        assert_eq!(
            heavy.clone().build(),
            Err(WeaponBuildError::MissingProperty(vec![
                WeaponProperty::TwoHanded
            ]))
        );

        let heavy = heavy.add_property(WeaponProperty::TwoHanded)?.build()?;

        assert_eq!(
            heavy,
            Weapon {
                uuid: heavy.uuid,
                weapon_type: WeaponType::Melee,
                style: WeaponStyle::Axe,
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
                base_range: Range::Spaces(1)
            },
        );

        assert_eq!(
            WeaponBuilder::new_melee()
                .add_properties(&[WeaponProperty::Heavy, WeaponProperty::Thrown])?
                .style(WeaponStyle::Axe)?
                .build(),
            Err(WeaponBuildError::MissingProperty(vec![
                WeaponProperty::TwoHanded,
                WeaponProperty::Toss
            ]))
        );

        Ok(())
    }

    #[test]
    fn _enforce_style_requirements_for_properties() -> Result<()> {
        let mut capture = WeaponBuilder::new_melee()
            .add_property(WeaponProperty::Capture)?
            .style(WeaponStyle::Hammer)?
            .to_owned();
        dbg!(&capture);

        let weapon_build_attempt = capture.clone().build();
        dbg!(&weapon_build_attempt);

        assert_eq!(
            weapon_build_attempt,
            Err(WeaponBuildError::MissingStyleDependencies(
                Logical::Unit(WeaponStyle::Chained).or(WeaponStyle::Whip.into()),
            ))
        );

        let weapon_build_attempt = capture.clone().style(WeaponStyle::Chained)?.build()?;
        dbg!(&weapon_build_attempt);
        assert_eq!(
            weapon_build_attempt,
            Weapon {
                uuid: weapon_build_attempt.uuid,
                weapon_type: WeaponType::Melee,
                style: WeaponStyle::Chained,
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Capture],
                base_range: Range::Spaces(1)
            }
        );

        let weapon_build_attempt = capture.style(WeaponStyle::Whip)?.build()?;
        dbg!(&weapon_build_attempt);
        assert_eq!(
            weapon_build_attempt,
            Weapon {
                uuid: weapon_build_attempt.uuid,
                weapon_type: WeaponType::Melee,
                style: WeaponStyle::Whip,
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Capture],
                base_range: Range::Spaces(1)
            }
        );

        Ok(())
    }

    #[test]
    fn _weapon_can_meet_style_requirements_using_multi_faceted_property() -> Result<()> {
        let mut urumi = WeaponBuilder::new_melee()
            .style(WeaponStyle::Sword)?
            .add_properties(&[WeaponProperty::Capture])?
            .to_owned();

        assert_eq!(
            urumi.clone().build(),
            Err(WeaponBuildError::MissingStyleDependencies(
                Logical::Unit(WeaponStyle::Chained).or(WeaponStyle::Whip.into())
            ))
        );

        let urumi = urumi
            .add_property(WeaponProperty::MultiFaceted(WeaponStyle::Whip))?
            .build()?;

        assert_eq!(
            urumi,
            Weapon {
                uuid: urumi.uuid,
                weapon_type: WeaponType::Melee,
                style: WeaponStyle::Sword,
                damage_type: DamageType::Slashing,
                properties: vec![
                    WeaponProperty::Capture,
                    WeaponProperty::MultiFaceted(WeaponStyle::Whip),
                ],
                base_range: Range::Spaces(1)
            }
        );

        Ok(())
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _cannot_have_duplicate_style_in_multi_faceted_property() -> Result<()> {
        todo!()
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn _properties_with_exclusion_rules_must_be_enforced() -> Result<()> {
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
