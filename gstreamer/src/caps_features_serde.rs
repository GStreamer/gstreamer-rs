// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use serde::{
    de,
    de::{Deserialize, Deserializer, EnumAccess, SeqAccess, VariantAccess, Visitor},
    ser::{Serialize, SerializeSeq, Serializer},
};

use crate::{CapsFeatures, CapsFeaturesRef};

enum CapsFeaturesVariantKinds {
    Any,
    Some,
}

const CAPS_FEATURES_VARIANT_ANY_ID: u32 = 0;
const CAPS_FEATURES_VARIANT_ANY_STR: &str = "Any";
const CAPS_FEATURES_VARIANT_SOME_ID: u32 = 1;
const CAPS_FEATURES_VARIANT_SOME_STR: &str = "Some";

const CAPS_FEATURES_VARIANT_NAMES: &[&str] = &[
    CAPS_FEATURES_VARIANT_ANY_STR,
    CAPS_FEATURES_VARIANT_SOME_STR,
];

struct CapsFeaturesForIterSe<'a>(&'a CapsFeaturesRef);
impl Serialize for CapsFeaturesForIterSe<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let iter = self.0.iter();
        let size = iter.size_hint().0;
        if size > 0 {
            let mut seq = serializer.serialize_seq(Some(size))?;
            for feature in iter {
                seq.serialize_element(feature.as_str())?;
            }
            seq.end()
        } else {
            let seq = serializer.serialize_seq(None)?;
            seq.end()
        }
    }
}

impl Serialize for CapsFeaturesRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.is_any() {
            serializer.serialize_unit_variant(
                stringify!(CapsFeatures),
                CAPS_FEATURES_VARIANT_ANY_ID,
                CAPS_FEATURES_VARIANT_ANY_STR,
            )
        } else {
            serializer.serialize_newtype_variant(
                stringify!(CapsFeatures),
                CAPS_FEATURES_VARIANT_SOME_ID,
                CAPS_FEATURES_VARIANT_SOME_STR,
                &CapsFeaturesForIterSe(self),
            )
        }
    }
}

impl Serialize for CapsFeatures {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

struct CapsFeaturesSome(CapsFeatures);

struct CapsFeaturesSomeVisitor;
impl<'de> Visitor<'de> for CapsFeaturesSomeVisitor {
    type Value = CapsFeaturesSome;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of `&str`")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut features = CapsFeatures::new_empty();
        while let Some(feature) = seq.next_element::<String>()? {
            features.add(feature.as_str());
        }
        Ok(CapsFeaturesSome(features))
    }
}

impl<'de> Deserialize<'de> for CapsFeaturesSome {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<CapsFeaturesSome, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_seq(CapsFeaturesSomeVisitor)
    }
}

struct CapsFeaturesVariantKindsVisitor;
impl Visitor<'_> for CapsFeaturesVariantKindsVisitor {
    type Value = CapsFeaturesVariantKinds;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Caps variant kind (`Any` or `Some`)")
    }

    fn visit_u32<E: de::Error>(self, value: u32) -> Result<Self::Value, E> {
        match value {
            CAPS_FEATURES_VARIANT_ANY_ID => Ok(CapsFeaturesVariantKinds::Any),
            CAPS_FEATURES_VARIANT_SOME_ID => Ok(CapsFeaturesVariantKinds::Some),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Unsigned(u64::from(value)),
                &self,
            )),
        }
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            CAPS_FEATURES_VARIANT_ANY_STR => Ok(CapsFeaturesVariantKinds::Any),
            CAPS_FEATURES_VARIANT_SOME_STR => Ok(CapsFeaturesVariantKinds::Some),
            _ => Err(de::Error::unknown_variant(
                value,
                CAPS_FEATURES_VARIANT_NAMES,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for CapsFeaturesVariantKinds {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_identifier(CapsFeaturesVariantKindsVisitor)
    }
}

struct CapsFeaturesVisitor;
impl<'de> Visitor<'de> for CapsFeaturesVisitor {
    type Value = CapsFeatures;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a CapsFeatures enum (`Any` or `Some()`)")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let res = match data.variant()? {
            (CapsFeaturesVariantKinds::Any, _v) => CapsFeatures::new_any(),
            (CapsFeaturesVariantKinds::Some, v) => v
                .newtype_variant::<CapsFeaturesSome>()
                .map(|caps_features_some| caps_features_some.0)?,
        };

        Ok(res)
    }
}

impl<'de> Deserialize<'de> for CapsFeatures {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_enum(
            stringify!(Caps),
            CAPS_FEATURES_VARIANT_NAMES,
            CapsFeaturesVisitor,
        )
    }
}
