// @generated
type DataStruct = < :: icu_provider_adapters :: fallback :: provider :: CollationFallbackSupplementV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_provider_adapters::fallback::provider::LocaleFallbackSupplementV1 {
        parents: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 121u8, 117u8, 101u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        122u8, 104u8, 0u8, 1u8, 72u8, 97u8, 110u8, 116u8, 0u8, 0u8, 0u8, 0u8,
                    ])
                },
            )
        },
        unicode_extension_defaults: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap2d::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[99u8, 111u8]) },
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[2u8, 0u8, 0u8, 0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 122u8, 104u8, 122u8, 104u8, 45u8,
                        72u8, 97u8, 110u8, 116u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 112u8, 105u8, 110u8, 121u8, 105u8,
                        110u8, 115u8, 116u8, 114u8, 111u8, 107u8, 101u8,
                    ])
                },
            )
        },
    };
