// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A formatter specifically for the time zone.

use crate::pattern::TimeZoneDataPayloadsBorrowed;
use crate::provider::time_zones::MetazoneId;
use crate::unchecked::MissingInputFieldKind;
use crate::{format::DateTimeInputUnchecked, provider::fields::FieldLength};
use core::fmt;
use fixed_decimal::Decimal;
use icu_decimal::DecimalFormatter;
use icu_time::zone::ZoneNameTimestamp;
use icu_time::{
    zone::{TimeZoneVariant, UtcOffset},
    TimeZone,
};
use writeable::Writeable;

impl crate::provider::time_zones::MetazonePeriod<'_> {
    fn resolve(
        &self,
        time_zone_id: TimeZone,
        zone_name_timestamp: ZoneNameTimestamp,
    ) -> Option<MetazoneId> {
        use zerovec::ule::AsULE;
        let cursor = self.list.get0(&time_zone_id)?;
        let mut metazone_id = None;
        for (bytes, id) in cursor.iter1().rev() {
            let candidate = ZoneNameTimestamp::from_unaligned(*bytes);
            if zone_name_timestamp >= candidate {
                metazone_id = id.get();
                break;
            }
        }
        metazone_id
    }
}

// An enum for time zone format unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum TimeZoneFormatterUnit {
    GenericNonLocation(FieldLength),
    SpecificNonLocation(FieldLength),
    GenericLocation,
    SpecificLocation,
    ExemplarCity,
    #[allow(dead_code)]
    GenericPartialLocation(FieldLength),
    LocalizedOffset(FieldLength),
    Iso8601(Iso8601Format),
    Bcp47Id,
}

#[derive(Debug)]
pub(super) enum FormatTimeZoneError {
    NamesNotLoaded,
    DecimalFormatterNotLoaded,
    Fallback,
    MissingInputField(MissingInputFieldKind),
}

pub(super) trait FormatTimeZone {
    /// Tries to write the timezone to the sink. If a DateTimeError is returned, the sink
    /// has not been touched, so another format can be attempted.
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error>;
}

impl FormatTimeZone for TimeZoneFormatterUnit {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        match *self {
            Self::GenericNonLocation(length) => {
                GenericNonLocationFormat(length).format(sink, input, data_payloads, fdf)
            }
            Self::SpecificNonLocation(length) => {
                SpecificNonLocationFormat(length).format(sink, input, data_payloads, fdf)
            }
            Self::GenericLocation => GenericLocationFormat.format(sink, input, data_payloads, fdf),
            Self::SpecificLocation => {
                SpecificLocationFormat.format(sink, input, data_payloads, fdf)
            }
            Self::ExemplarCity => ExemplarCityFormat.format(sink, input, data_payloads, fdf),
            Self::GenericPartialLocation(length) => {
                GenericPartialLocationFormat(length).format(sink, input, data_payloads, fdf)
            }
            Self::LocalizedOffset(length) => {
                LocalizedOffsetFormat(length).format(sink, input, data_payloads, fdf)
            }
            Self::Iso8601(iso) => iso.format(sink, input, data_payloads, fdf),
            Self::Bcp47Id => Bcp47IdFormat.format(sink, input, data_payloads, fdf),
        }
    }
}

// PT / Pacific Time
struct GenericNonLocationFormat(FieldLength);

impl FormatTimeZone for GenericNonLocationFormat {
    /// Writes the time zone in generic non-location format as defined by the UTS-35 spec.
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneId,
            )));
        };
        let Some(local_time) = input.zone_name_timestamp else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneNameTimestamp,
            )));
        };
        let Some(generic_names) = (match self.0 {
            FieldLength::Four => data_payloads.mz_generic_long.as_ref(),
            _ => data_payloads.mz_generic_short.as_ref(),
        }) else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(standard_names) = (match self.0 {
            FieldLength::Four => data_payloads.mz_standard_long.as_ref(),
            _ => data_payloads.mz_generic_short.as_ref(),
        }) else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(metazone_period) = data_payloads.mz_periods else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };

        let Some(name) = generic_names
            .overrides
            .get(&time_zone_id)
            .or_else(|| standard_names.overrides.get(&time_zone_id))
            .or_else(|| {
                let mz = metazone_period.resolve(time_zone_id, local_time)?;
                generic_names
                    .defaults
                    .get(&mz)
                    .or_else(|| standard_names.defaults.get(&mz))
            })
        else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

