//Copyright 2020 WHTCORPS INC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.


//Petri net assertions and retractions based on Petri's relativistic mesh: all assertions and retractions,
//including folding(assertion, extraction) lies on a relativistic cone.

use std::collections::BTreeMap;

//We are thus able to causally consistently asses for veracity the truthines of the :reldb/add and :reldb/retract,
//separately or conjoined; preserving some sense of temporal order based on the attribute cache and the relcache.

pub struct AddRelativisticAlterSet<K, V> {
    pub asserted: BTreeMap<K, V>,
    pub retracted: BTreeMap<K,V>,
    pub altered: BTreeMap<K, (V,V)>,

}

impl<K,V>Default for AddRelativisticAlterSet<K, V> where K: Ord {
    fn default() -> AddRelativisticAlterSet<K,V> {
        AddRelativisticAlterSet {
            asserted: BTreeMap::default(),
            retracted: BTreeMap::default(),
            altered: BTreeMap::default(),
        }
    }

}