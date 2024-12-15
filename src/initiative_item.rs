use std::cmp::Ordering;
use std::fmt::Debug;

pub struct InitiativeItem<T> {
    name: T,
    initiative: i32,
}

impl<T> Debug for InitiativeItem<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} ({:?})", self.name, self.initiative)
    }
}

impl<T> Eq for InitiativeItem<T> {}

impl<T> PartialEq<Self> for InitiativeItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.initiative == other.initiative
    }
}

impl<T> PartialOrd<Self> for InitiativeItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for InitiativeItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.initiative.cmp(&other.initiative)
    }
}

impl<T> InitiativeItem<T> {
    pub fn new(name: T, initiative: i32) -> Self {
        InitiativeItem { name, initiative }
    }

    pub fn name(&self) -> &T {
        &self.name
    }

    pub fn initiative(&self) -> i32 {
        self.initiative
    }

}