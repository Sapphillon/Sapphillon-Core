// @generated
impl serde::Serialize for Decl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.doc.is_empty() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Decl", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.doc.is_empty() {
            struct_ser.serialize_field("doc", &self.doc)?;
        }
        if let Some(v) = self.kind.as_ref() {
            match v {
                decl::Kind::Ident(v) => {
                    struct_ser.serialize_field("ident", v)?;
                }
                decl::Kind::Function(v) => {
                    struct_ser.serialize_field("function", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "doc",
            "ident",
            "function",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Doc,
            Ident,
            Function,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "doc" => Ok(GeneratedField::Doc),
                            "ident" => Ok(GeneratedField::Ident),
                            "function" => Ok(GeneratedField::Function),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Decl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Decl")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Decl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut doc__ = None;
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Doc => {
                            if doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            doc__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ident => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ident"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(decl::Kind::Ident)
;
                        }
                        GeneratedField::Function => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(decl::Kind::Function)
;
                        }
                    }
                }
                Ok(Decl {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    doc: doc__.unwrap_or_default(),
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Decl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeclType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.type_params.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.DeclType", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.type_params.is_empty() {
            struct_ser.serialize_field("typeParams", &self.type_params)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeclType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "type",
            "type_params",
            "typeParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Type,
            TypeParams,
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
                            "id" => Ok(GeneratedField::Id),
                            "type" => Ok(GeneratedField::Type),
                            "typeParams" | "type_params" => Ok(GeneratedField::TypeParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeclType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.DeclType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeclType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut r#type__ = None;
                let mut type_params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeParams => {
                            if type_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeParams"));
                            }
                            type_params__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeclType {
                    id: id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    type_params: type_params__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.DeclType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.EnumValue", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if self.value != 0 {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Value,
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
                            "type" => Ok(GeneratedField::Type),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.EnumValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnumValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EnumValue {
                    r#type: r#type__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.EnumValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.errors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.ErrorSet", len)?;
        if !self.errors.is_empty() {
            struct_ser.serialize_field("errors", &self.errors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "errors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Errors,
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
                            "errors" => Ok(GeneratedField::Errors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.ErrorSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut errors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Errors => {
                            if errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errors"));
                            }
                            errors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ErrorSet {
                    errors: errors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.ErrorSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvalState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.EvalState", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvalState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
            Results,
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
                            "values" => Ok(GeneratedField::Values),
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvalState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.EvalState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvalState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                let mut results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EvalState {
                    values: values__.unwrap_or_default(),
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.EvalState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for eval_state::Result {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expr.is_some() {
            len += 1;
        }
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.EvalState.Result", len)?;
        if let Some(v) = self.expr.as_ref() {
            struct_ser.serialize_field("expr", v)?;
        }
        if self.value != 0 {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for eval_state::Result {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expr",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expr,
            Value,
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
                            "expr" => Ok(GeneratedField::Expr),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = eval_state::Result;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.EvalState.Result")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<eval_state::Result, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expr__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expr => {
                            if expr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expr"));
                            }
                            expr__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(eval_state::Result {
                    expr: expr__,
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.EvalState.Result", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Expr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.expr_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Expr", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.expr_kind.as_ref() {
            match v {
                expr::ExprKind::LiteralExpr(v) => {
                    struct_ser.serialize_field("literalExpr", v)?;
                }
                expr::ExprKind::IdentExpr(v) => {
                    struct_ser.serialize_field("identExpr", v)?;
                }
                expr::ExprKind::SelectExpr(v) => {
                    struct_ser.serialize_field("selectExpr", v)?;
                }
                expr::ExprKind::CallExpr(v) => {
                    struct_ser.serialize_field("callExpr", v)?;
                }
                expr::ExprKind::ListExpr(v) => {
                    struct_ser.serialize_field("listExpr", v)?;
                }
                expr::ExprKind::StructExpr(v) => {
                    struct_ser.serialize_field("structExpr", v)?;
                }
                expr::ExprKind::ComprehensionExpr(v) => {
                    struct_ser.serialize_field("comprehensionExpr", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Expr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "literal_expr",
            "literalExpr",
            "ident_expr",
            "identExpr",
            "select_expr",
            "selectExpr",
            "call_expr",
            "callExpr",
            "list_expr",
            "listExpr",
            "struct_expr",
            "structExpr",
            "comprehension_expr",
            "comprehensionExpr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            LiteralExpr,
            IdentExpr,
            SelectExpr,
            CallExpr,
            ListExpr,
            StructExpr,
            ComprehensionExpr,
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
                            "id" => Ok(GeneratedField::Id),
                            "literalExpr" | "literal_expr" => Ok(GeneratedField::LiteralExpr),
                            "identExpr" | "ident_expr" => Ok(GeneratedField::IdentExpr),
                            "selectExpr" | "select_expr" => Ok(GeneratedField::SelectExpr),
                            "callExpr" | "call_expr" => Ok(GeneratedField::CallExpr),
                            "listExpr" | "list_expr" => Ok(GeneratedField::ListExpr),
                            "structExpr" | "struct_expr" => Ok(GeneratedField::StructExpr),
                            "comprehensionExpr" | "comprehension_expr" => Ok(GeneratedField::ComprehensionExpr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Expr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Expr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Expr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut expr_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LiteralExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("literalExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::LiteralExpr)
;
                        }
                        GeneratedField::IdentExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::IdentExpr)
;
                        }
                        GeneratedField::SelectExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::SelectExpr)
;
                        }
                        GeneratedField::CallExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::CallExpr)
;
                        }
                        GeneratedField::ListExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::ListExpr)
;
                        }
                        GeneratedField::StructExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::StructExpr)
;
                        }
                        GeneratedField::ComprehensionExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comprehensionExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::ComprehensionExpr)
;
                        }
                    }
                }
                Ok(Expr {
                    id: id__.unwrap_or_default(),
                    expr_kind: expr_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Expr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::Call {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.target.is_some() {
            len += 1;
        }
        if !self.function.is_empty() {
            len += 1;
        }
        if !self.args.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Expr.Call", len)?;
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        if !self.function.is_empty() {
            struct_ser.serialize_field("function", &self.function)?;
        }
        if !self.args.is_empty() {
            struct_ser.serialize_field("args", &self.args)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::Call {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target",
            "function",
            "args",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Target,
            Function,
            Args,
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
                            "target" => Ok(GeneratedField::Target),
                            "function" => Ok(GeneratedField::Function),
                            "args" => Ok(GeneratedField::Args),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::Call;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Expr.Call")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::Call, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target__ = None;
                let mut function__ = None;
                let mut args__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                        GeneratedField::Function => {
                            if function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            function__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Args => {
                            if args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("args"));
                            }
                            args__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::Call {
                    target: target__,
                    function: function__.unwrap_or_default(),
                    args: args__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Expr.Call", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::Comprehension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.iter_var.is_empty() {
            len += 1;
        }
        if self.iter_range.is_some() {
            len += 1;
        }
        if !self.accu_var.is_empty() {
            len += 1;
        }
        if self.accu_init.is_some() {
            len += 1;
        }
        if self.loop_condition.is_some() {
            len += 1;
        }
        if self.loop_step.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Expr.Comprehension", len)?;
        if !self.iter_var.is_empty() {
            struct_ser.serialize_field("iterVar", &self.iter_var)?;
        }
        if let Some(v) = self.iter_range.as_ref() {
            struct_ser.serialize_field("iterRange", v)?;
        }
        if !self.accu_var.is_empty() {
            struct_ser.serialize_field("accuVar", &self.accu_var)?;
        }
        if let Some(v) = self.accu_init.as_ref() {
            struct_ser.serialize_field("accuInit", v)?;
        }
        if let Some(v) = self.loop_condition.as_ref() {
            struct_ser.serialize_field("loopCondition", v)?;
        }
        if let Some(v) = self.loop_step.as_ref() {
            struct_ser.serialize_field("loopStep", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::Comprehension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "iter_var",
            "iterVar",
            "iter_range",
            "iterRange",
            "accu_var",
            "accuVar",
            "accu_init",
            "accuInit",
            "loop_condition",
            "loopCondition",
            "loop_step",
            "loopStep",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IterVar,
            IterRange,
            AccuVar,
            AccuInit,
            LoopCondition,
            LoopStep,
            Result,
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
                            "iterVar" | "iter_var" => Ok(GeneratedField::IterVar),
                            "iterRange" | "iter_range" => Ok(GeneratedField::IterRange),
                            "accuVar" | "accu_var" => Ok(GeneratedField::AccuVar),
                            "accuInit" | "accu_init" => Ok(GeneratedField::AccuInit),
                            "loopCondition" | "loop_condition" => Ok(GeneratedField::LoopCondition),
                            "loopStep" | "loop_step" => Ok(GeneratedField::LoopStep),
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::Comprehension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Expr.Comprehension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::Comprehension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut iter_var__ = None;
                let mut iter_range__ = None;
                let mut accu_var__ = None;
                let mut accu_init__ = None;
                let mut loop_condition__ = None;
                let mut loop_step__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IterVar => {
                            if iter_var__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iterVar"));
                            }
                            iter_var__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IterRange => {
                            if iter_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iterRange"));
                            }
                            iter_range__ = map_.next_value()?;
                        }
                        GeneratedField::AccuVar => {
                            if accu_var__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accuVar"));
                            }
                            accu_var__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccuInit => {
                            if accu_init__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accuInit"));
                            }
                            accu_init__ = map_.next_value()?;
                        }
                        GeneratedField::LoopCondition => {
                            if loop_condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopCondition"));
                            }
                            loop_condition__ = map_.next_value()?;
                        }
                        GeneratedField::LoopStep => {
                            if loop_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopStep"));
                            }
                            loop_step__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(expr::Comprehension {
                    iter_var: iter_var__.unwrap_or_default(),
                    iter_range: iter_range__,
                    accu_var: accu_var__.unwrap_or_default(),
                    accu_init: accu_init__,
                    loop_condition: loop_condition__,
                    loop_step: loop_step__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Expr.Comprehension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::CreateList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.elements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Expr.CreateList", len)?;
        if !self.elements.is_empty() {
            struct_ser.serialize_field("elements", &self.elements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::CreateList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "elements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Elements,
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
                            "elements" => Ok(GeneratedField::Elements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::CreateList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Expr.CreateList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::CreateList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut elements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Elements => {
                            if elements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elements"));
                            }
                            elements__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::CreateList {
                    elements: elements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Expr.CreateList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::CreateStruct {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Expr.CreateStruct", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::CreateStruct {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "entries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Entries,
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
                            "type" => Ok(GeneratedField::Type),
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::CreateStruct;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Expr.CreateStruct")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::CreateStruct, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::CreateStruct {
                    r#type: r#type__.unwrap_or_default(),
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Expr.CreateStruct", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::create_struct::Entry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.key_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Expr.CreateStruct.Entry", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.key_kind.as_ref() {
            match v {
                expr::create_struct::entry::KeyKind::FieldKey(v) => {
                    struct_ser.serialize_field("fieldKey", v)?;
                }
                expr::create_struct::entry::KeyKind::MapKey(v) => {
                    struct_ser.serialize_field("mapKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::create_struct::Entry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "value",
            "field_key",
            "fieldKey",
            "map_key",
            "mapKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Value,
            FieldKey,
            MapKey,
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
                            "id" => Ok(GeneratedField::Id),
                            "value" => Ok(GeneratedField::Value),
                            "fieldKey" | "field_key" => Ok(GeneratedField::FieldKey),
                            "mapKey" | "map_key" => Ok(GeneratedField::MapKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::create_struct::Entry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Expr.CreateStruct.Entry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::create_struct::Entry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut value__ = None;
                let mut key_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::FieldKey => {
                            if key_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldKey"));
                            }
                            key_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::create_struct::entry::KeyKind::FieldKey);
                        }
                        GeneratedField::MapKey => {
                            if key_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mapKey"));
                            }
                            key_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::create_struct::entry::KeyKind::MapKey)
;
                        }
                    }
                }
                Ok(expr::create_struct::Entry {
                    id: id__.unwrap_or_default(),
                    value: value__,
                    key_kind: key_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Expr.CreateStruct.Entry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::Ident {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Expr.Ident", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::Ident {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::Ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Expr.Ident")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::Ident, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::Ident {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Expr.Ident", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::Select {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operand.is_some() {
            len += 1;
        }
        if !self.field.is_empty() {
            len += 1;
        }
        if self.test_only {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Expr.Select", len)?;
        if let Some(v) = self.operand.as_ref() {
            struct_ser.serialize_field("operand", v)?;
        }
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if self.test_only {
            struct_ser.serialize_field("testOnly", &self.test_only)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::Select {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operand",
            "field",
            "test_only",
            "testOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operand,
            Field,
            TestOnly,
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
                            "operand" => Ok(GeneratedField::Operand),
                            "field" => Ok(GeneratedField::Field),
                            "testOnly" | "test_only" => Ok(GeneratedField::TestOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::Select;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Expr.Select")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::Select, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operand__ = None;
                let mut field__ = None;
                let mut test_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Operand => {
                            if operand__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operand"));
                            }
                            operand__ = map_.next_value()?;
                        }
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestOnly => {
                            if test_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testOnly"));
                            }
                            test_only__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::Select {
                    operand: operand__,
                    field: field__.unwrap_or_default(),
                    test_only: test_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Expr.Select", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExprValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.ExprValue", len)?;
        if let Some(v) = self.kind.as_ref() {
            match v {
                expr_value::Kind::Value(v) => {
                    struct_ser.serialize_field("value", v)?;
                }
                expr_value::Kind::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                expr_value::Kind::Unknown(v) => {
                    struct_ser.serialize_field("unknown", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExprValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "error",
            "unknown",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Error,
            Unknown,
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
                            "value" => Ok(GeneratedField::Value),
                            "error" => Ok(GeneratedField::Error),
                            "unknown" => Ok(GeneratedField::Unknown),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExprValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.ExprValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExprValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr_value::Kind::Value)
;
                        }
                        GeneratedField::Error => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr_value::Kind::Error)
;
                        }
                        GeneratedField::Unknown => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unknown"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr_value::Kind::Unknown)
;
                        }
                    }
                }
                Ok(ExprValue {
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.ExprValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionDecl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.args.is_empty() {
            len += 1;
        }
        if self.return_type.is_some() {
            len += 1;
        }
        if self.receiver_function {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.FunctionDecl", len)?;
        if !self.args.is_empty() {
            struct_ser.serialize_field("args", &self.args)?;
        }
        if let Some(v) = self.return_type.as_ref() {
            struct_ser.serialize_field("returnType", v)?;
        }
        if self.receiver_function {
            struct_ser.serialize_field("receiverFunction", &self.receiver_function)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionDecl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "args",
            "return_type",
            "returnType",
            "receiver_function",
            "receiverFunction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Args,
            ReturnType,
            ReceiverFunction,
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
                            "args" => Ok(GeneratedField::Args),
                            "returnType" | "return_type" => Ok(GeneratedField::ReturnType),
                            "receiverFunction" | "receiver_function" => Ok(GeneratedField::ReceiverFunction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionDecl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.FunctionDecl")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionDecl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut args__ = None;
                let mut return_type__ = None;
                let mut receiver_function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Args => {
                            if args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("args"));
                            }
                            args__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReturnType => {
                            if return_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnType"));
                            }
                            return_type__ = map_.next_value()?;
                        }
                        GeneratedField::ReceiverFunction => {
                            if receiver_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiverFunction"));
                            }
                            receiver_function__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionDecl {
                    args: args__.unwrap_or_default(),
                    return_type: return_type__,
                    receiver_function: receiver_function__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.FunctionDecl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.IdRef", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.IdRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(IdRef {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.IdRef", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdentDecl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.IdentDecl", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdentDecl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Value,
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
                            "type" => Ok(GeneratedField::Type),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdentDecl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.IdentDecl")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdentDecl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IdentDecl {
                    r#type: r#type__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.IdentDecl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.ListValue", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.ListValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListValue {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.ListValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Literal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.constant_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Literal", len)?;
        if let Some(v) = self.constant_kind.as_ref() {
            match v {
                literal::ConstantKind::NullValue(v) => {
                    let v = super::super::super::protobuf::NullValue::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("nullValue", &v)?;
                }
                literal::ConstantKind::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
                literal::ConstantKind::Int64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("int64Value", ToString::to_string(&v).as_str())?;
                }
                literal::ConstantKind::Uint64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("uint64Value", ToString::to_string(&v).as_str())?;
                }
                literal::ConstantKind::DoubleValue(v) => {
                    struct_ser.serialize_field("doubleValue", v)?;
                }
                literal::ConstantKind::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                literal::ConstantKind::BytesValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("bytesValue", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Literal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "null_value",
            "nullValue",
            "bool_value",
            "boolValue",
            "int64_value",
            "int64Value",
            "uint64_value",
            "uint64Value",
            "double_value",
            "doubleValue",
            "string_value",
            "stringValue",
            "bytes_value",
            "bytesValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NullValue,
            BoolValue,
            Int64Value,
            Uint64Value,
            DoubleValue,
            StringValue,
            BytesValue,
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
                            "nullValue" | "null_value" => Ok(GeneratedField::NullValue),
                            "boolValue" | "bool_value" => Ok(GeneratedField::BoolValue),
                            "int64Value" | "int64_value" => Ok(GeneratedField::Int64Value),
                            "uint64Value" | "uint64_value" => Ok(GeneratedField::Uint64Value),
                            "doubleValue" | "double_value" => Ok(GeneratedField::DoubleValue),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "bytesValue" | "bytes_value" => Ok(GeneratedField::BytesValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Literal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Literal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Literal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut constant_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NullValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<super::super::super::protobuf::NullValue>>()?.map(|x| literal::ConstantKind::NullValue(x as i32));
                        }
                        GeneratedField::BoolValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(literal::ConstantKind::BoolValue);
                        }
                        GeneratedField::Int64Value => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Value"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| literal::ConstantKind::Int64Value(x.0));
                        }
                        GeneratedField::Uint64Value => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint64Value"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| literal::ConstantKind::Uint64Value(x.0));
                        }
                        GeneratedField::DoubleValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| literal::ConstantKind::DoubleValue(x.0));
                        }
                        GeneratedField::StringValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(literal::ConstantKind::StringValue);
                        }
                        GeneratedField::BytesValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| literal::ConstantKind::BytesValue(x.0));
                        }
                    }
                }
                Ok(Literal {
                    constant_kind: constant_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Literal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MapValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.MapValue", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MapValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
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
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MapValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.MapValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MapValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MapValue {
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.MapValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for map_value::Entry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.MapValue.Entry", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for map_value::Entry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = map_value::Entry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.MapValue.Entry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<map_value::Entry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(map_value::Entry {
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.MapValue.Entry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParsedExpr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expr.is_some() {
            len += 1;
        }
        if self.source_info.is_some() {
            len += 1;
        }
        if !self.syntax_version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.ParsedExpr", len)?;
        if let Some(v) = self.expr.as_ref() {
            struct_ser.serialize_field("expr", v)?;
        }
        if let Some(v) = self.source_info.as_ref() {
            struct_ser.serialize_field("sourceInfo", v)?;
        }
        if !self.syntax_version.is_empty() {
            struct_ser.serialize_field("syntaxVersion", &self.syntax_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParsedExpr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expr",
            "source_info",
            "sourceInfo",
            "syntax_version",
            "syntaxVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expr,
            SourceInfo,
            SyntaxVersion,
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
                            "expr" => Ok(GeneratedField::Expr),
                            "sourceInfo" | "source_info" => Ok(GeneratedField::SourceInfo),
                            "syntaxVersion" | "syntax_version" => Ok(GeneratedField::SyntaxVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParsedExpr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.ParsedExpr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParsedExpr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expr__ = None;
                let mut source_info__ = None;
                let mut syntax_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expr => {
                            if expr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expr"));
                            }
                            expr__ = map_.next_value()?;
                        }
                        GeneratedField::SourceInfo => {
                            if source_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceInfo"));
                            }
                            source_info__ = map_.next_value()?;
                        }
                        GeneratedField::SyntaxVersion => {
                            if syntax_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syntaxVersion"));
                            }
                            syntax_version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ParsedExpr {
                    expr: expr__,
                    source_info: source_info__,
                    syntax_version: syntax_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.ParsedExpr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.location.is_empty() {
            len += 1;
        }
        if !self.line_offsets.is_empty() {
            len += 1;
        }
        if !self.positions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.SourceInfo", len)?;
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        if !self.line_offsets.is_empty() {
            struct_ser.serialize_field("lineOffsets", &self.line_offsets)?;
        }
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "location",
            "line_offsets",
            "lineOffsets",
            "positions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Location,
            LineOffsets,
            Positions,
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
                            "location" => Ok(GeneratedField::Location),
                            "lineOffsets" | "line_offsets" => Ok(GeneratedField::LineOffsets),
                            "positions" => Ok(GeneratedField::Positions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.SourceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut location__ = None;
                let mut line_offsets__ = None;
                let mut positions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LineOffsets => {
                            if line_offsets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineOffsets"));
                            }
                            line_offsets__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<i32>, ::pbjson::private::NumberDeserialize<i32>>>()?
                                    .into_iter().map(|(k,v)| (k.0, v.0)).collect()
                            );
                        }
                    }
                }
                Ok(SourceInfo {
                    location: location__.unwrap_or_default(),
                    line_offsets: line_offsets__.unwrap_or_default(),
                    positions: positions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.SourceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourcePosition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.location.is_empty() {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if self.line != 0 {
            len += 1;
        }
        if self.column != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.SourcePosition", len)?;
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        if self.line != 0 {
            struct_ser.serialize_field("line", &self.line)?;
        }
        if self.column != 0 {
            struct_ser.serialize_field("column", &self.column)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourcePosition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "location",
            "offset",
            "line",
            "column",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Location,
            Offset,
            Line,
            Column,
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
                            "location" => Ok(GeneratedField::Location),
                            "offset" => Ok(GeneratedField::Offset),
                            "line" => Ok(GeneratedField::Line),
                            "column" => Ok(GeneratedField::Column),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourcePosition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.SourcePosition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourcePosition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut location__ = None;
                let mut offset__ = None;
                let mut line__ = None;
                let mut column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Line => {
                            if line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SourcePosition {
                    location: location__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                    line: line__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.SourcePosition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnknownSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exprs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.UnknownSet", len)?;
        if !self.exprs.is_empty() {
            struct_ser.serialize_field("exprs", &self.exprs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnknownSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exprs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exprs,
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
                            "exprs" => Ok(GeneratedField::Exprs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnknownSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.UnknownSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnknownSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exprs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Exprs => {
                            if exprs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exprs"));
                            }
                            exprs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnknownSet {
                    exprs: exprs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.UnknownSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Value {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1beta1.Value", len)?;
        if let Some(v) = self.kind.as_ref() {
            match v {
                value::Kind::NullValue(v) => {
                    let v = super::super::super::protobuf::NullValue::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("nullValue", &v)?;
                }
                value::Kind::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
                value::Kind::Int64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("int64Value", ToString::to_string(&v).as_str())?;
                }
                value::Kind::Uint64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("uint64Value", ToString::to_string(&v).as_str())?;
                }
                value::Kind::DoubleValue(v) => {
                    struct_ser.serialize_field("doubleValue", v)?;
                }
                value::Kind::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                value::Kind::BytesValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("bytesValue", pbjson::private::base64::encode(&v).as_str())?;
                }
                value::Kind::EnumValue(v) => {
                    struct_ser.serialize_field("enumValue", v)?;
                }
                value::Kind::ObjectValue(v) => {
                    struct_ser.serialize_field("objectValue", v)?;
                }
                value::Kind::MapValue(v) => {
                    struct_ser.serialize_field("mapValue", v)?;
                }
                value::Kind::ListValue(v) => {
                    struct_ser.serialize_field("listValue", v)?;
                }
                value::Kind::TypeValue(v) => {
                    struct_ser.serialize_field("typeValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Value {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "null_value",
            "nullValue",
            "bool_value",
            "boolValue",
            "int64_value",
            "int64Value",
            "uint64_value",
            "uint64Value",
            "double_value",
            "doubleValue",
            "string_value",
            "stringValue",
            "bytes_value",
            "bytesValue",
            "enum_value",
            "enumValue",
            "object_value",
            "objectValue",
            "map_value",
            "mapValue",
            "list_value",
            "listValue",
            "type_value",
            "typeValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NullValue,
            BoolValue,
            Int64Value,
            Uint64Value,
            DoubleValue,
            StringValue,
            BytesValue,
            EnumValue,
            ObjectValue,
            MapValue,
            ListValue,
            TypeValue,
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
                            "nullValue" | "null_value" => Ok(GeneratedField::NullValue),
                            "boolValue" | "bool_value" => Ok(GeneratedField::BoolValue),
                            "int64Value" | "int64_value" => Ok(GeneratedField::Int64Value),
                            "uint64Value" | "uint64_value" => Ok(GeneratedField::Uint64Value),
                            "doubleValue" | "double_value" => Ok(GeneratedField::DoubleValue),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "bytesValue" | "bytes_value" => Ok(GeneratedField::BytesValue),
                            "enumValue" | "enum_value" => Ok(GeneratedField::EnumValue),
                            "objectValue" | "object_value" => Ok(GeneratedField::ObjectValue),
                            "mapValue" | "map_value" => Ok(GeneratedField::MapValue),
                            "listValue" | "list_value" => Ok(GeneratedField::ListValue),
                            "typeValue" | "type_value" => Ok(GeneratedField::TypeValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1beta1.Value")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Value, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NullValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<super::super::super::protobuf::NullValue>>()?.map(|x| value::Kind::NullValue(x as i32));
                        }
                        GeneratedField::BoolValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::BoolValue);
                        }
                        GeneratedField::Int64Value => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Value"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| value::Kind::Int64Value(x.0));
                        }
                        GeneratedField::Uint64Value => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint64Value"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| value::Kind::Uint64Value(x.0));
                        }
                        GeneratedField::DoubleValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| value::Kind::DoubleValue(x.0));
                        }
                        GeneratedField::StringValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::StringValue);
                        }
                        GeneratedField::BytesValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| value::Kind::BytesValue(x.0));
                        }
                        GeneratedField::EnumValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::EnumValue)
;
                        }
                        GeneratedField::ObjectValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::ObjectValue)
;
                        }
                        GeneratedField::MapValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mapValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::MapValue)
;
                        }
                        GeneratedField::ListValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::ListValue)
;
                        }
                        GeneratedField::TypeValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::TypeValue);
                        }
                    }
                }
                Ok(Value {
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1beta1.Value", FIELDS, GeneratedVisitor)
    }
}
