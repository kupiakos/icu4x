// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code for locale canonicalization.

use crate::provider::*;
use alloc::vec::Vec;
use core::cmp::Ordering;

use crate::LocaleExpander;
use crate::TransformResult;
use icu_locid::subtags::{Language, Region, Script};
use icu_locid::{
    extensions_unicode_key as key,
    subtags::{Variant, Variants},
    subtags_language as language, LanguageIdentifier, Locale,
};
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;

/// The LocaleCanonicalizer provides methods to canonicalize Locales and
/// LanguageIdentifiers based upon [`CLDR`] data.
///
/// It currently supports locale canonicalization based upon the canonicalization
/// algorithm from [`UTS #35: Unicode LDML 3. LocaleId Canonicalization`].
///
/// # Examples
///
/// ```
/// use icu_locid::Locale;
/// use icu_locid_transform::{LocaleCanonicalizer, TransformResult};
///
/// let lc = LocaleCanonicalizer::try_new_unstable(&icu_testdata::unstable())
///     .expect("create failed");
///
/// let mut locale: Locale = "ja-Latn-fonipa-hepburn-heploc"
///     .parse()
///     .expect("parse failed");
/// assert_eq!(lc.canonicalize(&mut locale), TransformResult::Modified);
/// assert_eq!(locale.to_string(), "ja-Latn-alalc97-fonipa");
/// ```
///
/// [`ICU4X`]: ../icu/index.html
/// [`CLDR`]: http://cldr.unicode.org/
/// [`UTS #35: Unicode LDML 3. LocaleId Canonicalization`]: http://unicode.org/reports/tr35/#LocaleId_Canonicalization,
pub struct LocaleCanonicalizer {
    /// Data to support canonicalization.
    aliases: DataPayload<AliasesV1Marker>,
    /// Likely subtags implementation for delegation.
    expander: LocaleExpander,
}

#[inline]
fn uts35_rule_matches<'a, I>(
    source: &Locale,
    language: Language,
    script: Option<Script>,
    region: Option<Region>,
    raw_variants: I,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    (language.is_empty() || language == source.id.language)
        && (script.is_none() || script == source.id.script)
        && (region.is_none() || region == source.id.region)
        && {
            // Checks if variants are a subset of source variants.
            // As both iterators are sorted, this can be done linearly.
            let mut source_variants = source.id.variants.iter();
            'outer: for it in raw_variants {
                for cand in source_variants.by_ref() {
                    match cand.strict_cmp(it.as_bytes()) {
                        Ordering::Equal => {
                            continue 'outer;
                        }
                        Ordering::Less => {}
                        _ => {
                            return false;
                        }
                    }
                }
                return false;
            }
            true
        }
}

fn uts35_replacement<'a, I>(
    source: &mut Locale,
    ruletype_has_language: bool,
    ruletype_has_script: bool,
    ruletype_has_region: bool,
    ruletype_variants: Option<I>,
    replacement: &LanguageIdentifier,
) where
    I: Iterator<Item = &'a str>,
{
    if ruletype_has_language || (source.id.language.is_empty() && !replacement.language.is_empty())
    {
        source.id.language = replacement.language;
    }
    if ruletype_has_script || (source.id.script.is_none() && replacement.script.is_some()) {
        source.id.script = replacement.script;
    }
    if ruletype_has_region || (source.id.region.is_none() && replacement.region.is_some()) {
        source.id.region = replacement.region;
    }
    if let Some(skips) = ruletype_variants {
        // The rule matches if the ruletype variants are a subset of the source variants.
        // This means ja-Latn-fonipa-hepburn-heploc matches against the rule for
        // hepburn-heploc and is canonicalized to ja-Latn-alalc97-fonipa

        // We're merging three sorted deduped iterators into a new sequence:
        // sources - skips + replacements

        let mut sources = source.id.variants.iter().copied().peekable();
        let mut replacements = replacement.variants.iter().copied().peekable();
        let mut skips = skips.peekable();

        let mut variants: Vec<Variant> = Vec::new();

        loop {
            match (sources.peek(), skips.peek(), replacements.peek()) {
                (Some(&source), Some(skip), _)
                    if source.strict_cmp(skip.as_bytes()) == Ordering::Greater =>
                {
                    skips.next();
                }
                (Some(&source), Some(skip), _)
                    if source.strict_cmp(skip.as_bytes()) == Ordering::Equal =>
                {
                    skips.next();
                    sources.next();
                }
                (Some(&source), _, Some(&replacement))
                    if replacement.cmp(&source) == Ordering::Less =>
                {
                    variants.push(replacement);
                    replacements.next();
                }
                (Some(&source), _, Some(&replacement))
                    if replacement.cmp(&source) == Ordering::Equal =>
                {
                    variants.push(source);
                    sources.next();
                    replacements.next();
                }
                (Some(&source), _, _) => {
                    variants.push(source);
                    sources.next();
                }
                (None, _, Some(&replacement)) => {
                    variants.push(replacement);
                    replacements.next();
                }
                (None, _, None) => {
                    break;
                }
            }
        }
        source.id.variants = Variants::from_vec_unchecked(variants);
    }
}

