// @generated
impl serde::Serialize for ::pbjson_types::compiler::CodeGeneratorRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_to_generate.is_empty() {
            len += 1;
        }
        if self.parameter.is_some() {
            len += 1;
        }
        if !self.proto_file.is_empty() {
            len += 1;
        }
        if self.compiler_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.compiler.CodeGeneratorRequest", len)?;
        if !self.file_to_generate.is_empty() {
            struct_ser.serialize_field("fileToGenerate", &self.file_to_generate)?;
        }
        if let Some(v) = self.parameter.as_ref() {
            struct_ser.serialize_field("parameter", v)?;
        }
        if !self.proto_file.is_empty() {
            struct_ser.serialize_field("protoFile", &self.proto_file)?;
        }
        if let Some(v) = self.compiler_version.as_ref() {
            struct_ser.serialize_field("compilerVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ::pbjson_types::compiler::CodeGeneratorRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_to_generate",
            "fileToGenerate",
            "parameter",
            "proto_file",
            "protoFile",
            "compiler_version",
            "compilerVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileToGenerate,
            Parameter,
            ProtoFile,
            CompilerVersion,
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
                            "fileToGenerate" | "file_to_generate" => Ok(GeneratedField::FileToGenerate),
                            "parameter" => Ok(GeneratedField::Parameter),
                            "protoFile" | "proto_file" => Ok(GeneratedField::ProtoFile),
                            "compilerVersion" | "compiler_version" => Ok(GeneratedField::CompilerVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ::pbjson_types::compiler::CodeGeneratorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.compiler.CodeGeneratorRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<::pbjson_types::compiler::CodeGeneratorRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_to_generate__ = None;
                let mut parameter__ = None;
                let mut proto_file__ = None;
                let mut compiler_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FileToGenerate => {
                            if file_to_generate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileToGenerate"));
                            }
                            file_to_generate__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parameter => {
                            if parameter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            parameter__ = map_.next_value()?;
                        }
                        GeneratedField::ProtoFile => {
                            if proto_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoFile"));
                            }
                            proto_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CompilerVersion => {
                            if compiler_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compilerVersion"));
                            }
                            compiler_version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(::pbjson_types::compiler::CodeGeneratorRequest {
                    file_to_generate: file_to_generate__.unwrap_or_default(),
                    parameter: parameter__,
                    proto_file: proto_file__.unwrap_or_default(),
                    compiler_version: compiler_version__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.compiler.CodeGeneratorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ::pbjson_types::compiler::CodeGeneratorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.error.is_some() {
            len += 1;
        }
        if self.supported_features.is_some() {
            len += 1;
        }
        if !self.file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.compiler.CodeGeneratorResponse", len)?;
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.supported_features.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("supportedFeatures", ToString::to_string(&v).as_str())?;
        }
        if !self.file.is_empty() {
            struct_ser.serialize_field("file", &self.file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ::pbjson_types::compiler::CodeGeneratorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error",
            "supported_features",
            "supportedFeatures",
            "file",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            SupportedFeatures,
            File,
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
                            "error" => Ok(GeneratedField::Error),
                            "supportedFeatures" | "supported_features" => Ok(GeneratedField::SupportedFeatures),
                            "file" => Ok(GeneratedField::File),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ::pbjson_types::compiler::CodeGeneratorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.compiler.CodeGeneratorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<::pbjson_types::compiler::CodeGeneratorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                let mut supported_features__ = None;
                let mut file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                        GeneratedField::SupportedFeatures => {
                            if supported_features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportedFeatures"));
                            }
                            supported_features__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(::pbjson_types::compiler::CodeGeneratorResponse {
                    error: error__,
                    supported_features: supported_features__,
                    file: file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.compiler.CodeGeneratorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ::pbjson_types::compiler::code_generator_response::Feature {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "FEATURE_NONE",
            Self::Proto3Optional => "FEATURE_PROTO3_OPTIONAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ::pbjson_types::compiler::code_generator_response::Feature {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FEATURE_NONE",
            "FEATURE_PROTO3_OPTIONAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ::pbjson_types::compiler::code_generator_response::Feature;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FEATURE_NONE" => Ok(::pbjson_types::compiler::code_generator_response::Feature::None),
                    "FEATURE_PROTO3_OPTIONAL" => Ok(::pbjson_types::compiler::code_generator_response::Feature::Proto3Optional),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ::pbjson_types::compiler::code_generator_response::File {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.insertion_point.is_some() {
            len += 1;
        }
        if self.content.is_some() {
            len += 1;
        }
        if self.generated_code_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.compiler.CodeGeneratorResponse.File", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.insertion_point.as_ref() {
            struct_ser.serialize_field("insertionPoint", v)?;
        }
        if let Some(v) = self.content.as_ref() {
            struct_ser.serialize_field("content", v)?;
        }
        if let Some(v) = self.generated_code_info.as_ref() {
            struct_ser.serialize_field("generatedCodeInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ::pbjson_types::compiler::code_generator_response::File {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "insertion_point",
            "insertionPoint",
            "content",
            "generated_code_info",
            "generatedCodeInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            InsertionPoint,
            Content,
            GeneratedCodeInfo,
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
                            "name" => Ok(GeneratedField::Name),
                            "insertionPoint" | "insertion_point" => Ok(GeneratedField::InsertionPoint),
                            "content" => Ok(GeneratedField::Content),
                            "generatedCodeInfo" | "generated_code_info" => Ok(GeneratedField::GeneratedCodeInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ::pbjson_types::compiler::code_generator_response::File;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.compiler.CodeGeneratorResponse.File")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<::pbjson_types::compiler::code_generator_response::File, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut insertion_point__ = None;
                let mut content__ = None;
                let mut generated_code_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::InsertionPoint => {
                            if insertion_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insertionPoint"));
                            }
                            insertion_point__ = map_.next_value()?;
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = map_.next_value()?;
                        }
                        GeneratedField::GeneratedCodeInfo => {
                            if generated_code_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generatedCodeInfo"));
                            }
                            generated_code_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(::pbjson_types::compiler::code_generator_response::File {
                    name: name__,
                    insertion_point: insertion_point__,
                    content: content__,
                    generated_code_info: generated_code_info__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.compiler.CodeGeneratorResponse.File", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ::pbjson_types::compiler::Version {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.major.is_some() {
            len += 1;
        }
        if self.minor.is_some() {
            len += 1;
        }
        if self.patch.is_some() {
            len += 1;
        }
        if self.suffix.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.compiler.Version", len)?;
        if let Some(v) = self.major.as_ref() {
            struct_ser.serialize_field("major", v)?;
        }
        if let Some(v) = self.minor.as_ref() {
            struct_ser.serialize_field("minor", v)?;
        }
        if let Some(v) = self.patch.as_ref() {
            struct_ser.serialize_field("patch", v)?;
        }
        if let Some(v) = self.suffix.as_ref() {
            struct_ser.serialize_field("suffix", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ::pbjson_types::compiler::Version {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "major",
            "minor",
            "patch",
            "suffix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Major,
            Minor,
            Patch,
            Suffix,
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
                            "major" => Ok(GeneratedField::Major),
                            "minor" => Ok(GeneratedField::Minor),
                            "patch" => Ok(GeneratedField::Patch),
                            "suffix" => Ok(GeneratedField::Suffix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ::pbjson_types::compiler::Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.compiler.Version")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<::pbjson_types::compiler::Version, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut major__ = None;
                let mut minor__ = None;
                let mut patch__ = None;
                let mut suffix__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Major => {
                            if major__.is_some() {
                                return Err(serde::de::Error::duplicate_field("major"));
                            }
                            major__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Minor => {
                            if minor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minor"));
                            }
                            minor__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Patch => {
                            if patch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("patch"));
                            }
                            patch__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Suffix => {
                            if suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffix"));
                            }
                            suffix__ = map_.next_value()?;
                        }
                    }
                }
                Ok(::pbjson_types::compiler::Version {
                    major: major__,
                    minor: minor__,
                    patch: patch__,
                    suffix: suffix__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.compiler.Version", FIELDS, GeneratedVisitor)
    }
}
