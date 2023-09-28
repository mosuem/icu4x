// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_collator_data_v1!($provider);
        impl_collator_dia_v1!($provider);
        impl_collator_jamo_v1!($provider);
        impl_collator_meta_v1!($provider);
        impl_collator_prim_v1!($provider);
        impl_collator_reord_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                icu_provider::impl_dynamic_data_provider!($provider, [icu::collator::provider::CollationDataV1Marker, icu::collator::provider::CollationDiacriticsV1Marker, icu::collator::provider::CollationJamoV1Marker, icu::collator::provider::CollationMetadataV1Marker, icu::collator::provider::CollationSpecialPrimariesV1Marker, icu::collator::provider::CollationReorderingV1Marker], icu_provider::AnyMarker);
                icu_provider::AsDynamicDataProviderAnyMarkerWrap::as_any_provider(&Self).load_any(key, req)
            }
        }
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
