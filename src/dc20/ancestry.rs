use core::fmt;
use std::error::Error;

use uuid::Uuid;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OriginBuilder {
    pub available_points: isize,
    pub ancestries: Option<Vec<AncestryInstanceBuilder>>,
}

impl OriginBuilder {
    pub fn new() -> Self {
        Self {
            available_points: 5,
            ..Self::default()
        }
    }

    pub fn with_ancestry(mut self, entry: AncestryEntry) -> Result<Self, &'static str> {
        if self.is_duplicate_entry(&entry) {
            return Err("Adding duplicate Entry");
        }

        self.ancestries
            .get_or_insert_default()
            .push(AncestryInstanceBuilder::from(entry));

        Ok(self)
    }

    /// What to do when multiple Entries have the
    /// same Trait available? Add to first one?
    /// Could prevent user from optimizing
    /// Origin. Force user to specify which
    /// Entry? Would be a pain to have to
    /// specify every time. Attempt and Error?
    /// Would waste more calls...
    pub fn add_ancestry_trait(
        mut self,
        ancestry_trait: AncestryTrait,
    ) -> Result<Self, &'static str> {
        if let Some(ref mut ancestries) = self.ancestries {
            for ancestry in ancestries.iter_mut() {
                if let Ok(modified) = ancestry.clone().add_ancestry_trait(ancestry_trait.clone()) {
                    *ancestry = modified;
                    return Ok(self);
                }
            }
            Err("Cannot add Trait to any available Ancestries")
        } else {
            Err("Must have Ancestry to add Trait")
        }
    }

    fn is_duplicate_entry(&self, entry: &AncestryEntry) -> bool {
        self.ancestries
            .as_ref()
            .is_some_and(|es| es.iter().filter(|a| a.entry == *entry).count() >= 1)
    }

    fn total_trait_points(&self) -> isize {
        self.ancestries.as_ref().map_or(0, |a_s| {
            a_s.iter()
                .map(AncestryInstanceBuilder::total_trait_points)
                .sum()
        })
    }

    fn trait_points_exceeded(&self) -> bool {
        self.total_trait_points() > self.available_points
    }

    fn trait_points_remaining(&self) -> bool {
        self.total_trait_points() < self.available_points
    }

    fn has_single_0_point_trait(&self) -> bool {
        self.ancestries.as_ref().map_or(0, |a_s| {
            a_s.iter()
                .fold(0, |acc, a| acc + a.zero_point_traits().len())
        }) == 1
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
    AncestryError(InstanceBuildError),
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
                OriginBuildError::ZeroPointTraitMissing =>
                    "Requires one and only one 0-point Trait".into(),
                OriginBuildError::AncestryError(e) => format!("\n\t{e}"),
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
        if !value.has_single_0_point_trait() {
            return Err(OriginBuildError::ZeroPointTraitMissing);
        }

        let instances: Vec<_> = value
            .ancestries
            .unwrap()
            .iter()
            .map(|b| b.clone().build().unwrap())
            .collect();
        dbg!(&instances);

        if instances.len() > 2 {
            return Ok(Origin::CustomOrigin(instances));
        }
        if instances.len() == 2 {
            return Ok(Origin::HybridBred(
                instances[0].clone(),
                instances[1].clone(),
            ));
        }

        Ok(Origin::PureBred(instances[0].clone()))
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
pub struct AncestryInstanceBuilder {
    pub entry: AncestryEntry,
    pub selected_traits: Option<Vec<AncestryTrait>>,
}

impl AncestryInstanceBuilder {
    pub fn add_ancestry_trait(
        mut self,
        ancestry_trait: AncestryTrait,
    ) -> Result<Self, &'static str> {
        if !self.trait_available(&ancestry_trait) {
            return Err("Trait not available for the given Ancestry");
        }

        if self.duplicate_trait(&ancestry_trait) {
            return Err("Attempting to add duplicate Trait");
        }

        self.selected_traits
            .get_or_insert_default()
            .push(ancestry_trait);

        Ok(self)
    }

    fn total_trait_points(&self) -> isize {
        self.selected_traits
            .as_ref()
            .map_or(0, |ts| ts.iter().map(|t| t.cost).sum())
    }

    fn trait_available(&self, ancestry_trait: &AncestryTrait) -> bool {
        self.entry.available_traits.contains(&ancestry_trait)
    }

    fn duplicate_trait(&self, ancestry_trait: &AncestryTrait) -> bool {
        self.selected_traits
            .as_ref()
            .is_some_and(|t| t.contains(&ancestry_trait))
    }

    fn zero_point_traits(&self) -> Vec<&AncestryTrait> {
        self.selected_traits
            .as_ref()
            .map(|ts| ts.iter().filter(|t| t.cost == 0).collect())
            .unwrap_or(vec![])
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
#[derive(Clone, Debug, PartialEq)]
pub struct AncestryInstance {
    pub uuid: Uuid,
    pub name: String,
    pub traits: Vec<AncestryTrait>,
}

impl AncestryInstance {
    /// I really need to remove this method so
    /// that I force myself to use the Builder.
    pub fn new(name: impl Into<String>) -> Self {
        AncestryInstance {
            uuid: Uuid::new_v4(),
            name: name.into(),
            traits: vec![],
        }
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
        fn _cannot_have_multiple_ancestries_of_same_entry() {
            let test_entry = AncestryEntry::default();

            let test_builder = OriginBuilder::new();
            assert!(!test_builder.is_duplicate_entry(&test_entry));

            let test_builder = OriginBuilder::new()
                .with_ancestry(test_entry.clone())
                .unwrap();
            assert!(test_builder.is_duplicate_entry(&test_entry));

            assert_eq!(
                OriginBuilder::new()
                    .with_ancestry(test_entry.clone())
                    .unwrap()
                    .with_ancestry(test_entry),
                Err("Adding duplicate Entry")
            )
        }

        #[test]
        fn _cannot_exceed_available_points() {
            let test_trait = AncestryTrait {
                cost: 6,
                ..AncestryTrait::default()
            };
            let test_entry = AncestryEntry {
                available_traits: vec![test_trait.clone()],
                ..AncestryEntry::default()
            };

            let test_builder = OriginBuilder::new().with_ancestry(test_entry).unwrap();

            assert!(!test_builder.trait_points_exceeded());

            let test_builder = test_builder.add_ancestry_trait(test_trait).unwrap();

            assert!(test_builder.trait_points_exceeded());
        }

        #[test]
        fn _must_use_all_available_points() {
            let test_trait = AncestryTrait {
                cost: 5,
                ..AncestryTrait::default()
            };
            let test_entry = AncestryEntry {
                available_traits: vec![test_trait.clone()],
                ..AncestryEntry::default()
            };

            let test_builder = OriginBuilder::new().with_ancestry(test_entry).unwrap();

            assert!(test_builder.trait_points_remaining());

            let test_builder = test_builder.add_ancestry_trait(test_trait).unwrap();

            assert!(!test_builder.trait_points_remaining());
        }

        #[test]
        fn _must_contain_one_and_only_one_zero_point_trait() {
            let test_trait_0 = AncestryTrait::default();
            let test_trait_1 = AncestryTrait {
                uuid: Uuid::new_v4(),
                ..Default::default()
            };
            let test_entry = AncestryEntry {
                available_traits: vec![test_trait_0.clone(), test_trait_1.clone()],
                ..Default::default()
            };

            let test_builder = OriginBuilder::new().with_ancestry(test_entry).unwrap();
            assert!(!test_builder.has_single_0_point_trait());

            let test_builder = test_builder.add_ancestry_trait(test_trait_0).unwrap();
            assert!(test_builder.has_single_0_point_trait());

            let test_builder = test_builder.add_ancestry_trait(test_trait_1).unwrap();
            assert!(!test_builder.has_single_0_point_trait());
        }

        #[test]
        #[ignore = "not yet implemented"]
        fn _builds_correct_origin_based_on_ancestry_count() {
            todo!()
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
