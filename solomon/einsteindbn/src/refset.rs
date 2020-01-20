use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{
    Deref,
    DerefMut,
};

use::{
    //ValueRc,
};

//Compactified groupoid action on the octal header leads to an inner or internal set.
//A set of potentially large values, maintaining a single writer instance owned by the
//RefSet, which leaves all readers with slim ref-counted address handles and addresses to avoid
//expensive clone().

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RefSet<T> where T: Eq + Hash {
    inner:HashSet<ValueRc<T>>,
}

impl<T> Deref for RefSet<T> where T: Eq + Hash {
    type target = HashSet<ValueRc<T>>;

    fn deref(&self) -> &Self::Target{ &self.inner }
}

impl<T>DeRefMut for RefSet<T> where T: Eq + Hash {
    fn deref_mut(&mut self) -> &mut Self::target{ &mut self.inner}
}

/// Intern a value, providing a ref-counted handle to the interned value.
  ///
  /// ```
  /// use einsteindbn::{RefSet, ValueRc};
  ///
  /// let mut s = RefSet::new();
  ///
  /// let one = "foo".to_string();
  /// let two = ValueRc::new("foo".to_string());
  ///

