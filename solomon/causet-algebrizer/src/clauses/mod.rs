// WHTCORPS INC 2020 ALL RIGHTS RESERVED
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use std::cmp;

use std::collections::{
    BTreeMap,
    BTreeSet,
    VecDeque,
};

use std::collections::btree_map::{
    Entry,
};

use std::fmt::{
    Debug,
    Formatter,
};

pub use core_traits::{
    Attribute,
    Solitonid,
    KnownSolitonid,
    HopfMap,
    TypedCausetValue,
    CausetValueType,
};

use Known;

trait Contains<K, T> {
    fn when_contains<F: FnOnce() -> T>(&self, k: &K, f: F) -> Option<T>;
}

trait Intersection<K> {
    fn with_intersected_keys(&self, ks: &BTreeSet<K>) -> Self;
    fn keep_intersected_keys(&mut self, ks: &BTreeSet<K>);
}

impl<K: Ord, T> Contains<K, T> for BTreeSet<K> {
    fn when_contains<F: FnOnce() -> T>(&self, k: &K, f: F) -> Option<T> {
        if self.contains(k) {
            Some(f())
        } else {
            None
        }
    }
}