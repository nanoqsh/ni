use {
    crate::name::Name,
    core::{fmt, str},
    serde::{Deserialize, Serialize, de},
};

impl Serialize for Name {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.decode().as_str())
    }
}

impl<'de> Deserialize<'de> for Name {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visit;

        impl de::Visitor<'_> for Visit {
            type Value = Name;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a name")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Name::encode(v).map_err(E::custom)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Name::encode(v.as_bytes()).map_err(E::custom)
            }

            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Name::encode_char(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visit)
    }
}
