use core::fmt;
use std::error::Error;

use uuid::Uuid;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OriginBuilder {
    pub available_points: isize,
    pub ancestries: Option<Vec<AncestryInstance>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AddAncestryError {
    DuplicateAncestry(String),
}

impl fmt::Display for AddAncestryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to add Ancestry: {}",
            match self {
                AddAncestryError::DuplicateAncestry(ancestry_name) =>
                    format!("attempting to add duplicate Ancestry \"{ancestry_name}\""),
            }
        )
    }
}

impl Error for AddAncestryError {}

impl OriginBuilder {
    pub fn new() -> Self {
        Self {
            available_points: 5,
            ..Self::default()
        }
    }

    pub fn with_ancestry(mut self, ancestry: AncestryInstance) -> Result<Self, AddAncestryError> {
        if self.duplicate_ancestry(&ancestry) {
            return Err(AddAncestryError::DuplicateAncestry(ancestry.name));
        }

        self.ancestries.get_or_insert_default().push(ancestry);

        Ok(self)
    }

    fn duplicate_ancestry(&self, ancestry: &AncestryInstance) -> bool {
        self.ancestries
            .as_ref()
            .is_some_and(|a_s| a_s.contains(ancestry))
    }

    fn total_trait_points(&self) -> isize {
        self.ancestries.as_ref().map_or(0, |a_s| {
            a_s.iter().map(AncestryInstance::total_trait_points).sum()
        })
    }

    fn trait_points_exceeded(&self) -> bool {
        self.total_trait_points() > self.available_points
    }

    fn trait_points_remaining(&self) -> bool {
        self.total_trait_points() < self.available_points
    }

    fn zero_point_trait_count(&self) -> usize {
        self.ancestries.as_ref().map_or(0, |a_s| {
            a_s.iter()
                .fold(0, |acc, a| acc + a.zero_point_traits().len())
        })
    }

    pub fn build(self) -> Result<Origin, OriginBuildError> {
        self.try_into()
    }
}

/// This is used to know whether the user
/// selected to start with 1 or 2 Ancestries
/// when creating their Character. RAW,
/// Characters can only pull from 2 Ancestries
/// unless they use a CustomOrigin.
#[derive(Clone, Debug, PartialEq)]
pub enum Origin {
    PureBred(AncestryInstance),
    HybridBred(AncestryInstance, AncestryInstance),
    CustomOrigin(Vec<AncestryInstance>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginBuildError {
    AncestryMissing,
    PointsExceeded,
    PointsRemaining,
    ZeroPointTraitMissing,
    MultipleZeroPointTraits,
}

impl fmt::Display for OriginBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to build Origin: {}",
            match self {
                OriginBuildError::AncestryMissing => "Requires at least one Ancestry".to_string(),
                OriginBuildError::PointsExceeded => "Too many Trait points used".into(),
                OriginBuildError::PointsRemaining => "All Trait points must be used".into(),
                OriginBuildError::ZeroPointTraitMissing => "Requires one 0-point Trait".into(),
                OriginBuildError::MultipleZeroPointTraits =>
                    "Cannot contain multiple 0-point Traits".into(),
            }
        )
    }
}

impl Error for OriginBuildError {}

impl TryFrom<OriginBuilder> for Origin {
    type Error = OriginBuildError;

    fn try_from(value: OriginBuilder) -> Result<Self, Self::Error> {
        if value.ancestries.is_none() {
            return Err(OriginBuildError::AncestryMissing);
        }
        if value.trait_points_exceeded() {
            return Err(OriginBuildError::PointsExceeded);
        }
        if value.trait_points_remaining() {
            return Err(OriginBuildError::PointsRemaining);
        }
        match value.zero_point_trait_count() {
            0 => {
                return Err(OriginBuildError::ZeroPointTraitMissing);
            }
            count if count > 1 => {
                return Err(OriginBuildError::MultipleZeroPointTraits);
            }
            _ => (),
        }

        let ancestries = value.ancestries.unwrap();
        match ancestries.iter().count() {
            0 => Err(OriginBuildError::AncestryMissing),
            1 => Ok(Origin::PureBred(ancestries[0].clone())),
            2 => Ok(Origin::HybridBred(
                ancestries[0].clone(),
                ancestries[1].clone(),
            )),
            _ => Ok(Origin::CustomOrigin(ancestries)),
        }
    }
}

/// This will be used to reflect the data in the
/// book. When the user is deciding on their
/// Ancestry(ies), the application will use this
/// struct to display the Ancestry's full
/// details.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AncestryEntry {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub available_traits: Vec<AncestryTrait>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AddAncestryTraitError {
    TraitNotAvailable(String),
    DuplicateTrait(String),
}

