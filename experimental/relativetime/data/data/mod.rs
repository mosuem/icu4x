// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_relativetime_long_day_v1!($provider);
        impl_relativetime_long_hour_v1!($provider);
        impl_relativetime_long_minute_v1!($provider);
        impl_relativetime_long_month_v1!($provider);
        impl_relativetime_long_quarter_v1!($provider);
        impl_relativetime_long_second_v1!($provider);
        impl_relativetime_long_week_v1!($provider);
        impl_relativetime_long_year_v1!($provider);
        impl_relativetime_narrow_day_v1!($provider);
        impl_relativetime_narrow_hour_v1!($provider);
        impl_relativetime_narrow_minute_v1!($provider);
        impl_relativetime_narrow_month_v1!($provider);
        impl_relativetime_narrow_quarter_v1!($provider);
        impl_relativetime_narrow_second_v1!($provider);
        impl_relativetime_narrow_week_v1!($provider);
        impl_relativetime_narrow_year_v1!($provider);
        impl_relativetime_short_day_v1!($provider);
        impl_relativetime_short_hour_v1!($provider);
        impl_relativetime_short_minute_v1!($provider);
        impl_relativetime_short_month_v1!($provider);
        impl_relativetime_short_quarter_v1!($provider);
        impl_relativetime_short_second_v1!($provider);
        impl_relativetime_short_week_v1!($provider);
        impl_relativetime_short_year_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        icu_provider::impl_dynamic_data_provider!($provider, [icu::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker, icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker, icu::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker, icu::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker, icu::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker, icu::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker, icu::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker, icu::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker, icu::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker, icu::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker, icu::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker, icu::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker, icu::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker, icu::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker, icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker, icu::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker, icu::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker, icu::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker, icu::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker, icu::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker, icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker, icu::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker, icu::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker, icu::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker], icu_provider::AnyMarker);
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
