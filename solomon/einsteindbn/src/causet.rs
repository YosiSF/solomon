use std::collections::{
    BTreeSet,
    HashSet,
};

use std;
use std::fmt;
use std::rc::{
    Rc,
};

use ::{
    BigInt,
    DateTime,
    OrderedFloat,
    Uuid,
    Utc,
};

use ::value_rc::{
    FromRc,
    ValueRc,
};

pub use ::{
    Keyword,
    PlainSymbol,
};

pub type SrcVarName = String;          // Do not include the required syntactic '$'.

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Variable(pub Rc<PlainSymbol>);

impl Variable {
    pub fn as_str(&self) -> &str {
        self.0.as_ref().0.as_str()
    }

    pub fn to_string(&self) -> String {
        self.0.as_ref().0.clone()
    }

    pub fn name(&self) -> PlainSymbol {
        self.0.as_ref().clone()
    }

    /// Return a new `Variable`, assuming that the provided string is a valid name.
    pub fn from_valid_name(name: &str) -> Variable {
        let s = PlainSymbol::plain(name);
        assert!(s.is_var_symbol());
        Variable(Rc::new(s))
    }
}

pub trait FromValue<T> {
    fn from_value(v: &::ValueAndSpan) -> Option<T>;
}


impl FromValue<Variable> for Variable {
    fn from_value(v: &::ValueAndSpan) -> Option<Variable> {
        if let ::SpannedValue::PlainSymbol(ref s) = v.inner {
            Variable::from_symbol(s)
        } else {
            None
        }
    }
}

impl Variable {
    pub fn from_rc(sym: Rc<PlainSymbol>) -> Option<Variable> {
        if sym.is_var_symbol() {
            Some(Variable(sym.clone()))
        } else {
            None
        }
    }


    pub fn from_symbol(sym: &PlainSymbol) -> Option<Variable> {
        if sym.is_var_symbol() {
            Some(Variable(Rc::new(sym.clone())))
        } else {
            None
        }
    }
}



impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var({})", self.0)
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//scalar value representations in einsteindbn
pub enum NonIntegerConstant {
    Boolean(bool),
    BigInteger(BigInt),
    Float(OrderedFloat<f64>),
    Text(ValueRc<String>),
    Instant(DateTime<Utc>),
    Uuid(Uuid),
}

impl<'a> From<&'a str> for NonIntegerConstant {
    fn from(val: &'a str) -> NonIntegerConstant {
        NonIntegerConstant::Text(ValueRc::new(val.to_string()))
    }
}

impl From<String> for NonIntegerConstant {
    fn from(val: String) -> NonIntegerConstant {
        NonIntegerConstant::Text(ValueRc::new(val))
    }
}