impl fmt::Display for AddAncestryTraitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to build AncestryInstance: {}",
            match self {
                AddAncestryTraitError::TraitNotAvailable(trait_name) =>
                    format!("Trait `{trait_name}` not available in Entry"),
                AddAncestryTraitError::DuplicateTrait(trait_name) =>
                    format!("Attempting to duplicate `{trait_name}` Trait"),
            }
        )
    }
}

impl Error for AddAncestryTraitError {}

#[derive(Clone, Debug, PartialEq)]
pub struct AncestryInstanceBuilder {
    pub entry: AncestryEntry,
    pub selected_traits: Option<Vec<AncestryTrait>>,
}

impl AncestryInstanceBuilder {
    pub fn add_ancestry_trait(
        mut self,
        ancestry_trait: AncestryTrait,
    ) -> Result<Self, AddAncestryTraitError> {
        if !self.trait_available(&ancestry_trait) {
            return Err(AddAncestryTraitError::TraitNotAvailable(
                ancestry_trait.name,
            ));
        }

        if self.duplicate_trait(&ancestry_trait) {
            return Err(AddAncestryTraitError::DuplicateTrait(ancestry_trait.name));
        }

        self.selected_traits
            .get_or_insert_default()
            .push(ancestry_trait);

        Ok(self)
    }

    fn trait_available(&self, ancestry_trait: &AncestryTrait) -> bool {
        self.entry.available_traits.contains(&ancestry_trait)
    }

    fn duplicate_trait(&self, ancestry_trait: &AncestryTrait) -> bool {
        self.selected_traits
            .as_ref()
            .is_some_and(|t| t.contains(&ancestry_trait))
    }

    pub fn build(self) -> Result<AncestryInstance, InstanceBuildError> {
        self.try_into()
    }
}

impl From<AncestryEntry> for AncestryInstanceBuilder {
    fn from(value: AncestryEntry) -> Self {
        AncestryInstanceBuilder {
            entry: value,
            selected_traits: None,
        }
    }
}

/// This is the specific details the user chose
/// for their Character's Ancestry.
///
/// The reason I'm not tracking all of this in
/// the Character's Origin is so that I can
/// keep the AncestryTraits separated. I don't
/// want to lose track of which Ancestry a given
/// AncestryTrait belongs to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AncestryInstance {
    pub uuid: Uuid,
    pub name: String,
    pub traits: Vec<AncestryTrait>,
}

impl AncestryInstance {
    pub fn with_trait(
        mut self,
        ancestry_trait: AncestryTrait,
    ) -> Result<Self, AddAncestryTraitError> {
        self.add_ancestry_trait(ancestry_trait)?;

        Ok(self)
    }

    pub fn add_ancestry_trait(
        &mut self,
        ancestry_trait: AncestryTrait,
    ) -> Result<(), AddAncestryTraitError> {
        self.traits.push(ancestry_trait);

        Ok(())
    }

    fn total_trait_points(&self) -> isize {
        self.traits.iter().map(|t| t.cost).sum()
    }

    fn zero_point_traits(&self) -> Vec<&AncestryTrait> {
        self.traits.iter().filter(|t| t.cost == 0).collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceBuildError {
    MissingTraits,
}

impl fmt::Display for InstanceBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to build AncestryInstance: {}",
            match self {
                InstanceBuildError::MissingTraits => "requires at least one trait",
            }
        )
    }
}

impl Error for InstanceBuildError {}

impl TryFrom<AncestryInstanceBuilder> for AncestryInstance {
    type Error = InstanceBuildError;

