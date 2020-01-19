

// Right now we use BTreeMap, because we expect few cached attributes.
pub type CacheMap<K, V> = BTreeMap<K, V>;

trait Remove<T> where T: PartialEq {
    fn remove_every(&mut self, item: &T) -> usize;
}

impl<T> Remove<T> for Vec<T> where T: PartialEq {
    /// Remove all occurrences from a vector in-place, by equality.
    fn remove_every(&mut self, item: &T) -> usize {
        let initial_len = self.len();
        self.retain(|v| v != item);
        initial_len - self.len()
    }
}

trait Absorb {
    fn absorb(&mut self, other: Self);
}

impl<K, V> Absorb for CacheMap<K, Option<V>> where K: Ord {
    fn absorb(&mut self, other: Self) {
        for (e, v) in other.into_iter() {
            match v {
                None => {
                    // It was deleted. Remove it from our map.
                    self.remove(&e);
                },
                s @ Some(_) => {
                    self.insert(e, s);
                },
            }
        }
    }
}