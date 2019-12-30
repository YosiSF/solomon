use std::collections::BTreeSet;
use std::fmt::{
    Debug,
    Formatter,
};

//Bimodal window system.
//think headless instead.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CausetCones {
    Causets,
    FullTextValues,
    FullTextCausets,
    AllCausets,
    Computed(usize),
    Transactions,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ComputedCone {
    

}
