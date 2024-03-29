//Copyright 2020 WHTCORPS INC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.


/// Cache traits.

use std::collections::{
    BTreeSet,
};

pub trait CachedCausets {

    fn is_causet_cached_spacelike(&self, solitonid: Solitonid) -> bool;
    fn is_causet_cached_timelike(&self, solitonid: Solitonid) -> bool;
    fn has_cached_causets(&self) -> bool;

    fn get_values_for_

}