// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::script::ScriptWithExtensions;
use core::ops::RangeInclusive;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_collections::codepointtrie::{CodePointMapRange, CodePointTrie, TrieValue};
use icu_provider::prelude::*;
use zerofrom::ZeroFrom;
use zerovec::ZeroVecError;

/// A set of characters with a particular property.
///
/// This data enum is extensible, more backends may be added in the future.
/// Old data can be used with newer code but not vice versa.
#[derive(Debug, Eq, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum PropertyCodePointSetV1<'data> {
    /// The set of characters, represented as an inversion list
    InversionList(#[cfg_attr(feature = "serde", serde(borrow))] CodePointInversionList<'data>),
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}

/// A map efficiently storing data about individual characters.
///
/// This data enum is extensible, more backends may be added in the future.
/// Old data can be used with newer code but not vice versa.
#[derive(Clone, Debug, Eq, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum PropertyCodePointMapV1<'data, T: TrieValue> {
    /// A codepoint trie storing the data
    CodePointTrie(#[cfg_attr(feature = "serde", serde(borrow))] CodePointTrie<'data, T>),
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}

/// A struct that efficiently stores `Script` and `Script_Extensions` property data.
#[icu_provider::data_struct(ScriptWithExtensionsPropertyV1Marker = "props/scx@1")]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ScriptWithExtensionsPropertyV1<'data> {
    /// A special data structure for `Script` and `Script_Extensions`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub data: ScriptWithExtensions<'data>,
}

// See CodePointSetData for documentation of these functions
impl<'data> PropertyCodePointSetV1<'data> {
    #[inline]
    pub(crate) fn contains(&self, ch: char) -> bool {
        match *self {
            Self::InversionList(ref l) => l.contains(ch),
        }
    }

    #[inline]
    pub(crate) fn contains32(&self, ch: u32) -> bool {
        match *self {
            Self::InversionList(ref l) => l.contains32(ch),
        }
    }

    #[inline]
    pub(crate) fn iter_ranges(&self) -> impl Iterator<Item = RangeInclusive<u32>> + '_ {
        match *self {
            Self::InversionList(ref l) => l.iter_ranges(),
        }
    }

    #[inline]
    pub(crate) fn from_code_point_inversion_list(l: CodePointInversionList<'static>) -> Self {
        Self::InversionList(l)
    }

    #[inline]
    pub(crate) fn as_code_point_inversion_list(
        &'_ self,
    ) -> Option<&'_ CodePointInversionList<'data>> {
        match *self {
            Self::InversionList(ref l) => Some(l),
            // any other backing data structure that cannot return a CPInvList in O(1) time should return None
        }
    }

    #[inline]
    pub(crate) fn to_code_point_inversion_list(&self) -> CodePointInversionList<'_> {
        match *self {
            Self::InversionList(ref t) => ZeroFrom::zero_from(t),
        }
    }
}

// See CodePointMapData for documentation of these functions
impl<'data, T: TrieValue> PropertyCodePointMapV1<'data, T> {
    #[inline]
    pub(crate) fn get32(&self, ch: u32) -> T {
        match *self {
            Self::CodePointTrie(ref t) => t.get32(ch),
        }
    }

    #[inline]
    pub(crate) fn try_into_converted<P>(
        self,
    ) -> Result<PropertyCodePointMapV1<'data, P>, ZeroVecError>
    where
        P: TrieValue,
    {
        match self {
            Self::CodePointTrie(t) => t
                .try_into_converted()
                .map(PropertyCodePointMapV1::CodePointTrie),
        }
    }

    #[inline]
    pub(crate) fn get_set_for_value(&self, value: T) -> CodePointInversionList<'static> {
        match *self {
            Self::CodePointTrie(ref t) => t.get_set_for_value(value),
        }
    }

    #[inline]
    pub(crate) fn iter_ranges(&self) -> impl Iterator<Item = CodePointMapRange<T>> + '_ {
        match *self {
            Self::CodePointTrie(ref t) => t.iter_ranges(),
        }
    }

    #[inline]
    pub(crate) fn from_code_point_trie(trie: CodePointTrie<'static, T>) -> Self {
        Self::CodePointTrie(trie)
    }

    #[inline]
    pub(crate) fn as_code_point_trie(&self) -> Option<&CodePointTrie<'data, T>> {
        match *self {
            Self::CodePointTrie(ref t) => Some(t),
            // any other backing data structure that cannot return a CPT in O(1) time should return None
        }
    }

    #[inline]
    pub(crate) fn to_code_point_trie(&self) -> CodePointTrie<'_, T> {
        match *self {
            Self::CodePointTrie(ref t) => ZeroFrom::zero_from(t),
        }
    }
}

