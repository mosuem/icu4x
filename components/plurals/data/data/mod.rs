// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_plurals_cardinal_v1!($provider);
        impl_plurals_ordinal_v1!($provider);
        impl_plurals_ranges_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        icu_provider::impl_dynamic_data_provider!($provider, [icu::plurals::provider::CardinalV1Marker, icu::plurals::provider::OrdinalV1Marker, icu::plurals::provider::PluralRangesV1Marker], icu_provider::AnyMarker);
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
