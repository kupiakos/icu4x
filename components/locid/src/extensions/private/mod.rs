// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Private Use Extensions is a list of extensions intended for
//! private use.
//!
//! Those extensions are treated as a pass-through, and no Unicode related
//! behavior depends on them.
//!
//! The main struct for this extension is [`Private`] which is a list of [`Keys`].
//!
//! # Examples
//!
//! ```
//! use icu::locid::extensions::private::{Key, Private};
//! use icu::locid::Locale;
//!
//! let mut loc: Locale = "en-US-x-foo-faa".parse().expect("Parsing failed.");
//!
//! let key: Key = "foo".parse().expect("Parsing key failed.");
//! assert_eq!(loc.extensions.private.contains(&key), true);
//! assert_eq!(loc.extensions.private.iter().next(), Some(&key));
//! loc.extensions.private.clear();
//! assert_eq!(loc.to_string(), "en-US");
//! ```
//!
//! [`Keys`]: Key

mod key;

use alloc::vec::Vec;
use core::ops::Deref;

pub use key::Key;

use crate::parser::ParserError;
use crate::parser::SubtagIterator;

/// A list of [`Private Use Extensions`] as defined in [`Unicode Locale
/// Identifier`] specification.
///
/// Those extensions are treated as a pass-through, and no Unicode related
/// behavior depends on them.
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::private::{Key, Private};
///
/// let key1: Key = "foo".parse().expect("Failed to parse a Key.");
/// let key2: Key = "bar".parse().expect("Failed to parse a Key.");
///
/// let private = Private::from_vec_unchecked(vec![key1, key2]);
/// assert_eq!(&private.to_string(), "-x-foo-bar");
/// ```
///
/// [`Private Use Extensions`]: https://unicode.org/reports/tr35/#pu_extensions
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Private(Vec<Key>);

impl Private {
    /// Returns a new empty list of private-use extensions. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::private::Private;
    ///
    /// assert_eq!(Private::new(), Private::default());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// A constructor which takes a pre-sorted list of [`Key`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::private::{Key, Private};
    ///
    /// let key1: Key = "foo".parse().expect("Failed to parse a Key.");
    /// let key2: Key = "bar".parse().expect("Failed to parse a Key.");
    ///
    /// let private = Private::from_vec_unchecked(vec![key1, key2]);
    /// assert_eq!(&private.to_string(), "-x-foo-bar");
    /// ```
    pub fn from_vec_unchecked(input: Vec<Key>) -> Self {
        Self(input)
    }

    /// Empties the [`Private`] list.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::private::{Key, Private};
    ///
    /// let key1: Key = "foo".parse().expect("Failed to parse a Key.");
    /// let key2: Key = "bar".parse().expect("Failed to parse a Key.");
    /// let mut private = Private::from_vec_unchecked(vec![key1, key2]);
    ///
    /// assert_eq!(&private.to_string(), "-x-foo-bar");
    ///
    /// private.clear();
    ///
    /// assert_eq!(&private.to_string(), "");
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub(crate) fn try_from_iter(iter: &mut SubtagIterator) -> Result<Self, ParserError> {
        let keys = iter.map(Key::from_bytes).collect::<Result<Vec<_>, _>>()?;

        Ok(Self::from_vec_unchecked(keys))
    }

    pub(crate) fn for_each_subtag_str<E, F>(&self, f: &mut F) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        if self.is_empty() {
            return Ok(());
        }
        f("x")?;
        self.deref().iter().map(|t| t.as_str()).try_for_each(f)
    }
}

writeable::impl_display_with_writeable!(Private);

impl writeable::Writeable for Private {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        sink.write_str("-x")?;
        for key in self.iter() {
            sink.write_char('-')?;
            writeable::Writeable::write_to(key, sink)?;
        }
        Ok(())
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        if self.is_empty() {
            return writeable::LengthHint::exact(0);
        }
        let mut result = writeable::LengthHint::exact(2);
        for key in self.iter() {
            result += writeable::Writeable::writeable_length_hint(key) + 1;
        }
        result
    }
}

impl Deref for Private {
    type Target = [Key];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