// PDT / Pacific Daylight Time
struct SpecificNonLocationFormat(FieldLength);

impl FormatTimeZone for SpecificNonLocationFormat {
    /// Writes the time zone in short specific non-location format as defined by the UTS-35 spec.
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneId,
            )));
        };
        let Some(variant) = input.zone_variant else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneVariant,
            )));
        };
        let Some(local_time) = input.zone_name_timestamp else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneNameTimestamp,
            )));
        };

        let Some(specific) = (match self.0 {
            FieldLength::Four => data_payloads.mz_specific_long.as_ref(),
            _ => data_payloads.mz_specific_short.as_ref(),
        }) else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(metazone_period) = data_payloads.mz_periods else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };

        let name = if variant == TimeZoneVariant::Standard && self.0 == FieldLength::Four {
            let Some(standard_names) = data_payloads.mz_standard_long.as_ref() else {
                return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
            };
            if let Some(n) = specific
                .overrides
                .get(&(time_zone_id, TimeZoneVariant::Standard))
            {
                n
            } else if let Some(mz) = metazone_period.resolve(time_zone_id, local_time) {
                if specific.use_standard.binary_search(&mz).is_ok() {
                    if let Some(n) = standard_names.defaults.get(&mz) {
                        n
                    } else {
                        // The only reason why the name is not in GenericStandard even though we expect it
                        // to be, is that it was deduplicated against the generic location format.
                        return GenericLocationFormat.format(sink, input, data_payloads, _fdf);
                    }
                } else if let Some(n) = specific.defaults.get(&(mz, TimeZoneVariant::Standard)) {
                    n
                } else {
                    return Ok(Err(FormatTimeZoneError::Fallback));
                }
            } else {
                return Ok(Err(FormatTimeZoneError::Fallback));
            }
        } else if let Some(n) = specific
            .overrides
            .get(&(time_zone_id, variant))
            .or_else(|| {
                specific
                    .defaults
                    .get(&(metazone_period.resolve(time_zone_id, local_time)?, variant))
            })
        {
            n
        } else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

// UTC+7:00
struct LocalizedOffsetFormat(FieldLength);

impl FormatTimeZone for LocalizedOffsetFormat {
    /// Writes the time zone in localized offset format according to the CLDR localized hour format.
    /// This goes explicitly against the UTS-35 spec, which specifies long or short localized
    /// offset formats regardless of locale.
    ///
    /// You can see more information about our decision to resolve this conflict here:
    /// <https://docs.google.com/document/d/16GAqaDRS6hzL8jNYjus5MglSevGBflISM-BrIS7bd4A/edit?usp=sharing>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        formatter: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(formatter) = formatter else {
            return Ok(Err(FormatTimeZoneError::DecimalFormatterNotLoaded));
        };
        let Some(offset) = input.zone_offset else {
            sink.write_str(&essentials.offset_unknown)?;
            return Ok(Ok(()));
        };
        struct FormattedOffset<'a> {
            offset: UtcOffset,
            separator: &'a str,
            formatter: &'a DecimalFormatter,
            length: FieldLength,
        }

        impl Writeable for FormattedOffset<'_> {
            fn write_to_parts<S: writeable::PartsWrite + ?Sized>(
                &self,
                sink: &mut S,
            ) -> fmt::Result {
                let decimal = {
                    let mut decimal = Decimal::from(self.offset.hours_part())
                        .with_sign_display(fixed_decimal::SignDisplay::Always);
                    decimal.pad_start(if self.length == FieldLength::Four {
                        2
                    } else {
                        0
                    });
                    decimal
                };
                self.formatter.format(&decimal).write_to(sink)?;

                if self.length == FieldLength::Four
                    || self.offset.minutes_part() != 0
                    || self.offset.seconds_part() != 0
                {
                    let mut decimal = Decimal::from(self.offset.minutes_part());
                    decimal.absolute.pad_start(2);
                    sink.write_str(self.separator)?;
                    self.formatter.format(&decimal).write_to(sink)?;
                }

                if self.offset.seconds_part() != 0 {
                    sink.write_str(self.separator)?;

                    let mut decimal = Decimal::from(self.offset.seconds_part());
                    decimal.absolute.pad_start(2);
                    self.formatter.format(&decimal).write_to(sink)?;
                }

                Ok(())
            }
        }

        essentials
            .offset_pattern
            .interpolate([FormattedOffset {
                offset,
                separator: &essentials.offset_separator,
                formatter,
                length: self.0,
            }])
            .write_to(sink)?;

        Ok(Ok(()))
    }
}