#[inline]
fn uts35_check_language_rules(
    locale: &mut Locale,
    alias_data: &DataPayload<AliasesV1Marker>,
) -> TransformResult {
    if !locale.id.language.is_empty() {
        let lang: TinyAsciiStr<3> = locale.id.language.into();
        let replacement = if lang.len() == 2 {
            alias_data.get().language_len2.get(&lang.resize())
        } else {
            alias_data.get().language_len3.get(&lang)
        };

        if let Some(replacement) = replacement {
            if let Ok(langid) = replacement.parse() {
                uts35_replacement::<core::iter::Empty<&str>>(
                    locale, true, false, false, None, &langid,
                );
                return TransformResult::Modified;
            }
        }
    }

    TransformResult::Unmodified
}

fn is_iter_sorted<I, T>(mut iter: I) -> bool
where
    I: Iterator<Item = T>,
    T: PartialOrd,
{
    if let Some(mut last) = iter.next() {
        for curr in iter {
            if last > curr {
                return false;
            }
            last = curr;
        }
    }
    true
}

impl LocaleCanonicalizer {
    /// A constructor which takes a [`DataProvider`] and creates a [`LocaleCanonicalizer`].
    pub fn try_new_unstable<P>(provider: &P) -> Result<LocaleCanonicalizer, DataError>
    where
        P: DataProvider<AliasesV1Marker> + DataProvider<LikelySubtagsV1Marker> + ?Sized,
    {
        let aliases: DataPayload<AliasesV1Marker> =
            provider.load(Default::default())?.take_payload()?;

        let expander = LocaleExpander::try_new_unstable(provider)?;

        Ok(LocaleCanonicalizer { aliases, expander })
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: DataError);

