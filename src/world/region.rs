use crate::world::RegionSection;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionId(pub u32);

#[derive(Debug, Clone)]
pub struct RegionSlot {
    pub id: RegionId,
    pub sections: Vec<RegionSection>,
}

#[derive(Debug, Default, Clone)]
pub struct RegionDirectory {
    slots: Vec<RegionSlot>,
    section_owner: HashMap<u64, RegionId>,
}

impl RegionDirectory {
    pub fn insert_region(&mut self, sections: Vec<RegionSection>) -> RegionId {
        let id = RegionId(self.slots.len() as u32);
        for section in &sections {
            self.section_owner.insert(section.packed(), id);
        }
        self.slots.push(RegionSlot { id, sections });
        id
    }

    pub fn owner_of(&self, section: RegionSection) -> Option<RegionId> {
        self.section_owner.get(&section.packed()).copied()
    }

    pub fn section_count(&self) -> usize {
        self.section_owner.len()
    }

    pub fn region_count(&self) -> usize {
        self.slots.len()
    }
}

#[cfg(test)]
mod tests {
    use super::RegionDirectory;
    use crate::world::RegionSection;

    #[test]
    fn owns_sparse_sections() {
        let mut directory = RegionDirectory::default();
        let id = directory.insert_region(vec![RegionSection::new(-1, 0), RegionSection::new(0, 0)]);
        assert_eq!(directory.owner_of(RegionSection::new(-1, 0)), Some(id));
        assert_eq!(directory.owner_of(RegionSection::new(1, 0)), None);
        assert_eq!(directory.section_count(), 2);
        assert_eq!(directory.region_count(), 1);
    }
}
