use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

/// Wrapper for `HashMap` with additional serialization features:
///  * keys are sorted alphabetically
///  * if value vec have only one element, it will be serialized as an element, not a `vec![element]`
#[derive(Debug, Clone)]
pub struct PickyHashMap<V>(HashMap<String, V>);

impl<V> PickyHashMap<V> {
    pub fn new() -> PickyHashMap<V> {
        PickyHashMap(
            HashMap::new()
        )
    }

    pub fn entry(&mut self, key: String) -> Entry<String, V> {
        self.0.entry(key)
    }

}

impl<V> FromIterator<(String, V)> for PickyHashMap<V> {
    fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> PickyHashMap<V> {
        let mut map = HashMap::with_hasher(Default::default());
        map.extend(iter);

        PickyHashMap(map)
    }
}


impl Serialize for PickyHashMap<Vec<String>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut entries = self.0.iter().collect::<Vec<_>>();
        entries.sort_unstable_by(|(key1, _), (key2, _)| key1.cmp(key2));

        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in entries {
            if value.len() == 1 {
                map.serialize_entry(key, &value[0])?;
            } else {
                map.serialize_entry(key, value)?;
            }
        }
        map.end()
    }
}

impl Serialize for PickyHashMap<String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut entries = self.0.iter().collect::<Vec<_>>();
        entries.sort_unstable_by(|(key1, _), (key2, _)| key1.cmp(key2));

        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in entries {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
