extern crate chrono;
extern crate itertools;
extern crate num;
extern crate ordered_float;
extern crate pretty;
extern crate uuid;

#[cfg(feature = "serde_support")]
extern crate serde;

#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde_derive;

pub mod solitons;
pub mod intern_set;
pub use intern_set::{
    InternSet,
};
// Intentionally not pub.
mod namespaceable_name;
pub mod query;
pub mod symbols;
pub mod types;
pub mod pretty_print;
pub mod utils;
pub mod matcher;
pub mod value_rc;
pub use value_rc::{
    Cloned,
    FromRc,
    ValueRc,
};

pub mod parse {
    include!(concat!(env!("OUT_DIR"), "/edn.rs"));
}

// Re-export the types we use.
pub use chrono::{DateTime, Utc};
pub use num::BigInt;
pub use ordered_float::OrderedFloat;
pub use uuid::Uuid;

// Export from our modules.
pub use parse::ParseError;
pub use uuid::ParseError as UuidParseError;
pub use types::{
    FromMicros,
    FromMillis,
    Span,
    SpannedValue,
    ToMicros,
    ToMillis,
    Value,
    ValueAndSpan,
};

pub use symbols::{
    Keyword,
    NamespacedSymbol,
    PlainSymbol,
};