    fn try_from(value: AncestryInstanceBuilder) -> Result<Self, Self::Error> {
        if let Some(selected_traits) = value.selected_traits.clone() {
            Ok(AncestryInstance {
                uuid: Uuid::new_v4(),
                name: value.entry.name,
                traits: selected_traits,
            })
        } else {
            Err(InstanceBuildError::MissingTraits)
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AncestryTrait {
    pub uuid: Uuid,
    pub name: String,
    pub cost: isize,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod origin {
        use super::*;

        #[test]
        #[ignore = "not yet implemented"]
        fn _cannot_have_multiple_ancestries_of_same_entry() {
            todo!()
        }

        #[test]
        fn _cannot_have_duplicate_ancestry() {
            let test_ancestry = AncestryInstance::default();

            let test_builder = OriginBuilder::new();
            assert!(!test_builder.duplicate_ancestry(&test_ancestry));

            let test_builder = OriginBuilder::new()
                .with_ancestry(test_ancestry.clone())
                .unwrap();
            assert!(test_builder.duplicate_ancestry(&test_ancestry));

            assert_eq!(
                OriginBuilder::new()
                    .with_ancestry(test_ancestry.clone())
                    .unwrap()
                    .with_ancestry(test_ancestry),
                Err(AddAncestryError::DuplicateAncestry("".into()))
            )
        }

        #[test]
        fn _cannot_exceed_available_points() -> Result<(), Box<dyn Error>> {
            let test_trait = AncestryTrait {
                cost: 6,
                ..AncestryTrait::default()
            };
            let test_ancestry = AncestryInstance::default();

            let mut test_builder = OriginBuilder::new().with_ancestry(test_ancestry).unwrap();

            assert!(!test_builder.trait_points_exceeded());

            test_builder.ancestries.as_mut().unwrap()[0].add_ancestry_trait(test_trait)?;

            assert!(test_builder.trait_points_exceeded());

            Ok(())
        }

        #[test]
        fn _must_use_all_available_points() -> Result<(), Box<dyn Error>> {
            let test_trait = AncestryTrait {
                cost: 5,
                ..AncestryTrait::default()
            };
            let test_ancestry = AncestryInstance {
                traits: vec![],
                ..Default::default()
            };

            let mut test_builder = OriginBuilder::new().with_ancestry(test_ancestry).unwrap();

            assert!(test_builder.trait_points_remaining());

            test_builder.ancestries.as_mut().unwrap()[0].add_ancestry_trait(test_trait)?;

            assert!(!test_builder.trait_points_remaining());

            Ok(())
        }

        #[test]
        fn _counts_zero_point_traits() -> Result<(), Box<dyn Error>> {
            let test_trait_0 = AncestryTrait::default();
            let test_trait_1 = AncestryTrait {
                uuid: Uuid::new_v4(),
                ..Default::default()
            };
            let test_ancestry = AncestryInstance::default();

            let mut test_builder = OriginBuilder::new().with_ancestry(test_ancestry)?;
            assert_eq!(test_builder.zero_point_trait_count(), 0);

            test_builder.ancestries.as_mut().unwrap()[0].add_ancestry_trait(test_trait_0)?;
            assert_eq!(test_builder.zero_point_trait_count(), 1);

            test_builder.ancestries.as_mut().unwrap()[0].add_ancestry_trait(test_trait_1)?;
            assert_eq!(test_builder.zero_point_trait_count(), 2);

            Ok(())
        }

        #[test]
        fn _builds_correct_origin_based_on_ancestry_count() -> Result<(), Box<dyn Error>> {
            let builder = OriginBuilder::new();
            assert_eq!(
                builder.clone().build(),
                Err(OriginBuildError::AncestryMissing)
            );

            let t0 = AncestryTrait {
                cost: 5,
                ..Default::default()
            };
            let t0_0 = AncestryTrait {
                cost: 0,
                ..Default::default()
            };
            let a0 = AncestryInstance {
                traits: vec![t0.clone(), t0_0.clone()],
                ..Default::default()
            };

            let builder = builder.with_ancestry(a0.clone())?;
            let result = builder.clone().build();
            dbg!(&result);
            assert!(matches!(result, Ok(Origin::PureBred(_))));

            let t1 = AncestryTrait {
                cost: 1,
                ..Default::default()
            };
            let a1 = AncestryInstance {
                traits: vec![t1.clone()],
                ..Default::default()
            };

            let mut builder = builder.with_ancestry(a1.clone())?;
            builder.available_points = 6;
            let result = builder.clone().build();
            dbg!(&result);
            assert!(matches!(result, Ok(Origin::HybridBred(_, _))));

            let t2 = AncestryTrait {
                cost: 2,
                ..Default::default()
            };
            let a2 = AncestryInstance {
                traits: vec![t2.clone()],
                ..Default::default()
            };

            let mut builder = builder.with_ancestry(a2)?;
            builder.available_points = 8;
            let result = builder.build();
            dbg!(&result);
            assert!(matches!(result, Ok(Origin::CustomOrigin(_))));

            Ok(())
        }
    }

    mod instance {
        use super::*;

        #[test]
        fn _cannot_add_traits_missing_from_available_list() {
            let test_trait = AncestryTrait::default();

            let test_builder = AncestryInstanceBuilder::from(AncestryEntry::default());

            assert!(!test_builder.trait_available(&test_trait));

            let test_entry = AncestryEntry {
                available_traits: vec![test_trait.clone()],
                ..Default::default()
            };

            let test_builder = AncestryInstanceBuilder::from(test_entry);

            assert!(test_builder.trait_available(&test_trait))
        }

        #[test]
        fn _cannot_contain_duplicate_traits() {
            let test_trait = AncestryTrait::default();

            let test_entry = AncestryEntry {
                available_traits: vec![test_trait.clone()],
                ..Default::default()
            };

            let test_builder = AncestryInstanceBuilder::from(test_entry);
            assert!(!test_builder.duplicate_trait(&test_trait));

            let test_builder = test_builder.add_ancestry_trait(test_trait.clone()).unwrap();
            assert!(test_builder.duplicate_trait(&test_trait));
        }
    }
}