// Los Angeles Time
struct GenericLocationFormat;

impl FormatTimeZone for GenericLocationFormat {
    /// Writes the time zone in generic location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _decimal_formatter: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneId,
            )));
        };

        let Some(locations) = data_payloads.locations else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };

        let Some(locations_root) = data_payloads.locations_root else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };

        let Some(location) = locations
            .locations
            .get(&time_zone_id)
            .or_else(|| locations_root.locations.get(&time_zone_id))
        else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        locations
            .pattern_generic
            .interpolate([location])
            .write_to(sink)?;

        Ok(Ok(()))
    }
}

// Los Angeles Daylight Time
struct SpecificLocationFormat;

impl FormatTimeZone for SpecificLocationFormat {
    /// Writes the time zone in a specific location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _decimal_formatter: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneId,
            )));
        };
        let Some(variant) = input.zone_variant else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneVariant,
            )));
        };
        let Some(locations) = data_payloads.locations else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(locations_root) = data_payloads.locations_root else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };

        let Some(location) = locations
            .locations
            .get(&time_zone_id)
            .or_else(|| locations_root.locations.get(&time_zone_id))
        else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        match variant {
            TimeZoneVariant::Standard => &locations.pattern_standard,
            TimeZoneVariant::Daylight => &locations.pattern_daylight,
            // Compiles out due to tilde dependency on `icu_time`
            _ => unreachable!(),
        }
        .interpolate([location])
        .write_to(sink)?;

        Ok(Ok(()))
    }
}

// Los Angeles
struct ExemplarCityFormat;

impl FormatTimeZone for ExemplarCityFormat {
    /// Writes the time zone exemplar city format as defined by the UTS-35 spec.
    /// e.g. Los Angeles
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneId,
            )));
        };
        let Some(exemplars) = data_payloads.exemplars else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(exemplars_root) = data_payloads.exemplars_root else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(locations) = data_payloads.locations else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(locations_root) = data_payloads.locations_root else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };

        let Some(location) = exemplars
            .exemplars
            .get(&time_zone_id)
            .or_else(|| exemplars_root.exemplars.get(&time_zone_id))
            .or_else(|| locations.locations.get(&time_zone_id))
            .or_else(|| locations_root.locations.get(&time_zone_id))
            .or_else(|| exemplars.exemplars.get(&TimeZone::UNKNOWN))
            .or_else(|| exemplars_root.exemplars.get(&TimeZone::UNKNOWN))
        else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        location.write_to(sink)?;

        Ok(Ok(()))
    }
}

// Pacific Time (Los Angeles) / PT (Los Angeles)
struct GenericPartialLocationFormat(FieldLength);

impl FormatTimeZone for GenericPartialLocationFormat {
    /// Writes the time zone in a long generic partial location format as defined by the UTS-35 spec.
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneId,
            )));
        };
        let Some(local_time) = input.zone_name_timestamp else {
            return Ok(Err(FormatTimeZoneError::MissingInputField(
                MissingInputFieldKind::TimeZoneNameTimestamp,
            )));
        };

        let Some(locations) = data_payloads.locations else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(locations_root) = data_payloads.locations_root else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(non_locations) = (match self.0 {
            FieldLength::Four => data_payloads.mz_generic_long.as_ref(),
            _ => data_payloads.mz_generic_short.as_ref(),
        }) else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(metazone_period) = data_payloads.mz_periods else {
            return Ok(Err(FormatTimeZoneError::NamesNotLoaded));
        };
        let Some(location) = locations
            .locations
            .get(&time_zone_id)
            .or_else(|| locations_root.locations.get(&time_zone_id))
        else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };
        let Some(non_location) = non_locations.overrides.get(&time_zone_id).or_else(|| {
            non_locations
                .defaults
                .get(&metazone_period.resolve(time_zone_id, local_time)?)
        }) else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        locations
            .pattern_partial_location
            .interpolate((location, non_location))
            .write_to(sink)?;

        Ok(Ok(()))
    }
}

/// Whether the minutes field should be optional or required in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Minutes {
    /// Minutes are always displayed.
    Required,

    /// Minutes are displayed only if they are non-zero.
    Optional,
}

