/// This file defines several enums that represent units and unit categories
/// according to the OSCQuery specification. The enums include
/// distance units, angle units, gain units, time units, and speed units.
///
/// see [OSCQuery Proposal](https://github.com/Vidvox/OSCQueryProposal)
///
/// The `OSCUnit` enum combines one of the above unit enums with a value that
/// has that unit. It is intended to be used to represent values with units
/// in OSC messages or other contexts where units are important.
///
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The OSCDistance enum defines a set of units for distance measurements,
///  including meters, kilometers, decimeters,
///  centimeters, millimeters, micrometers,
///  nanometers, picometers, inches,
///  feet, miles, and pixels.
#[derive(Debug, PartialEq, Eq)]
pub enum OSCDistance {
    Meter,
    Kilometer,
    Decimeter,
    Centimeter,
    Millimeter,
    Micrometer,
    Nanometer,
    Picometer,
    Inches,
    Feet,
    Miles,
    Pixels,
}

impl std::fmt::Display for OSCDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OSCDistance::Meter => write!(f, "m"),
            OSCDistance::Kilometer => write!(f, "km"),
            OSCDistance::Decimeter => write!(f, "dm"),
            OSCDistance::Centimeter => write!(f, "cm"),
            OSCDistance::Millimeter => write!(f, "mm"),
            OSCDistance::Micrometer => write!(f, "um"),
            OSCDistance::Nanometer => write!(f, "nm"),
            OSCDistance::Picometer => write!(f, "pm"),
            OSCDistance::Inches => write!(f, "inch"),
            OSCDistance::Feet => write!(f, "feet"),
            OSCDistance::Miles => write!(f, "mile"),
            OSCDistance::Pixels => write!(f, "pixels"),
        }
    }
}

/// The OSCAngle enum defines a set of units for angle measurements,
///  incl uding degrees and radians.
#[derive(Debug, PartialEq, Eq)]
pub enum OSCAngle {
    Degree,
    Radian,
}

impl std::fmt::Display for OSCAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OSCAngle::Degree => write!(f, "degree"),
            OSCAngle::Radian => write!(f, "radian"),
        }
    }
}

/// The OSCGain enum defines a set of units for gain measurements,
///  including linear (normalized range mapping to (-inf 0dB]),
///  midigain (MIDI-adapted gain with recommended mapping),
///  db (clipped to a minimum headroom value),
///  and db-raw (not clipped).
#[derive(Debug, PartialEq, Eq)]
pub enum OSCGain {
    Linear,
    Midigain,
    Db,
    DbRaw,
}

impl std::fmt::Display for OSCGain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OSCGain::Linear => write!(f, "linear"),
            OSCGain::Midigain => write!(f, "midigain"),
            OSCGain::Db => write!(f, "db"),
            OSCGain::DbRaw => write!(f, "db-raw"),
        }
    }
}

///The OSCTime enum defines a set of units for time and pitch measurements,
///  including seconds, bark, bpm, cents, hz,
///  mel, midinote (MIDI note convention),
///  milliseconds, speed, and samples.
#[derive(Debug, PartialEq, Eq)]
pub enum OSCTime {
    Second,
    Bark,
    Bpm,
    Cents,
    Hz,
    Mel,
    Midinote,
    Millisecond,
    Speed,
    Samples,
}

impl std::fmt::Display for OSCTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OSCTime::Second => write!(f, "second"),
            OSCTime::Bark => write!(f, "bark"),
            OSCTime::Bpm => write!(f, "bpm"),
            OSCTime::Cents => write!(f, "cents"),
            OSCTime::Hz => write!(f, "hz"),
            OSCTime::Mel => write!(f, "mel"),
            OSCTime::Midinote => write!(f, "midinote"),
            OSCTime::Millisecond => write!(f, "ms"),
            OSCTime::Speed => write!(f, "speed"),
            OSCTime::Samples => write!(f, "samples"),
        }
    }
}

/// The OSCSpeed enum defines a set of units for speed measurements,
///  including meters per second, miles per hour,
///  kilometers per hour, knots, feet per second,
///  feet per hour, and pixels per second.
#[derive(Debug, PartialEq, Eq)]
pub enum OSCSpeed {
    MetersPerSeconds,
    MilesPerHour,
    KilometersPerHour,
    Knots,
    FeetPerSecond,
    FeetPerHour,
    PixelsPerSecond,
}

impl std::fmt::Display for OSCSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OSCSpeed::MetersPerSeconds => write!(f, "m/s"),
            OSCSpeed::MilesPerHour => write!(f, "mph"),
            OSCSpeed::KilometersPerHour => write!(f, "km/h"),
            OSCSpeed::Knots => write!(f, "knots"),
            OSCSpeed::FeetPerSecond => write!(f, "ft/s"),
            OSCSpeed::FeetPerHour => write!(f, "ft/h"),
            OSCSpeed::PixelsPerSecond => write!(f, "pix/s"),
        }
    }
}

/// The OSCUnit enum is a composite enum
/// that defines the unit types of measurements supported by the OSCQuery protocol.
/// It includes:
/// - Distance (which takes an OSCDistance value),
/// - Angle (which takes an OSCAngle value),
/// - Gain (which takes an OSCGain value),
/// - Time (which takes an OSCTime value),
/// - Speed (which takes an OSCSpeed value).
#[derive(Debug, PartialEq, Eq)]
pub enum OSCUnit {
    Distance(OSCDistance),
    Angle(OSCAngle),
    Gain(OSCGain),
    Time(OSCTime),
    Speed(OSCSpeed),
}