    /// The canonicalize method potentially updates a passed in locale in place
    /// depending up the results of running the canonicalization algorithm
    /// from <http://unicode.org/reports/tr35/#LocaleId_Canonicalization>.
    ///
    /// Some BCP47 canonicalization data is not part of the CLDR json package. Because
    /// of this, some canonicalizations are not performed, e.g. the canonicalization of
    /// `und-u-ca-islamicc` to `und-u-ca-islamic-civil`. This will be fixed in a future
    /// release once the missing data has been added to the CLDR json data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use icu_locid_transform::{LocaleCanonicalizer, TransformResult};
    ///
    /// let lc = LocaleCanonicalizer::try_new_unstable(&icu_testdata::unstable())
    ///     .expect("create failed");
    ///
    /// let mut locale: Locale = "ja-Latn-fonipa-hepburn-heploc"
    ///     .parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.canonicalize(&mut locale), TransformResult::Modified);
    /// assert_eq!(locale.to_string(), "ja-Latn-alalc97-fonipa");
    /// ```
    pub fn canonicalize(&self, locale: &mut Locale) -> TransformResult {
        let mut result = TransformResult::Unmodified;

        // This loops until we get a 'fixed point', where applying the rules do not
        // result in any more changes.
        'outer: loop {
            // These are linear searches due to the ordering imposed by the canonicalization
            // rules, where rules with more variants should be considered first. With the
            // current data in CLDR, we will only do this for locales which have variants,
            // or new rules which we haven't special-cased yet (of which there are fewer
            // than 20).
            if !locale.id.variants.is_empty() {
                // These language/variant comibnations have around 20 rules
                for StrStrPair(raw_lang_variants, raw_to) in self
                    .aliases
                    .get()
                    .language_variants
                    .iter()
                    .map(zerofrom::ZeroFrom::zero_from)
                {
                    let (raw_lang, raw_variants) = {
                        let mut subtags = raw_lang_variants.split('-');
                        (
                            // str::split can't return empty iterators
                            unsafe { subtags.next().unwrap_unchecked() },
                            subtags,
                        )
                    };
                    if is_iter_sorted(raw_variants.clone()) {
                        if let Ok(lang) = raw_lang.parse::<Language>() {
                            if uts35_rule_matches(locale, lang, None, None, raw_variants.clone()) {
                                if let Ok(to) = raw_to.parse() {
                                    uts35_replacement(
                                        locale,
                                        !lang.is_empty(),
                                        false,
                                        false,
                                        Some(raw_variants),
                                        &to,
                                    );
                                    result = TransformResult::Modified;
                                    continue 'outer;
                                }
                            }
                        }
                    }
                }
            } else {
                // These are absolute fallbacks, and currently empty.
                for StrStrPair(raw_from, raw_to) in self
                    .aliases
                    .get()
                    .language
                    .iter()
                    .map(zerofrom::ZeroFrom::zero_from)
                {
                    if let Ok(from) = raw_from.parse::<LanguageIdentifier>() {
                        if uts35_rule_matches(
                            locale,
                            from.language,
                            from.script,
                            from.region,
                            from.variants.iter().map(Variant::as_str),
                        ) {
                            if let Ok(to) = raw_to.parse() {
                                uts35_replacement(
                                    locale,
                                    !from.language.is_empty(),
                                    from.script.is_some(),
                                    from.region.is_some(),
                                    Some(from.variants.iter().map(Variant::as_str)),
                                    &to,
                                );
                                result = TransformResult::Modified;
                                continue 'outer;
                            }
                        }
                    }
                }
            }

            if !locale.id.language.is_empty() {
                // If the region is specified, check sgn-region rules first
                if let Some(region) = locale.id.region {
                    if locale.id.language == language!("sgn") {
                        if let Some(&sgn_lang) = self.aliases.get().sgn_region.get(&region.into()) {
                            uts35_replacement::<core::iter::Empty<&str>>(
                                locale,
                                true,
                                false,
                                true,
                                None,
                                &sgn_lang.into(),
                            );
                            result = TransformResult::Modified;
                            continue;
                        }
                    }
                }

                if uts35_check_language_rules(locale, &self.aliases) == TransformResult::Modified {
                    result = TransformResult::Modified;
                    continue;
                }
            }

            if let Some(script) = locale.id.script {
                if let Some(&replacement) = self.aliases.get().script.get(&script.into()) {
                    locale.id.script = Some(replacement);
                    result = TransformResult::Modified;
                    continue;
                }
            }

            if let Some(region) = locale.id.region {
                let replacement = if region.is_alphabetic() {
                    let region: TinyAsciiStr<3> = region.into();
                    self.aliases.get().region_alpha.get(&region.resize())
                } else {
                    self.aliases.get().region_num.get(&region.into())
                };
                if let Some(&replacement) = replacement {
                    locale.id.region = Some(replacement);
                    result = TransformResult::Modified;
                    continue;
                }

                if let Some(regions) = self.aliases.get().complex_region.get(&region.into()) {
                    // Skip if regions are empty
                    if let Some(default_region) = regions.get(0) {
                        let mut maximized = LanguageIdentifier {
                            language: locale.id.language,
                            script: locale.id.script,
                            region: None,
                            variants: Variants::default(),
                        };

                        locale.id.region = Some(
                            match (self.expander.maximize(&mut maximized), maximized.region) {
                                (TransformResult::Modified, Some(candidate))
                                    if regions.iter().any(|x| x == candidate) =>
                                {
                                    candidate
                                }
                                _ => default_region,
                            },
                        );
                        result = TransformResult::Modified;
                        continue;
                    }
                }
            }

            if !locale.id.variants.is_empty() {
                let mut modified = Vec::new();
                let mut unmodified = Vec::new();
                for &variant in locale.id.variants.iter() {
                    if let Some(&updated) = self.aliases.get().variant.get(&variant.into()) {
                        modified.push(updated);
                    } else {
                        unmodified.push(variant);
                    }
                }

                if !modified.is_empty() {
                    for variant in unmodified {
                        modified.push(variant);
                    }
                    modified.sort();
                    modified.dedup();
                    locale.id.variants = Variants::from_vec_unchecked(modified);
                    result = TransformResult::Modified;
                    continue;
                }
            }

            // Nothing matched in this iteration, we're done.
            break;
        }

