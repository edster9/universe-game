//! Tables that remember what they were. While the gate applies a set of
//! changes, every table notes the old value of each entry it changes, so a
//! refused set can be undone without copying the world, and the gate can
//! check just what changed. Reading a table is reading a sorted map.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Deref;

/// A sorted map that can record the old value of every entry it changes.
#[derive(Clone, Debug)]
pub struct Table<K: Ord + Clone, V: Clone> {
    map: BTreeMap<K, V>,
    /// While recording: each changed entry's value before its first change.
    journal: Option<Vec<(K, Option<V>)>>,
    /// The entries already in the journal.
    noted: BTreeSet<K>,
}

impl<K: Ord + Clone, V: Clone> Default for Table<K, V> {
    fn default() -> Self {
        Table {
            map: BTreeMap::new(),
            journal: None,
            noted: BTreeSet::new(),
        }
    }
}

/// Two tables are equal if they hold the same entries, whatever they're
/// recording.
impl<K: Ord + Clone, V: Clone + PartialEq> PartialEq for Table<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<K: Ord + Clone, V: Clone + Eq> Eq for Table<K, V> {}

impl<K: Ord + Clone, V: Clone> Deref for Table<K, V> {
    type Target = BTreeMap<K, V>;
    fn deref(&self) -> &BTreeMap<K, V> {
        &self.map
    }
}

impl<K: Ord + Clone, V: Clone> FromIterator<(K, V)> for Table<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Table {
            map: iter.into_iter().collect(),
            journal: None,
            noted: BTreeSet::new(),
        }
    }
}

impl<K: Ord + Clone, V: Clone> Table<K, V> {
    /// Notes what `key` holds, the first time it changes while recording.
    fn note(&mut self, key: &K) {
        if let Some(journal) = &mut self.journal
            && self.noted.insert(key.clone())
        {
            journal.push((key.clone(), self.map.get(key).cloned()));
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.note(&key);
        self.map.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.map.contains_key(key) {
            self.note(key);
        }
        self.map.remove(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            self.note(key);
        }
        self.map.get_mut(key)
    }

    /// The entry for `key`, made with `make` if it isn't there.
    pub fn or_insert_with(&mut self, key: K, make: impl FnOnce() -> V) -> &mut V {
        self.note(&key);
        self.map.entry(key).or_insert_with(make)
    }

    /// Every entry, for changing in place. Everything is noted.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        let keys: Vec<K> = self.map.keys().cloned().collect();
        for key in &keys {
            self.note(key);
        }
        self.map.values_mut()
    }

    pub fn retain(&mut self, mut keep: impl FnMut(&K, &mut V) -> bool) {
        let gone: Vec<K> = self
            .map
            .iter_mut()
            .filter_map(|(k, v)| (!keep(k, v)).then(|| k.clone()))
            .collect();
        for key in gone {
            self.remove(&key);
        }
    }

    /// Each entry changed since recording began, with what it held then
    /// (`None` if it wasn't there).
    pub fn originals(&self) -> BTreeMap<&K, Option<&V>> {
        self.journal
            .iter()
            .flatten()
            .map(|(key, old)| (key, old.as_ref()))
            .collect()
    }
}

impl<K: Ord + Clone, V: Clone> Journal for Table<K, V> {
    fn begin(&mut self) {
        self.journal = Some(Vec::new());
        self.noted.clear();
    }

    fn commit(&mut self) {
        self.journal = None;
        self.noted.clear();
    }

    fn undo(&mut self) {
        self.noted.clear();
        for (key, old) in self.journal.take().into_iter().flatten().rev() {
            match old {
                Some(value) => self.map.insert(key, value),
                None => self.map.remove(&key),
            };
        }
    }
}

/// A sorted set that can record what it held, like a [`Table`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Set<K: Ord + Clone>(Table<K, ()>);

impl<K: Ord + Clone> Default for Set<K> {
    fn default() -> Self {
        Set(Table::default())
    }
}

impl<K: Ord + Clone> Set<K> {
    pub fn insert(&mut self, key: K) -> bool {
        self.0.insert(key, ()).is_none()
    }

    pub fn remove(&mut self, key: &K) -> bool {
        self.0.remove(key).is_some()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.0.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.0.keys()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Each key added or removed since recording began, and whether it was
    /// there then.
    pub fn originals(&self) -> BTreeMap<&K, bool> {
        self.0
            .originals()
            .into_iter()
            .map(|(k, v)| (k, v.is_some()))
            .collect()
    }
}

impl<K: Ord + Clone> FromIterator<K> for Set<K> {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        Set(iter.into_iter().map(|k| (k, ())).collect())
    }
}

impl<K: Ord + Clone> Journal for Set<K> {
    fn begin(&mut self) {
        self.0.begin();
    }
    fn commit(&mut self) {
        self.0.commit();
    }
    fn undo(&mut self) {
        self.0.undo();
    }
}

/// Something that can record its changes, keep them, or undo them.
pub trait Journal {
    fn begin(&mut self);
    fn commit(&mut self);
    fn undo(&mut self);
}

impl<'a, K: Ord + Clone, V: Clone> IntoIterator for &'a Table<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::collections::btree_map::Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<'a, K: Ord + Clone> IntoIterator for &'a Set<K> {
    type Item = &'a K;
    type IntoIter = std::collections::btree_map::Keys<'a, K, ()>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.map.keys()
    }
}