impl std::fmt::Display for OSCUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OSCUnit::Distance(d) => write!(f, "distance.{}", d),
            OSCUnit::Angle(a) => write!(f, "angle.{}", a),
            OSCUnit::Gain(g) => write!(f, "gain.{}", g),
            OSCUnit::Time(t) => write!(f, "time.{}", t),
            OSCUnit::Speed(s) => write!(f, "speed.{}", s),
        }
    }
}

impl Serialize for OSCUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for OSCUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('.').collect();
        match parts.as_slice() {
            ["distance", unit] => Ok(OSCUnit::Distance(match *unit {
                "m" => OSCDistance::Meter,
                "km" => OSCDistance::Kilometer,
                "dm" => OSCDistance::Decimeter,
                "cm" => OSCDistance::Centimeter,
                "mm" => OSCDistance::Millimeter,
                "um" => OSCDistance::Micrometer,
                "nm" => OSCDistance::Nanometer,
                "pm" => OSCDistance::Picometer,
                "inches" => OSCDistance::Inches,
                "feet" => OSCDistance::Feet,
                "miles" => OSCDistance::Miles,
                "pixels" => OSCDistance::Pixels,
                _ => {
                    return Err(serde::de::Error::unknown_variant(
                        unit,
                        &[
                            "m", "km", "dm", "cm", "mm", "um", "nm", "pm", "inches", "feet",
                            "miles", "pixels",
                        ],
                    ))
                }
            })),
            ["angle", unit] => Ok(OSCUnit::Angle(match *unit {
                "degree" => OSCAngle::Degree,
                "radian" => OSCAngle::Radian,
                _ => {
                    return Err(serde::de::Error::unknown_variant(
                        unit,
                        &["degree", "radian"],
                    ))
                }
            })),
            ["gain", unit] => Ok(OSCUnit::Gain(match *unit {
                "linear" => OSCGain::Linear,
                "midigain" => OSCGain::Midigain,
                "db" => OSCGain::Db,
                "db-raw" => OSCGain::DbRaw,
                _ => {
                    return Err(serde::de::Error::unknown_variant(
                        unit,
                        &["linear", "midigain", "db", "db-raw"],
                    ))
                }
            })),
            ["time", unit] => Ok(OSCUnit::Time(match *unit {
                "second" => OSCTime::Second,
                "bark" => OSCTime::Bark,
                "bpm" => OSCTime::Bpm,
                "cents" => OSCTime::Cents,
                "hz" => OSCTime::Hz,
                "mel" => OSCTime::Mel,
                "midinote" => OSCTime::Midinote,
                "ms" => OSCTime::Millisecond,
                "speed" => OSCTime::Speed,
                "samples" => OSCTime::Samples,
                _ => {
                    return Err(serde::de::Error::unknown_variant(
                        unit,
                        &[
                            "second", "bark", "bpm", "cents", "hz", "mel", "midinote", "ms",
                            "speed", "samples",
                        ],
                    ))
                }
            })),
            ["speed", unit] => Ok(OSCUnit::Speed(match *unit {
                "m/s" => OSCSpeed::MetersPerSeconds,
                "mph" => OSCSpeed::MilesPerHour,
                "km/h" => OSCSpeed::KilometersPerHour,
                "kn" => OSCSpeed::Knots,
                "ft/s" => OSCSpeed::FeetPerSecond,
                "ft/h" => OSCSpeed::FeetPerHour,
                "pix/s" => OSCSpeed::PixelsPerSecond,
                _ => {
                    return Err(serde::de::Error::unknown_variant(
                        unit,
                        &["m/s", "mph", "km/h", "kn", "ft/s", "ft/h"],
                    ))
                }
            })),
            _ => {
                return Err(serde::de::Error::unknown_variant(
                    parts.concat().as_str(),
                    &[
                        "distance.<..>",
                        "angle<..>",
                        "gain<..>",
                        "time<..>",
                        "speed<..>",
                    ],
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&OSCUnit::Distance(OSCDistance::Kilometer)).unwrap(),
            r#""distance.km""#
        );
        assert_eq!(
            serde_json::to_string(&OSCUnit::Speed(OSCSpeed::MetersPerSeconds)).unwrap(),
            r#""speed.m/s""#
        );
        assert_eq!(
            serde_json::to_string(&OSCUnit::Time(OSCTime::Samples)).unwrap(),
            r#""time.samples""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<OSCUnit>(r#""distance.km""#).unwrap(),
            OSCUnit::Distance(OSCDistance::Kilometer)
        );
        assert_eq!(
            serde_json::from_str::<OSCUnit>(r#""speed.m/s""#).unwrap(),
            OSCUnit::Speed(OSCSpeed::MetersPerSeconds)
        );
        assert_eq!(
            serde_json::from_str::<OSCUnit>(r#""time.samples""#).unwrap(),
            OSCUnit::Time(OSCTime::Samples)
        );
    }

    #[test]
    fn test_serialize_deserialize() {
        let osc_units = vec![
            OSCUnit::Distance(OSCDistance::Meter),
            OSCUnit::Angle(OSCAngle::Degree),
            OSCUnit::Gain(OSCGain::Db),
            OSCUnit::Time(OSCTime::Bpm),
            OSCUnit::Speed(OSCSpeed::KilometersPerHour),
        ];

        for osc_unit in osc_units {
            let json_str = serde_json::to_string(&osc_unit).unwrap();
            let parsed_osc_unit = serde_json::from_str::<OSCUnit>(&json_str).unwrap();

            assert_eq!(osc_unit, parsed_osc_unit);
        }
    }

    #[test]
    fn test_deserialize_invalid_string() {
        let invalid_str = r#""invalid.string""#;
        let result = serde_json::from_str::<OSCUnit>(invalid_str);
        assert!(result.is_err());
    }
}
