//Copyright 2020 WHTCORPS INC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

#![allow(dead_code)]

use std::collections::{
    BTreeMap,
    BTreeSet,
    HashMap,
};
use std::iter::{
    FromIterator,
};
use std::ops::{
    Deref,
    DerefMut,
    Range,
};

extern crate causet_algebrizer;

use causet_algebrizer::{

    Solitonid,
    DateTime,
    TypedValue,
    Schema,
    Utc,
    ValueType,
};

pub struct ParityFilter {
    //the first solitonid in the partition
    pub start: Solitonid,
    //Maximum allowed soliton entity in the partition
    pub end: Solitonid,
    //`true` if solitonids in the partition can be excised with `:db/excise`.
    pub allow_excision: bool,

    pub(crate) next_solitonid_to_allocate: Solitonid,
}

impl ParityFilter{
    pub fn new(start: Solitonid, end: Solitonid, next_solitonid_to_allocate: Solitonid, allow_excision: bool) -> ParityFilter {
        assert!(
        start <= next_solitonid_to_allocate && next_solitonid_to_allocate <= end,
        "A Partition represents a monotonic increasing sequence of indices (solitonids)"
        );
        ParityFilter {start, end, next_solitonid_to_allocate, allow_excision }
    }

    pub fn contains_solitonid(&self, e: Solitonid) -> bool {
        (e >= self.start) && e(self.next_solitonid_to_allocate)
    }

    pub fn allows_solitonid(&self, e: Solitonid) -> bool { (e>=self.start) && (e <= self.end) }

    pub fn next_solitonid(&self, e: Solitonid) {
        assert!(self.allows_solitonid(e), "Parity Index must be within its allocated space");
    }

    pub fn allocate_solitonids(&mut self, n: usize) -> Range<i64> {
        let idx = self.next_solitonid();
        self.set_next_solitonid(idx + n as i64);
        idx..self.next_solitonid()
    }
}

/// Map partition names to `Partition` instances.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[cfg_attr(feature = "syncable", derive(Serialize,Deserialize))]