// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::IdeographicV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    6u8, 48u8, 0u8, 0u8, 8u8, 48u8, 0u8, 0u8, 33u8, 48u8, 0u8, 0u8, 42u8, 48u8,
                    0u8, 0u8, 56u8, 48u8, 0u8, 0u8, 59u8, 48u8, 0u8, 0u8, 0u8, 52u8, 0u8, 0u8,
                    192u8, 77u8, 0u8, 0u8, 0u8, 78u8, 0u8, 0u8, 0u8, 160u8, 0u8, 0u8, 0u8, 249u8,
                    0u8, 0u8, 110u8, 250u8, 0u8, 0u8, 112u8, 250u8, 0u8, 0u8, 218u8, 250u8, 0u8,
                    0u8, 228u8, 111u8, 1u8, 0u8, 229u8, 111u8, 1u8, 0u8, 0u8, 112u8, 1u8, 0u8,
                    248u8, 135u8, 1u8, 0u8, 0u8, 136u8, 1u8, 0u8, 214u8, 140u8, 1u8, 0u8, 0u8,
                    141u8, 1u8, 0u8, 9u8, 141u8, 1u8, 0u8, 112u8, 177u8, 1u8, 0u8, 252u8, 178u8,
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 224u8, 166u8, 2u8, 0u8, 0u8, 167u8, 2u8, 0u8,
                    57u8, 183u8, 2u8, 0u8, 64u8, 183u8, 2u8, 0u8, 30u8, 184u8, 2u8, 0u8, 32u8,
                    184u8, 2u8, 0u8, 162u8, 206u8, 2u8, 0u8, 176u8, 206u8, 2u8, 0u8, 225u8, 235u8,
                    2u8, 0u8, 0u8, 248u8, 2u8, 0u8, 30u8, 250u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8,
                    75u8, 19u8, 3u8, 0u8,
                ])
            },
            101661usize,
        )
    });
