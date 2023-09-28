// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_displaynames_languages_v1!($provider);
        impl_displaynames_locales_v1!($provider);
        impl_displaynames_regions_v1!($provider);
        impl_displaynames_scripts_v1!($provider);
        impl_displaynames_variants_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        icu_provider::impl_dynamic_data_provider!($provider, [icu::displaynames::provider::LanguageDisplayNamesV1Marker, icu::displaynames::provider::LocaleDisplayNamesV1Marker, icu::displaynames::provider::RegionDisplayNamesV1Marker, icu::displaynames::provider::ScriptDisplayNamesV1Marker, icu::displaynames::provider::VariantDisplayNamesV1Marker], icu_provider::AnyMarker);
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