macro_rules! expand {
    (
        ($(($bin_marker:ident, $bin_s:literal),)+),
        ($(($enum_marker:ident, $enum_s:literal, $value_ty:ident),)+)
    ) => {

            $(
                #[doc = core::concat!("Data marker for the '", $bin_s, "' Unicode property")]
                pub struct $bin_marker;

                impl DataMarker for $bin_marker {
                    type Yokeable = PropertyCodePointSetV1<'static>;
                }
                impl KeyedDataMarker for $bin_marker {
                    const KEY: DataKey = data_key!(concat!("props/", $bin_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $bin_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $bin_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ ::icu_properties::provider::$bin_marker }
                    }
                }
            )+

            $(
                #[doc = core::concat!("Data marker for the '", $enum_s, "' Unicode property")]
                pub struct $enum_marker;

                impl DataMarker for $enum_marker {
                    type Yokeable = PropertyCodePointMapV1<'static, crate::$value_ty>;
                }

                impl KeyedDataMarker for $enum_marker {
                    const KEY: DataKey = data_key!(concat!("props/", $enum_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $enum_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $enum_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ ::icu_properties::provider::$enum_marker }
                    }
                }
            )+
    };
}

expand!(
    (
        // Binary properties
        (AsciiHexDigitV1Marker, "AHex"),
        (AlnumV1Marker, "alnum"),
        (AlphabeticV1Marker, "Alpha"),
        (BidiControlV1Marker, "Bidi_C"),
        (BidiMirroredV1Marker, "Bidi_M"),
        (BlankV1Marker, "blank"),
        (CasedV1Marker, "Cased"),
        (CaseIgnorableV1Marker, "CI"),
        (FullCompositionExclusionV1Marker, "Comp_Ex"),
        (ChangesWhenCasefoldedV1Marker, "CWCF"),
        (ChangesWhenCasemappedV1Marker, "CWCM"),
        (ChangesWhenNfkcCasefoldedV1Marker, "CWKCF"),
        (ChangesWhenLowercasedV1Marker, "CWL"),
        (ChangesWhenTitlecasedV1Marker, "CWT"),
        (ChangesWhenUppercasedV1Marker, "CWU"),
        (DashV1Marker, "Dash"),
        (DeprecatedV1Marker, "Dep"),
        (DefaultIgnorableCodePointV1Marker, "DI"),
        (DiacriticV1Marker, "Dia"),
        (EmojiModifierBaseV1Marker, "EBase"),
        (EmojiComponentV1Marker, "EComp"),
        (EmojiModifierV1Marker, "EMod"),
        (EmojiV1Marker, "Emoji"),
        (EmojiPresentationV1Marker, "EPres"),
        (ExtenderV1Marker, "Ext"),
        (ExtendedPictographicV1Marker, "ExtPict"),
        (GraphV1Marker, "graph"),
        (GraphemeBaseV1Marker, "Gr_Base"),
        (GraphemeExtendV1Marker, "Gr_Ext"),
        (GraphemeLinkV1Marker, "Gr_Link"),
        (HexDigitV1Marker, "Hex"),
        (HyphenV1Marker, "Hyphen"),
        (IdContinueV1Marker, "IDC"),
        (IdeographicV1Marker, "Ideo"),
        (IdStartV1Marker, "IDS"),
        (IdsBinaryOperatorV1Marker, "IDSB"),
        (IdsTrinaryOperatorV1Marker, "IDST"),
        (JoinControlV1Marker, "Join_C"),
        (LogicalOrderExceptionV1Marker, "LOE"),
        (LowercaseV1Marker, "Lower"),
        (MathV1Marker, "Math"),
        (NoncharacterCodePointV1Marker, "NChar"),
        (NfcInertV1Marker, "nfcinert"),
        (NfdInertV1Marker, "nfdinert"),
        (NfkcInertV1Marker, "nfkcinert"),
        (NfkdInertV1Marker, "nfkdinert"),
        (PatternSyntaxV1Marker, "Pat_Syn"),
        (PatternWhiteSpaceV1Marker, "Pat_WS"),
        (PrependedConcatenationMarkV1Marker, "PCM"),
        (PrintV1Marker, "print"),
        (QuotationMarkV1Marker, "QMark"),
        (RadicalV1Marker, "Radical"),
        (RegionalIndicatorV1Marker, "RI"),
        (SoftDottedV1Marker, "SD"),
        (SegmentStarterV1Marker, "segstart"),
        (CaseSensitiveV1Marker, "Sensitive"),
        (SentenceTerminalV1Marker, "STerm"),
        (TerminalPunctuationV1Marker, "Term"),
        (UnifiedIdeographV1Marker, "UIdeo"),
        (UppercaseV1Marker, "Upper"),
        (VariationSelectorV1Marker, "VS"),
        (WhiteSpaceV1Marker, "WSpace"),
        (XdigitV1Marker, "xdigit"),
        (XidContinueV1Marker, "XIDC"),
        (XidStartV1Marker, "XIDS"),
    ),
    (
        // Enum properties
        (
            CanonicalCombiningClassV1Marker,
            "ccc",
            CanonicalCombiningClass
        ),
        (GeneralCategoryV1Marker, "gc", GeneralCategory),
        (BidiClassV1Marker, "bc", BidiClass),
        (ScriptV1Marker, "sc", Script),
        (EastAsianWidthV1Marker, "ea", EastAsianWidth),
        (LineBreakV1Marker, "lb", LineBreak),
        (GraphemeClusterBreakV1Marker, "GCB", GraphemeClusterBreak),
        (WordBreakV1Marker, "WB", WordBreak),
        (SentenceBreakV1Marker, "SB", SentenceBreak),
    )
);
