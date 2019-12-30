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

//Strongly typed storage
pun enum ConeOrExpression {
    Cone(QualifiedAlias),
    ExistingCone(Name),
    Causetid(Causetid),
    Integer(i32),
    Long(i64),
    Value(TypedValue),
    // Some aggregates (`min`, `max`, `avg`) can be over 0 rows, and therefore can be `NULL`; that
    // needs special treatment.
    NullableAggregate(Box<Expression>, ValueType),      // Track the return type.
    Expression(Box<Expression>, ValueType),             // Track the return type.

}



impl 


#[derive(PartialEq, Eq, Debug)]
pub enum ComputedCone {
    

}