/// Whether the seconds field should be optional or excluded in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Seconds {
    /// Seconds are displayed only if they are non-zero.
    Optional,

    /// Seconds are not displayed.
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Iso8601Format {
    // 1000 vs 10:00
    extended: bool,
    // 00:00 vs Z
    z: bool,
    minutes: Minutes,
    seconds: Seconds,
}

impl Iso8601Format {
    pub(crate) fn with_z(length: FieldLength) -> Self {
        match length {
            FieldLength::One => Self {
                extended: false,
                z: true,
                minutes: Minutes::Optional,
                seconds: Seconds::Never,
            },
            FieldLength::Two => Self {
                extended: false,
                z: true,
                minutes: Minutes::Required,
                seconds: Seconds::Never,
            },
            FieldLength::Three => Self {
                extended: true,
                z: true,
                minutes: Minutes::Required,
                seconds: Seconds::Never,
            },
            FieldLength::Four => Self {
                extended: false,
                z: true,
                minutes: Minutes::Required,
                seconds: Seconds::Optional,
            },
            _ => Self {
                extended: true,
                z: true,
                minutes: Minutes::Required,
                seconds: Seconds::Optional,
            },
        }
    }

    pub(crate) fn without_z(length: FieldLength) -> Self {
        match length {
            FieldLength::One => Self {
                extended: false,
                z: false,
                minutes: Minutes::Optional,
                seconds: Seconds::Never,
            },
            FieldLength::Two => Self {
                extended: false,
                z: false,
                minutes: Minutes::Required,
                seconds: Seconds::Never,
            },
            FieldLength::Three => Self {
                extended: true,
                z: false,
                minutes: Minutes::Required,
                seconds: Seconds::Never,
            },
            FieldLength::Four => Self {
                extended: false,
                z: false,
                minutes: Minutes::Required,
                seconds: Seconds::Optional,
            },
            _ => Self {
                extended: true,
                z: false,
                minutes: Minutes::Required,
                seconds: Seconds::Optional,
            },
        }
    }
}

impl FormatTimeZone for Iso8601Format {
    /// Writes a [`UtcOffset`](crate::input::UtcOffset) in ISO-8601 format according to the
    /// given formatting options.
    ///
    /// [`IsoFormat`] determines whether the format should be Basic or Extended,
    /// and whether a zero-offset should be formatted numerically or with
    /// The UTC indicator: "Z"
    /// - Basic    e.g. +0800
    /// - Extended e.g. +08:00
    ///
    /// [`Minutes`] can be required or optional.
    /// [`Seconds`] can be optional or never.
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        _data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(offset) = input.zone_offset else {
            sink.write_str("+?")?;
            return Ok(Ok(()));
        };
        self.format_infallible(sink, offset).map(|()| Ok(()))
    }
}

impl Iso8601Format {
    pub(crate) fn format_infallible<W: writeable::PartsWrite + ?Sized>(
        self,
        sink: &mut W,
        offset: UtcOffset,
    ) -> Result<(), fmt::Error> {
        if offset.is_zero() && self.z {
            return sink.write_char('Z');
        }

        // Always in latin digits according to spec
        {
            let mut fd = Decimal::from(offset.hours_part())
                .with_sign_display(fixed_decimal::SignDisplay::Always);
            fd.pad_start(2);
            fd
        }
        .write_to(sink)?;

        if self.minutes == Minutes::Required
            || (self.minutes == Minutes::Optional && offset.minutes_part() != 0)
        {
            if self.extended {
                sink.write_char(':')?;
            }
            {
                let mut fd = Decimal::from(offset.minutes_part());
                fd.pad_start(2);
                fd
            }
            .write_to(sink)?;
        }

        if self.seconds == Seconds::Optional && offset.seconds_part() != 0 {
            if self.extended {
                sink.write_char(':')?;
            }
            {
                let mut fd = Decimal::from(offset.seconds_part());
                fd.pad_start(2);
                fd
            }
            .write_to(sink)?;
        }

        Ok(())
    }
}

// It is only used for pattern in special case and not public to users.
struct Bcp47IdFormat;

impl FormatTimeZone for Bcp47IdFormat {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &DateTimeInputUnchecked,
        _data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&DecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let time_zone_id = input.zone_id.unwrap_or(TimeZone::UNKNOWN);

        sink.write_str(time_zone_id.as_str())?;

        Ok(Ok(()))
    }
}
