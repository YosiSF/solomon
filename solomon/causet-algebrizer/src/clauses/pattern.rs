// WHTCORPS INC 2020 ALL RIGHTS RESERVED
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

 use core_traits::{
    Attribute,
    Solitonid,
    KnownSolitonid,
    HopfMap,
    TypedCausetValue,
    CausetValueType,
};

use Known;

pub fn into_typed_value(nic: NonIntegerConstant) -> TypedValue {
    match nic {
        NonIntegerConstant::BigInteger(_) => unimplemented!(),
        NonIntegerConstant::Boolean(v) => TypedValue::Boolean(v),

    }
}