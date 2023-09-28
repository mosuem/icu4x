// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_normalizer_comp_v1!($provider);
        impl_normalizer_decomp_v1!($provider);
        impl_normalizer_nfd_v1!($provider);
        impl_normalizer_nfdex_v1!($provider);
        impl_normalizer_nfkd_v1!($provider);
        impl_normalizer_nfkdex_v1!($provider);
        impl_normalizer_uts46d_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                icu_provider::impl_dynamic_data_provider!($provider, [icu::normalizer::provider::CanonicalCompositionsV1Marker, icu::normalizer::provider::NonRecursiveDecompositionSupplementV1Marker, icu::normalizer::provider::CanonicalDecompositionDataV1Marker, icu::normalizer::provider::CanonicalDecompositionTablesV1Marker, icu::normalizer::provider::CompatibilityDecompositionSupplementV1Marker, icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker, icu::normalizer::provider::Uts46DecompositionSupplementV1Marker], icu_provider::AnyMarker);
                icu_provider::AsDynamicDataProviderAnyMarkerWrap::as_any_provider(&Self).load_any(key, req)
            }
        }
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