        // Handle Locale extensions in their own loops, because these rules do not interact
        // with each other.
        if let Some(lang) = &locale.extensions.transform.lang {
            let mut tlang: Locale = lang.clone().into();
            let mut matched = false;
            loop {
                if uts35_check_language_rules(&mut tlang, &self.aliases)
                    == TransformResult::Modified
                {
                    result = TransformResult::Modified;
                    matched = true;
                    continue;
                }

                break;
            }

            if matched {
                locale.extensions.transform.lang = Some(tlang.id);
            }
        }

        // The `rg` region override and `sd` regional subdivision keys may contain
        // language codes that require canonicalization.
        for key in &[key!("rg"), key!("sd")] {
            if let Some(value) = locale.extensions.unicode.keywords.get_mut(key) {
                if let &[only_value] = value.as_tinystr_slice() {
                    if let Some(modified_value) =
                        self.aliases.get().subdivision.get(&only_value.resize())
                    {
                        if let Ok(modified_value) = modified_value.parse() {
                            *value = modified_value;
                            result = TransformResult::Modified;
                        }
                    }
                }
            }
        }

        result
    }
}

#[test]
fn test_uts35_rule_matches() {
    for (source, rule, result) in [
        ("ja", "und", true),
        ("und-heploc-hepburn", "und-hepburn", true),
        ("ja-heploc-hepburn", "und-hepburn", true),
        ("ja-hepburn", "und-hepburn-heploc", false),
    ] {
        let source = source.parse().unwrap();
        let rule = rule.parse::<LanguageIdentifier>().unwrap();
        assert_eq!(
            uts35_rule_matches(
                &source,
                rule.language,
                rule.script,
                rule.region,
                rule.variants.iter().map(Variant::as_str),
            ),
            result,
            "{}",
            source
        );
    }
}

#[test]
fn test_uts35_replacement() {
    for (locale, rule_0, rule_1, result) in [
        (
            "ja-Latn-fonipa-hepburn-heploc",
            "und-hepburn-heploc",
            "und-alalc97",
            "ja-Latn-alalc97-fonipa",
        ),
        ("sgn-DD", "und-DD", "und-DE", "sgn-DE"),
        ("sgn-DE", "sgn-DE", "gsg", "gsg"),
    ] {
        let mut locale = locale.parse().unwrap();
        let rule_0 = rule_0.parse::<LanguageIdentifier>().unwrap();
        let rule_1 = rule_1.parse().unwrap();
        let result = result.parse::<Locale>().unwrap();
        uts35_replacement(
            &mut locale,
            !rule_0.language.is_empty(),
            rule_0.script.is_some(),
            rule_0.region.is_some(),
            Some(rule_0.variants.iter().map(Variant::as_str)),
            &rule_1,
        );
        assert_eq!(result, locale);
    }
}
