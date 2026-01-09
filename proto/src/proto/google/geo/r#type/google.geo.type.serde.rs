// @generated
impl serde::Serialize for Viewport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.low.is_some() {
            len += 1;
        }
        if self.high.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.geo.r#type.Viewport", len)?;
        if let Some(v) = self.low.as_ref() {
            struct_ser.serialize_field("low", v)?;
        }
        if let Some(v) = self.high.as_ref() {
            struct_ser.serialize_field("high", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Viewport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "low",
            "high",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Low,
            High,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "low" => Ok(GeneratedField::Low),
                            "high" => Ok(GeneratedField::High),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Viewport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.geo.r#type.Viewport")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Viewport, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut low__ = None;
                let mut high__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Low => {
                            if low__.is_some() {
                                return Err(serde::de::Error::duplicate_field("low"));
                            }
                            low__ = map_.next_value()?;
                        }
                        GeneratedField::High => {
                            if high__.is_some() {
                                return Err(serde::de::Error::duplicate_field("high"));
                            }
                            high__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Viewport {
                    low: low__,
                    high: high__,
                })
            }
        }
        deserializer.deserialize_struct("google.geo.r#type.Viewport", FIELDS, GeneratedVisitor)
    }
}
