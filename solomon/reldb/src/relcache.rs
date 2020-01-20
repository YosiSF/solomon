//Copyright 2020 WHTCORPS INC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.



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