// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_datetime_buddhist_datelengths_v1!($provider);
        impl_datetime_buddhist_datesymbols_v1!($provider);
        impl_datetime_chinese_datelengths_v1!($provider);
        impl_datetime_chinese_datesymbols_v1!($provider);
        impl_datetime_coptic_datelengths_v1!($provider);
        impl_datetime_coptic_datesymbols_v1!($provider);
        impl_datetime_dangi_datelengths_v1!($provider);
        impl_datetime_dangi_datesymbols_v1!($provider);
        impl_datetime_ethiopic_datelengths_v1!($provider);
        impl_datetime_ethiopic_datesymbols_v1!($provider);
        impl_datetime_gregory_datelengths_v1!($provider);
        impl_datetime_gregory_datesymbols_v1!($provider);
        impl_datetime_hebrew_datelengths_v1!($provider);
        impl_datetime_hebrew_datesymbols_v1!($provider);
        impl_datetime_indian_datelengths_v1!($provider);
        impl_datetime_indian_datesymbols_v1!($provider);
        impl_datetime_islamic_datelengths_v1!($provider);
        impl_datetime_islamic_datesymbols_v1!($provider);
        impl_datetime_japanese_datelengths_v1!($provider);
        impl_datetime_japanese_datesymbols_v1!($provider);
        impl_datetime_japanext_datelengths_v1!($provider);
        impl_datetime_japanext_datesymbols_v1!($provider);
        impl_datetime_patterns_buddhist_date_v1!($provider);
        impl_datetime_patterns_chinese_date_v1!($provider);
        impl_datetime_patterns_coptic_date_v1!($provider);
        impl_datetime_patterns_dangi_date_v1!($provider);
        impl_datetime_patterns_datetime_v1!($provider);
        impl_datetime_patterns_ethiopic_date_v1!($provider);
        impl_datetime_patterns_gregory_date_v1!($provider);
        impl_datetime_patterns_hebrew_date_v1!($provider);
        impl_datetime_patterns_indian_date_v1!($provider);
        impl_datetime_patterns_islamic_date_v1!($provider);
        impl_datetime_patterns_japanese_date_v1!($provider);
        impl_datetime_patterns_japanext_date_v1!($provider);
        impl_datetime_patterns_persian_date_v1!($provider);
        impl_datetime_patterns_roc_date_v1!($provider);
        impl_datetime_patterns_time_v1!($provider);
        impl_datetime_persian_datelengths_v1!($provider);
        impl_datetime_persian_datesymbols_v1!($provider);
        impl_datetime_roc_datelengths_v1!($provider);
        impl_datetime_roc_datesymbols_v1!($provider);
        impl_datetime_skeletons_v1!($provider);
        impl_datetime_symbols_buddhist_months_v1!($provider);
        impl_datetime_symbols_buddhist_years_v1!($provider);
        impl_datetime_symbols_chinese_months_v1!($provider);
        impl_datetime_symbols_chinese_years_v1!($provider);
        impl_datetime_symbols_coptic_months_v1!($provider);
        impl_datetime_symbols_coptic_years_v1!($provider);
        impl_datetime_symbols_dangi_months_v1!($provider);
        impl_datetime_symbols_dangi_years_v1!($provider);
        impl_datetime_symbols_dayperiods_v1!($provider);
        impl_datetime_symbols_ethiopic_months_v1!($provider);
        impl_datetime_symbols_ethiopic_years_v1!($provider);
        impl_datetime_symbols_gregory_months_v1!($provider);
        impl_datetime_symbols_gregory_years_v1!($provider);
        impl_datetime_symbols_hebrew_months_v1!($provider);
        impl_datetime_symbols_hebrew_years_v1!($provider);
        impl_datetime_symbols_indian_months_v1!($provider);
        impl_datetime_symbols_indian_years_v1!($provider);
        impl_datetime_symbols_islamic_months_v1!($provider);
        impl_datetime_symbols_islamic_years_v1!($provider);
        impl_datetime_symbols_japanese_months_v1!($provider);
        impl_datetime_symbols_japanese_years_v1!($provider);
        impl_datetime_symbols_japanext_months_v1!($provider);
        impl_datetime_symbols_japanext_years_v1!($provider);
        impl_datetime_symbols_persian_months_v1!($provider);
        impl_datetime_symbols_persian_years_v1!($provider);
        impl_datetime_symbols_roc_months_v1!($provider);
        impl_datetime_symbols_roc_years_v1!($provider);
        impl_datetime_symbols_weekdays_v1!($provider);
        impl_datetime_timelengths_v1!($provider);
        impl_datetime_timesymbols_v1!($provider);
        impl_time_zone_exemplar_cities_v1!($provider);
        impl_time_zone_formats_v1!($provider);
        impl_time_zone_generic_long_v1!($provider);
        impl_time_zone_generic_short_v1!($provider);
        impl_time_zone_specific_long_v1!($provider);
        impl_time_zone_specific_short_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        icu_provider::impl_dynamic_data_provider!($provider, [icu::datetime::provider::calendar::BuddhistDateLengthsV1Marker, icu::datetime::provider::calendar::BuddhistDateSymbolsV1Marker, icu::datetime::provider::calendar::ChineseDateLengthsV1Marker, icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker, icu::datetime::provider::calendar::CopticDateLengthsV1Marker, icu::datetime::provider::calendar::CopticDateSymbolsV1Marker, icu::datetime::provider::calendar::DangiDateLengthsV1Marker, icu::datetime::provider::calendar::DangiDateSymbolsV1Marker, icu::datetime::provider::calendar::EthiopianDateLengthsV1Marker, icu::datetime::provider::calendar::EthiopianDateSymbolsV1Marker, icu::datetime::provider::calendar::GregorianDateLengthsV1Marker, icu::datetime::provider::calendar::GregorianDateSymbolsV1Marker, icu::datetime::provider::calendar::HebrewDateLengthsV1Marker, icu::datetime::provider::calendar::HebrewDateSymbolsV1Marker, icu::datetime::provider::calendar::IndianDateLengthsV1Marker, icu::datetime::provider::calendar::IndianDateSymbolsV1Marker, icu::datetime::provider::calendar::IslamicDateLengthsV1Marker, icu::datetime::provider::calendar::IslamicDateSymbolsV1Marker, icu::datetime::provider::calendar::JapaneseDateLengthsV1Marker, icu::datetime::provider::calendar::JapaneseDateSymbolsV1Marker, icu::datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker, icu::datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker, icu::datetime::provider::neo::BuddhistDatePatternV1Marker, icu::datetime::provider::neo::ChineseDatePatternV1Marker, icu::datetime::provider::neo::CopticDatePatternV1Marker, icu::datetime::provider::neo::DangiDatePatternV1Marker, icu::datetime::provider::neo::DateTimePatternV1Marker, icu::datetime::provider::neo::EthiopianDatePatternV1Marker, icu::datetime::provider::neo::GregorianDatePatternV1Marker, icu::datetime::provider::neo::HebrewDatePatternV1Marker, icu::datetime::provider::neo::IndianDatePatternV1Marker, icu::datetime::provider::neo::IslamicDatePatternV1Marker, icu::datetime::provider::neo::JapaneseDatePatternV1Marker, icu::datetime::provider::neo::JapaneseExtendedDatePatternV1Marker, icu::datetime::provider::neo::PersianDatePatternV1Marker, icu::datetime::provider::neo::RocDatePatternV1Marker, icu::datetime::provider::neo::TimePatternV1Marker, icu::datetime::provider::calendar::PersianDateLengthsV1Marker, icu::datetime::provider::calendar::PersianDateSymbolsV1Marker, icu::datetime::provider::calendar::RocDateLengthsV1Marker, icu::datetime::provider::calendar::RocDateSymbolsV1Marker, icu::datetime::provider::calendar::DateSkeletonPatternsV1Marker, icu::datetime::provider::neo::BuddhistMonthSymbolsV1Marker, icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker, icu::datetime::provider::neo::ChineseMonthSymbolsV1Marker, icu::datetime::provider::neo::ChineseYearSymbolsV1Marker, icu::datetime::provider::neo::CopticMonthSymbolsV1Marker, icu::datetime::provider::neo::CopticYearSymbolsV1Marker, icu::datetime::provider::neo::DangiMonthSymbolsV1Marker, icu::datetime::provider::neo::DangiYearSymbolsV1Marker, icu::datetime::provider::neo::DayPeriodSymbolsV1Marker, icu::datetime::provider::neo::EthiopianMonthSymbolsV1Marker, icu::datetime::provider::neo::EthiopianYearSymbolsV1Marker, icu::datetime::provider::neo::GregorianMonthSymbolsV1Marker, icu::datetime::provider::neo::GregorianYearSymbolsV1Marker, icu::datetime::provider::neo::HebrewMonthSymbolsV1Marker, icu::datetime::provider::neo::HebrewYearSymbolsV1Marker, icu::datetime::provider::neo::IndianMonthSymbolsV1Marker, icu::datetime::provider::neo::IndianYearSymbolsV1Marker, icu::datetime::provider::neo::IslamicMonthSymbolsV1Marker, icu::datetime::provider::neo::IslamicYearSymbolsV1Marker, icu::datetime::provider::neo::JapaneseMonthSymbolsV1Marker, icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker, icu::datetime::provider::neo::JapaneseExtendedMonthSymbolsV1Marker, icu::datetime::provider::neo::JapaneseExtendedYearSymbolsV1Marker, icu::datetime::provider::neo::PersianMonthSymbolsV1Marker, icu::datetime::provider::neo::PersianYearSymbolsV1Marker, icu::datetime::provider::neo::RocMonthSymbolsV1Marker, icu::datetime::provider::neo::RocYearSymbolsV1Marker, icu::datetime::provider::neo::WeekdaySymbolsV1Marker, icu::datetime::provider::calendar::TimeLengthsV1Marker, icu::datetime::provider::calendar::TimeSymbolsV1Marker, icu::datetime::provider::time_zones::ExemplarCitiesV1Marker, icu::datetime::provider::time_zones::TimeZoneFormatsV1Marker, icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker, icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker, icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker, icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker], icu_provider::AnyMarker);
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
