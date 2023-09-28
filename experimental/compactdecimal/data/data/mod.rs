// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_compactdecimal_long_v1!($provider);
        impl_compactdecimal_short_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        icu_provider::impl_dynamic_data_provider!($provider, [icu::compactdecimal::provider::LongCompactDecimalFormatDataV1Marker, icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker], icu_provider::AnyMarker);
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
