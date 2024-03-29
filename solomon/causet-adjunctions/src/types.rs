//Copyright 2020 WHTCORPS INC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use std::collections::BTreeSet;
use std::fmt::{
    Debug,
    Formatter,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CausetsTable {

    Causets,            //Non-fulltext causets table.
    FulltextValues,     //virtual table mapping Ids to strings.
    FullTextCausets,    //The fulltext-causets view
    AllCausets,         //Fulltext and non-fulltext causets.
    Computed(usize),    //A computed table, tracked elsewhere in the query
    Transactions,      //The transactions table, tracked elsewhere in the query.

}



