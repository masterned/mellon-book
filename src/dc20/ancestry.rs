use uuid::Uuid;

/// This is used to know whether the user
/// selected to start with 1 or 2 Ancestries
/// when creating their Character. RAW,
/// Characters can only pull from 2 Ancestries
/// unless they use a CustomOrigin.
///
/// I'm thinking that I may need an
/// OriginBuilder as well. I need to make sure
/// that HybridBreds and CustomOrigins don't
/// get a weird AncestryTrait advantage over
/// PureBreds due to logic errors.
#[derive(Clone, Debug, PartialEq)]
pub enum Origin {
    PureBred(AncestryInstance),
    HybridBred(AncestryInstance, AncestryInstance),
    CustomOrigin(Vec<AncestryInstance>),
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
    pub available_points: isize,
}

impl AncestryInstanceBuilder {
    pub fn add_ancetry_trait(
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
        if let Some(selected_traits) = self.selected_traits.clone() {
            selected_traits.iter().map(|t: &AncestryTrait| t.cost).sum()
        } else {
            0
        }
    }

    fn trait_points_exceeded(&self) -> bool {
        self.total_trait_points() > self.available_points
    }

    fn trait_points_remaining(&self) -> bool {
        self.total_trait_points() < self.available_points
    }

    fn trait_available(&self, ancestry_trait: &AncestryTrait) -> bool {
        self.entry.available_traits.contains(&ancestry_trait)
    }

    fn duplicate_trait(&self, ancestry_trait: &AncestryTrait) -> bool {
        self.selected_traits
            .as_ref()
            .is_some_and(|t| t.contains(&ancestry_trait))
    }

    fn has_single_0_point_trait(&self) -> bool {
        self.selected_traits
            .as_ref()
            .is_some_and(|ts| ts.iter().filter(|t| t.cost == 0).count() == 1)
    }

    pub fn build(self) -> Result<AncestryInstance, &'static str> {
        self.try_into()
    }
}

impl From<AncestryEntry> for AncestryInstanceBuilder {
    fn from(value: AncestryEntry) -> Self {
        AncestryInstanceBuilder {
            entry: value,
            selected_traits: None,
            available_points: 5,
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

impl TryFrom<AncestryInstanceBuilder> for AncestryInstance {
    type Error = &'static str;

    fn try_from(value: AncestryInstanceBuilder) -> Result<Self, Self::Error> {
        if let Some(selected_traits) = value.selected_traits.clone() {
            if value.trait_points_remaining() {
                return Err("All Trait points must be used");
            }
            if value.trait_points_exceeded() {
                return Err("Too many Trait points used");
            }
            if !value.has_single_0_point_trait() {
                return Err("Requires one and only on 0-point Trait");
            }

            Ok(AncestryInstance {
                uuid: Uuid::new_v4(),
                name: value.entry.name,
                traits: selected_traits,
            })
        } else {
            return Err("Ancetry requires Traits");
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

        let test_builder = test_builder.add_ancetry_trait(test_trait.clone()).unwrap();
        assert!(test_builder.duplicate_trait(&test_trait));
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

        let test_builder = AncestryInstanceBuilder::from(test_entry);

        assert!(!test_builder.trait_points_exceeded());

        let test_builder = test_builder.add_ancetry_trait(test_trait).unwrap();

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

        let test_builder = AncestryInstanceBuilder::from(test_entry);

        assert!(test_builder.trait_points_remaining());

        let test_builder = test_builder.add_ancetry_trait(test_trait).unwrap();

        assert!(!test_builder.trait_points_remaining());
    }

    /// This seriously needs to apply to creating
    /// an Origin rather than a single Instance.
    /// The requirement for a single 0-point
    /// Trait applies to multi-ancestry
    /// characters as well as purebreds.
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

        let test_builder = AncestryInstanceBuilder::from(test_entry);
        assert!(!test_builder.has_single_0_point_trait());

        let test_builder = test_builder.add_ancetry_trait(test_trait_0).unwrap();
        assert!(test_builder.has_single_0_point_trait());

        let test_builder = test_builder.add_ancetry_trait(test_trait_1).unwrap();
        assert!(!test_builder.has_single_0_point_trait());
    }
}
