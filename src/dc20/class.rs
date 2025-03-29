use uuid::Uuid;

use super::Level;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassTableColumn(pub [usize; 10]);

impl ClassTableColumn {
    pub fn get(&self, level: usize) -> usize {
        self.0[level - 1]
    }
}

impl From<[usize; 10]> for ClassTableColumn {
    fn from(value: [usize; 10]) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CombatStyle {
    Martial {
        maneuvers: Vec<Maneuver>,
        stamina_points: Box<ClassTableColumn>,
        maneuvers_known: Box<ClassTableColumn>,
        techniques_known: Box<ClassTableColumn>,
    },
    Spellcasting {
        spell_list: Vec<Spell>,
        mana_points: Box<ClassTableColumn>,
        cantrips_known: Box<ClassTableColumn>,
        spells_known: Box<ClassTableColumn>,
    },
}

impl CombatStyle {
    pub fn default_martial() -> Self {
        Self::Martial {
            maneuvers: Default::default(),
            stamina_points: Box::new([1, 0, 1, 0, 0, 1, 0, 0, 1, 0].into()),
            maneuvers_known: Box::new([4, 0, 0, 0, 1, 0, 0, 1, 0, 0].into()),
            techniques_known: Box::new([0, 0, 1, 0, 1, 0, 0, 1, 0, 0].into()),
        }
    }

    pub fn default_spellcasting() -> Self {
        Self::Spellcasting {
            spell_list: Default::default(),
            mana_points: Box::new([6, 0, 2, 0, 2, 2, 0, 2, 2, 0].into()),
            cantrips_known: Box::new([2, 0, 0, 0, 1, 0, 0, 1, 0, 0].into()),
            spells_known: Box::new([3, 0, 1, 0, 0, 1, 0, 0, 1, 0].into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpellFamily {
    Conjuration,
    Divination,
    Enchantment,
    Illusion,
    Restoration,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Spell {
    pub uuid: Uuid,
    pub name: String,
    pub family: SpellFamily,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Maneuver {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Technique {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClassFeature {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Talent {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Item {
    pub uuid: Uuid,
    pub name: String,
}

/// This represents the Class as it is in the
/// book. The user will use this entry to
/// select the Class-related traits for their
/// Character.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClassEntry {
    pub uuid: Uuid,
    pub name: String,
    pub combat_style: Vec<CombatStyle>,
    pub available_subclasses: Vec<SubclassEntry>,
    pub starting_equipment: Vec<Item>,
    pub features: Vec<(Level, ClassFeature)>,
}

impl ClassEntry {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubclassEntry {
    pub uuid: Uuid,
    pub name: String,
    pub features: Vec<ClassFeature>,
    pub flavor_feature: Option<ClassFeature>,
}

impl SubclassEntry {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod class {
        use super::*;

        #[test]
        fn _new_creates_new_uuid_and_sets_name() {
            let test_class = ClassEntry::new("Test");

            assert_eq!(test_class.name, "Test".to_string());

            assert_ne!(test_class.uuid, Uuid::nil());
        }
    }

    mod subclass {
        use super::*;

        #[test]
        fn _new_creates_new_uuid_and_sets_name() {
            let test_subclass = SubclassEntry::new("Test");

            assert_eq!(test_subclass.name, "Test".to_string());

            assert_ne!(test_subclass.uuid, Uuid::nil());
        }
    }
}
