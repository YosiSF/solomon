//Copyright 2020 WHTCORPS INC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.



//! This module defines core types that support the transaction processor.

use std::collections::BTreeMap;
use std::fmt;

use value_rc::{
    ValueRc,
};

use symbols::{
    Keyword,
    PlainSymbol,
};

use types::{
    ValueAndSpan,
};

//In order to maintain the graph of `Into` and
///// `From` relations; mark, remove, or admit
pub trait TransmutableValueMarker {}

/// `ValueAndSpan` is the value type coming out of the entity parser.
impl TransmutableValueMarker for ValueAndSpan {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum TempId {
    External(String),
    Internal(i64),
}

impl TempId {
    pub fn into_external(self) -> Option<String> {
        match self {
            TempId::External(s) => Some(s),
            TempId::Internal(_) => None,
        }
    }
}

//handle the prompt
impl fmt::Display for TempId {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &TempId::External(ref s) => write!(f, "{}", s),
            &TempId::Internal(x) => write!(f, "<tempid {}>", x),
        }
    }
}




#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct LookupRef<V> {
    pub a: LocalAttribute,
}

//late binding transaction


#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum LocalValue<V> {
    // We never know at parse-time whether an integer or solitonid is a Solitonid or not, but
    //we will often become knowledgeable of such while building solitons programmatically.
    Solitonid(SolitonidOrSolitonid),
    TempId(ValueRc<TempId>),
    LookupRef(LookupRef<V>),
    TxFunction(TxFunction),
    Vector(Vec<LocalValue<V>>),
    Atom(V),
    MapNotation(MapNotation<V>),

}

//late binding transaction