// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_segmenter_dictionary_w_auto_v1!($provider);
        impl_segmenter_dictionary_wl_ext_v1!($provider);
        impl_segmenter_grapheme_v1!($provider);
        impl_segmenter_line_v1!($provider);
        impl_segmenter_lstm_wl_auto_v1!($provider);
        impl_segmenter_sentence_v1!($provider);
        impl_segmenter_word_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                icu_provider::impl_dynamic_data_provider!($provider, [icu::segmenter::provider::DictionaryForWordOnlyAutoV1Marker, icu::segmenter::provider::DictionaryForWordLineExtendedV1Marker, icu::segmenter::provider::GraphemeClusterBreakDataV1Marker, icu::segmenter::provider::LineBreakDataV1Marker, icu::segmenter::provider::LstmForWordLineAutoV1Marker, icu::segmenter::provider::SentenceBreakDataV1Marker, icu::segmenter::provider::WordBreakDataV1Marker], icu_provider::AnyMarker);
                icu_provider::AsDynamicDataProviderAnyMarkerWrap::as_any_provider(&Self).load_any(key, req)
            }
        }
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
