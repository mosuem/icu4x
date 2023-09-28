// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_calendar_japanese_v1!($provider);
        impl_calendar_japanext_v1!($provider);
        impl_datetime_week_data_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                icu_provider::impl_dynamic_data_provider!($provider, [icu::calendar::provider::JapaneseErasV1Marker, icu::calendar::provider::JapaneseExtendedErasV1Marker, icu::calendar::provider::WeekDataV1Marker], icu_provider::AnyMarker);
                icu_provider::AsDynamicDataProviderAnyMarkerWrap::as_any_provider(&Self).load_any(key, req)
            }
        }
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
