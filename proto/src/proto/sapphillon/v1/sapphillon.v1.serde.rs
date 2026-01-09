// @generated
impl serde::Serialize for AllowedPermission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.plugin_function_id.is_empty() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.AllowedPermission", len)?;
        if !self.plugin_function_id.is_empty() {
            struct_ser.serialize_field("pluginFunctionId", &self.plugin_function_id)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowedPermission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plugin_function_id",
            "pluginFunctionId",
            "permissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PluginFunctionId,
            Permissions,
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
                            "pluginFunctionId" | "plugin_function_id" => Ok(GeneratedField::PluginFunctionId),
                            "permissions" => Ok(GeneratedField::Permissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowedPermission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.AllowedPermission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AllowedPermission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plugin_function_id__ = None;
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PluginFunctionId => {
                            if plugin_function_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pluginFunctionId"));
                            }
                            plugin_function_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AllowedPermission {
                    plugin_function_id: plugin_function_id__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.AllowedPermission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteWorkflowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workflow_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.DeleteWorkflowRequest", len)?;
        if !self.workflow_id.is_empty() {
            struct_ser.serialize_field("workflowId", &self.workflow_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteWorkflowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow_id",
            "workflowId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkflowId,
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
                            "workflowId" | "workflow_id" => Ok(GeneratedField::WorkflowId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteWorkflowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.DeleteWorkflowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteWorkflowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkflowId => {
                            if workflow_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowId"));
                            }
                            workflow_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteWorkflowRequest {
                    workflow_id: workflow_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.DeleteWorkflowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteWorkflowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sapphillon.v1.DeleteWorkflowResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteWorkflowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteWorkflowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.DeleteWorkflowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteWorkflowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteWorkflowResponse {
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.DeleteWorkflowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FixWorkflowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workflow_definition.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.FixWorkflowRequest", len)?;
        if !self.workflow_definition.is_empty() {
            struct_ser.serialize_field("workflowDefinition", &self.workflow_definition)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FixWorkflowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow_definition",
            "workflowDefinition",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkflowDefinition,
            Description,
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
                            "workflowDefinition" | "workflow_definition" => Ok(GeneratedField::WorkflowDefinition),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FixWorkflowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.FixWorkflowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FixWorkflowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow_definition__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkflowDefinition => {
                            if workflow_definition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowDefinition"));
                            }
                            workflow_definition__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FixWorkflowRequest {
                    workflow_definition: workflow_definition__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.FixWorkflowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FixWorkflowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fixed_workflow_definition.is_some() {
            len += 1;
        }
        if !self.change_summary.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.FixWorkflowResponse", len)?;
        if let Some(v) = self.fixed_workflow_definition.as_ref() {
            struct_ser.serialize_field("fixedWorkflowDefinition", v)?;
        }
        if !self.change_summary.is_empty() {
            struct_ser.serialize_field("changeSummary", &self.change_summary)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FixWorkflowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fixed_workflow_definition",
            "fixedWorkflowDefinition",
            "change_summary",
            "changeSummary",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FixedWorkflowDefinition,
            ChangeSummary,
            Status,
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
                            "fixedWorkflowDefinition" | "fixed_workflow_definition" => Ok(GeneratedField::FixedWorkflowDefinition),
                            "changeSummary" | "change_summary" => Ok(GeneratedField::ChangeSummary),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FixWorkflowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.FixWorkflowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FixWorkflowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fixed_workflow_definition__ = None;
                let mut change_summary__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FixedWorkflowDefinition => {
                            if fixed_workflow_definition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedWorkflowDefinition"));
                            }
                            fixed_workflow_definition__ = map_.next_value()?;
                        }
                        GeneratedField::ChangeSummary => {
                            if change_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeSummary"));
                            }
                            change_summary__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FixWorkflowResponse {
                    fixed_workflow_definition: fixed_workflow_definition__,
                    change_summary: change_summary__.unwrap_or_default(),
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.FixWorkflowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionDefine {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parameters.is_empty() {
            len += 1;
        }
        if !self.returns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.FunctionDefine", len)?;
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if !self.returns.is_empty() {
            struct_ser.serialize_field("returns", &self.returns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionDefine {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parameters",
            "returns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parameters,
            Returns,
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
                            "parameters" => Ok(GeneratedField::Parameters),
                            "returns" => Ok(GeneratedField::Returns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionDefine;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.FunctionDefine")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionDefine, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parameters__ = None;
                let mut returns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Returns => {
                            if returns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returns"));
                            }
                            returns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionDefine {
                    parameters: parameters__.unwrap_or_default(),
                    returns: returns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.FunctionDefine", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionParameter {
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
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.FunctionParameter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            Description,
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
                            "type" => Ok(GeneratedField::Type),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.FunctionParameter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionParameter {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.FunctionParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateWorkflowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prompt.is_empty() {
            len += 1;
        }
        if !self.model_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.GenerateWorkflowRequest", len)?;
        if !self.prompt.is_empty() {
            struct_ser.serialize_field("prompt", &self.prompt)?;
        }
        if !self.model_name.is_empty() {
            struct_ser.serialize_field("modelName", &self.model_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateWorkflowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prompt",
            "model_name",
            "modelName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prompt,
            ModelName,
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
                            "prompt" => Ok(GeneratedField::Prompt),
                            "modelName" | "model_name" => Ok(GeneratedField::ModelName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateWorkflowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.GenerateWorkflowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateWorkflowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prompt__ = None;
                let mut model_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Prompt => {
                            if prompt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prompt"));
                            }
                            prompt__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelName => {
                            if model_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelName"));
                            }
                            model_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenerateWorkflowRequest {
                    prompt: prompt__.unwrap_or_default(),
                    model_name: model_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.GenerateWorkflowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateWorkflowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.workflow_definition.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.GenerateWorkflowResponse", len)?;
        if let Some(v) = self.workflow_definition.as_ref() {
            struct_ser.serialize_field("workflowDefinition", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateWorkflowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow_definition",
            "workflowDefinition",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkflowDefinition,
            Status,
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
                            "workflowDefinition" | "workflow_definition" => Ok(GeneratedField::WorkflowDefinition),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateWorkflowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.GenerateWorkflowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateWorkflowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow_definition__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkflowDefinition => {
                            if workflow_definition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowDefinition"));
                            }
                            workflow_definition__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GenerateWorkflowResponse {
                    workflow_definition: workflow_definition__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.GenerateWorkflowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sapphillon.v1.GetVersionRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.GetVersionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetVersionRequest {
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.GetVersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.GetVersionResponse", len)?;
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
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
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.GetVersionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetVersionResponse {
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.GetVersionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWorkflowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workflow_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.GetWorkflowRequest", len)?;
        if !self.workflow_id.is_empty() {
            struct_ser.serialize_field("workflowId", &self.workflow_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWorkflowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow_id",
            "workflowId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkflowId,
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
                            "workflowId" | "workflow_id" => Ok(GeneratedField::WorkflowId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWorkflowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.GetWorkflowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWorkflowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkflowId => {
                            if workflow_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowId"));
                            }
                            workflow_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetWorkflowRequest {
                    workflow_id: workflow_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.GetWorkflowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWorkflowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.workflow.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.GetWorkflowResponse", len)?;
        if let Some(v) = self.workflow.as_ref() {
            struct_ser.serialize_field("workflow", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWorkflowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Workflow,
            Status,
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
                            "workflow" => Ok(GeneratedField::Workflow),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWorkflowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.GetWorkflowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWorkflowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Workflow => {
                            if workflow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflow"));
                            }
                            workflow__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWorkflowResponse {
                    workflow: workflow__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.GetWorkflowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstallPluginRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.InstallPluginRequest", len)?;
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstallPluginRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
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
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstallPluginRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.InstallPluginRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InstallPluginRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uri__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InstallPluginRequest {
                    uri: uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.InstallPluginRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstallPluginResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.plugin.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.InstallPluginResponse", len)?;
        if let Some(v) = self.plugin.as_ref() {
            struct_ser.serialize_field("plugin", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstallPluginResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plugin",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Plugin,
            Status,
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
                            "plugin" => Ok(GeneratedField::Plugin),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstallPluginResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.InstallPluginResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InstallPluginResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plugin__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Plugin => {
                            if plugin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plugin"));
                            }
                            plugin__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InstallPluginResponse {
                    plugin: plugin__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.InstallPluginResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPluginsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.ListPluginsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPluginsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
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
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPluginsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.ListPluginsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPluginsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListPluginsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.ListPluginsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPluginsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.plugins.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.ListPluginsResponse", len)?;
        if !self.plugins.is_empty() {
            struct_ser.serialize_field("plugins", &self.plugins)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPluginsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plugins",
            "next_page_token",
            "nextPageToken",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Plugins,
            NextPageToken,
            Status,
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
                            "plugins" => Ok(GeneratedField::Plugins),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPluginsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.ListPluginsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPluginsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plugins__ = None;
                let mut next_page_token__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Plugins => {
                            if plugins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plugins"));
                            }
                            plugins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPluginsResponse {
                    plugins: plugins__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.ListPluginsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWorkflowsFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.workflow_language != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.ListWorkflowsFilter", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.workflow_language != 0 {
            let v = WorkflowLanguage::try_from(self.workflow_language)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.workflow_language)))?;
            struct_ser.serialize_field("workflowLanguage", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWorkflowsFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_name",
            "displayName",
            "workflow_language",
            "workflowLanguage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayName,
            WorkflowLanguage,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "workflowLanguage" | "workflow_language" => Ok(GeneratedField::WorkflowLanguage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWorkflowsFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.ListWorkflowsFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWorkflowsFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_name__ = None;
                let mut workflow_language__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WorkflowLanguage => {
                            if workflow_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowLanguage"));
                            }
                            workflow_language__ = Some(map_.next_value::<WorkflowLanguage>()? as i32);
                        }
                    }
                }
                Ok(ListWorkflowsFilter {
                    display_name: display_name__.unwrap_or_default(),
                    workflow_language: workflow_language__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.ListWorkflowsFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWorkflowsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.ListWorkflowsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWorkflowsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrderBy,
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
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWorkflowsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.ListWorkflowsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWorkflowsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListWorkflowsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__,
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.ListWorkflowsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWorkflowsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workflows.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.ListWorkflowsResponse", len)?;
        if !self.workflows.is_empty() {
            struct_ser.serialize_field("workflows", &self.workflows)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWorkflowsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflows",
            "next_page_token",
            "nextPageToken",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Workflows,
            NextPageToken,
            Status,
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
                            "workflows" => Ok(GeneratedField::Workflows),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWorkflowsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.ListWorkflowsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWorkflowsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflows__ = None;
                let mut next_page_token__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Workflows => {
                            if workflows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflows"));
                            }
                            workflows__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListWorkflowsResponse {
                    workflows: workflows__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.ListWorkflowsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderByClause {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if self.direction != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.OrderByClause", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if self.direction != 0 {
            let v = OrderByDirection::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderByClause {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Direction,
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
                            "field" => Ok(GeneratedField::Field),
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderByClause;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.OrderByClause")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderByClause, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut direction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<OrderByDirection>()? as i32);
                        }
                    }
                }
                Ok(OrderByClause {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.OrderByClause", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderByDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ORDER_BY_DIRECTION_UNSPECIFIED",
            Self::Asc => "ORDER_BY_DIRECTION_ASC",
            Self::Desc => "ORDER_BY_DIRECTION_DESC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OrderByDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ORDER_BY_DIRECTION_UNSPECIFIED",
            "ORDER_BY_DIRECTION_ASC",
            "ORDER_BY_DIRECTION_DESC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderByDirection;

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
                    "ORDER_BY_DIRECTION_UNSPECIFIED" => Ok(OrderByDirection::Unspecified),
                    "ORDER_BY_DIRECTION_ASC" => Ok(OrderByDirection::Asc),
                    "ORDER_BY_DIRECTION_DESC" => Ok(OrderByDirection::Desc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Permission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.permission_type != 0 {
            len += 1;
        }
        if !self.resource.is_empty() {
            len += 1;
        }
        if self.permission_level != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.Permission", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.permission_type != 0 {
            let v = PermissionType::try_from(self.permission_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.permission_type)))?;
            struct_ser.serialize_field("permissionType", &v)?;
        }
        if !self.resource.is_empty() {
            struct_ser.serialize_field("resource", &self.resource)?;
        }
        if self.permission_level != 0 {
            let v = PermissionLevel::try_from(self.permission_level)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.permission_level)))?;
            struct_ser.serialize_field("permissionLevel", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Permission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_name",
            "displayName",
            "description",
            "permission_type",
            "permissionType",
            "resource",
            "permission_level",
            "permissionLevel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayName,
            Description,
            PermissionType,
            Resource,
            PermissionLevel,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            "permissionType" | "permission_type" => Ok(GeneratedField::PermissionType),
                            "resource" => Ok(GeneratedField::Resource),
                            "permissionLevel" | "permission_level" => Ok(GeneratedField::PermissionLevel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Permission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.Permission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Permission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_name__ = None;
                let mut description__ = None;
                let mut permission_type__ = None;
                let mut resource__ = None;
                let mut permission_level__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PermissionType => {
                            if permission_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionType"));
                            }
                            permission_type__ = Some(map_.next_value::<PermissionType>()? as i32);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PermissionLevel => {
                            if permission_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionLevel"));
                            }
                            permission_level__ = Some(map_.next_value::<PermissionLevel>()? as i32);
                        }
                    }
                }
                Ok(Permission {
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    permission_type: permission_type__.unwrap_or_default(),
                    resource: resource__.unwrap_or_default(),
                    permission_level: permission_level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.Permission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PERMISSION_LEVEL_UNSPECIFIED",
            Self::Medium => "PERMISSION_LEVEL_MEDIUM",
            Self::High => "PERMISSION_LEVEL_HIGH",
            Self::Critical => "PERMISSION_LEVEL_CRITICAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PermissionLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PERMISSION_LEVEL_UNSPECIFIED",
            "PERMISSION_LEVEL_MEDIUM",
            "PERMISSION_LEVEL_HIGH",
            "PERMISSION_LEVEL_CRITICAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionLevel;

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
                    "PERMISSION_LEVEL_UNSPECIFIED" => Ok(PermissionLevel::Unspecified),
                    "PERMISSION_LEVEL_MEDIUM" => Ok(PermissionLevel::Medium),
                    "PERMISSION_LEVEL_HIGH" => Ok(PermissionLevel::High),
                    "PERMISSION_LEVEL_CRITICAL" => Ok(PermissionLevel::Critical),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PERMISSION_TYPE_UNSPECIFIED",
            Self::Execute => "PERMISSION_TYPE_EXECUTE",
            Self::FilesystemRead => "PERMISSION_TYPE_FILESYSTEM_READ",
            Self::FilesystemWrite => "PERMISSION_TYPE_FILESYSTEM_WRITE",
            Self::NetAccess => "PERMISSION_TYPE_NET_ACCESS",
            Self::AllowMcp => "PERMISSION_TYPE_ALLOW_MCP",
            Self::AllowAll => "PERMISSION_TYPE_ALLOW_ALL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PermissionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PERMISSION_TYPE_UNSPECIFIED",
            "PERMISSION_TYPE_EXECUTE",
            "PERMISSION_TYPE_FILESYSTEM_READ",
            "PERMISSION_TYPE_FILESYSTEM_WRITE",
            "PERMISSION_TYPE_NET_ACCESS",
            "PERMISSION_TYPE_ALLOW_MCP",
            "PERMISSION_TYPE_ALLOW_ALL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionType;

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
                    "PERMISSION_TYPE_UNSPECIFIED" => Ok(PermissionType::Unspecified),
                    "PERMISSION_TYPE_EXECUTE" => Ok(PermissionType::Execute),
                    "PERMISSION_TYPE_FILESYSTEM_READ" => Ok(PermissionType::FilesystemRead),
                    "PERMISSION_TYPE_FILESYSTEM_WRITE" => Ok(PermissionType::FilesystemWrite),
                    "PERMISSION_TYPE_NET_ACCESS" => Ok(PermissionType::NetAccess),
                    "PERMISSION_TYPE_ALLOW_MCP" => Ok(PermissionType::AllowMcp),
                    "PERMISSION_TYPE_ALLOW_ALL" => Ok(PermissionType::AllowAll),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PluginFunction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.function_id.is_empty() {
            len += 1;
        }
        if !self.function_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        if self.function_define.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.PluginFunction", len)?;
        if !self.function_id.is_empty() {
            struct_ser.serialize_field("functionId", &self.function_id)?;
        }
        if !self.function_name.is_empty() {
            struct_ser.serialize_field("functionName", &self.function_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if let Some(v) = self.function_define.as_ref() {
            struct_ser.serialize_field("functionDefine", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginFunction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "function_id",
            "functionId",
            "function_name",
            "functionName",
            "description",
            "permissions",
            "function_define",
            "functionDefine",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FunctionId,
            FunctionName,
            Description,
            Permissions,
            FunctionDefine,
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
                            "functionId" | "function_id" => Ok(GeneratedField::FunctionId),
                            "functionName" | "function_name" => Ok(GeneratedField::FunctionName),
                            "description" => Ok(GeneratedField::Description),
                            "permissions" => Ok(GeneratedField::Permissions),
                            "functionDefine" | "function_define" => Ok(GeneratedField::FunctionDefine),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginFunction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.PluginFunction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PluginFunction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut function_id__ = None;
                let mut function_name__ = None;
                let mut description__ = None;
                let mut permissions__ = None;
                let mut function_define__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FunctionId => {
                            if function_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionId"));
                            }
                            function_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FunctionName => {
                            if function_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionName"));
                            }
                            function_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FunctionDefine => {
                            if function_define__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionDefine"));
                            }
                            function_define__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PluginFunction {
                    function_id: function_id__.unwrap_or_default(),
                    function_name: function_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                    function_define: function_define__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.PluginFunction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PluginPackage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.package_id.is_empty() {
            len += 1;
        }
        if !self.package_name.is_empty() {
            len += 1;
        }
        if !self.package_version.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.functions.is_empty() {
            len += 1;
        }
        if !self.plugin_store_url.is_empty() {
            len += 1;
        }
        if self.internal_plugin.is_some() {
            len += 1;
        }
        if self.verified.is_some() {
            len += 1;
        }
        if self.deprecated.is_some() {
            len += 1;
        }
        if self.installed_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.PluginPackage", len)?;
        if !self.package_id.is_empty() {
            struct_ser.serialize_field("packageId", &self.package_id)?;
        }
        if !self.package_name.is_empty() {
            struct_ser.serialize_field("packageName", &self.package_name)?;
        }
        if !self.package_version.is_empty() {
            struct_ser.serialize_field("packageVersion", &self.package_version)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.functions.is_empty() {
            struct_ser.serialize_field("functions", &self.functions)?;
        }
        if !self.plugin_store_url.is_empty() {
            struct_ser.serialize_field("pluginStoreUrl", &self.plugin_store_url)?;
        }
        if let Some(v) = self.internal_plugin.as_ref() {
            struct_ser.serialize_field("internalPlugin", v)?;
        }
        if let Some(v) = self.verified.as_ref() {
            struct_ser.serialize_field("verified", v)?;
        }
        if let Some(v) = self.deprecated.as_ref() {
            struct_ser.serialize_field("deprecated", v)?;
        }
        if let Some(v) = self.installed_at.as_ref() {
            struct_ser.serialize_field("installedAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginPackage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "package_id",
            "packageId",
            "package_name",
            "packageName",
            "package_version",
            "packageVersion",
            "description",
            "functions",
            "plugin_store_url",
            "pluginStoreUrl",
            "internal_plugin",
            "internalPlugin",
            "verified",
            "deprecated",
            "installed_at",
            "installedAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PackageId,
            PackageName,
            PackageVersion,
            Description,
            Functions,
            PluginStoreUrl,
            InternalPlugin,
            Verified,
            Deprecated,
            InstalledAt,
            UpdatedAt,
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
                            "packageId" | "package_id" => Ok(GeneratedField::PackageId),
                            "packageName" | "package_name" => Ok(GeneratedField::PackageName),
                            "packageVersion" | "package_version" => Ok(GeneratedField::PackageVersion),
                            "description" => Ok(GeneratedField::Description),
                            "functions" => Ok(GeneratedField::Functions),
                            "pluginStoreUrl" | "plugin_store_url" => Ok(GeneratedField::PluginStoreUrl),
                            "internalPlugin" | "internal_plugin" => Ok(GeneratedField::InternalPlugin),
                            "verified" => Ok(GeneratedField::Verified),
                            "deprecated" => Ok(GeneratedField::Deprecated),
                            "installedAt" | "installed_at" => Ok(GeneratedField::InstalledAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginPackage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.PluginPackage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PluginPackage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut package_id__ = None;
                let mut package_name__ = None;
                let mut package_version__ = None;
                let mut description__ = None;
                let mut functions__ = None;
                let mut plugin_store_url__ = None;
                let mut internal_plugin__ = None;
                let mut verified__ = None;
                let mut deprecated__ = None;
                let mut installed_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PackageId => {
                            if package_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageId"));
                            }
                            package_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PackageName => {
                            if package_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageName"));
                            }
                            package_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PackageVersion => {
                            if package_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageVersion"));
                            }
                            package_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Functions => {
                            if functions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functions"));
                            }
                            functions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PluginStoreUrl => {
                            if plugin_store_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pluginStoreUrl"));
                            }
                            plugin_store_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InternalPlugin => {
                            if internal_plugin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalPlugin"));
                            }
                            internal_plugin__ = map_.next_value()?;
                        }
                        GeneratedField::Verified => {
                            if verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verified"));
                            }
                            verified__ = map_.next_value()?;
                        }
                        GeneratedField::Deprecated => {
                            if deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecated"));
                            }
                            deprecated__ = map_.next_value()?;
                        }
                        GeneratedField::InstalledAt => {
                            if installed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("installedAt"));
                            }
                            installed_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PluginPackage {
                    package_id: package_id__.unwrap_or_default(),
                    package_name: package_name__.unwrap_or_default(),
                    package_version: package_version__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    functions: functions__.unwrap_or_default(),
                    plugin_store_url: plugin_store_url__.unwrap_or_default(),
                    internal_plugin: internal_plugin__,
                    verified: verified__,
                    deprecated: deprecated__,
                    installed_at: installed_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.PluginPackage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RunWorkflowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.by_id.is_some() {
            len += 1;
        }
        if self.ai_model_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.RunWorkflowRequest", len)?;
        if let Some(v) = self.by_id.as_ref() {
            struct_ser.serialize_field("byId", v)?;
        }
        if let Some(v) = self.ai_model_name.as_ref() {
            struct_ser.serialize_field("aiModelName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RunWorkflowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "by_id",
            "byId",
            "ai_model_name",
            "aiModelName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ById,
            AiModelName,
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
                            "byId" | "by_id" => Ok(GeneratedField::ById),
                            "aiModelName" | "ai_model_name" => Ok(GeneratedField::AiModelName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunWorkflowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.RunWorkflowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RunWorkflowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut by_id__ = None;
                let mut ai_model_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ById => {
                            if by_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("byId"));
                            }
                            by_id__ = map_.next_value()?;
                        }
                        GeneratedField::AiModelName => {
                            if ai_model_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aiModelName"));
                            }
                            ai_model_name__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RunWorkflowRequest {
                    by_id: by_id__,
                    ai_model_name: ai_model_name__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.RunWorkflowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RunWorkflowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.workflow_result.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.RunWorkflowResponse", len)?;
        if let Some(v) = self.workflow_result.as_ref() {
            struct_ser.serialize_field("workflowResult", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RunWorkflowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow_result",
            "workflowResult",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkflowResult,
            Status,
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
                            "workflowResult" | "workflow_result" => Ok(GeneratedField::WorkflowResult),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunWorkflowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.RunWorkflowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RunWorkflowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow_result__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkflowResult => {
                            if workflow_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowResult"));
                            }
                            workflow_result__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RunWorkflowResponse {
                    workflow_result: workflow_result__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.RunWorkflowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UninstallPluginRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.package_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.UninstallPluginRequest", len)?;
        if !self.package_id.is_empty() {
            struct_ser.serialize_field("packageId", &self.package_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UninstallPluginRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "package_id",
            "packageId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PackageId,
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
                            "packageId" | "package_id" => Ok(GeneratedField::PackageId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UninstallPluginRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.UninstallPluginRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UninstallPluginRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut package_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PackageId => {
                            if package_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageId"));
                            }
                            package_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UninstallPluginRequest {
                    package_id: package_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.UninstallPluginRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UninstallPluginResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.UninstallPluginResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UninstallPluginResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
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
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UninstallPluginResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.UninstallPluginResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UninstallPluginResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UninstallPluginResponse {
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.UninstallPluginResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateWorkflowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.workflow.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.UpdateWorkflowRequest", len)?;
        if let Some(v) = self.workflow.as_ref() {
            struct_ser.serialize_field("workflow", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateWorkflowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Workflow,
            UpdateMask,
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
                            "workflow" => Ok(GeneratedField::Workflow),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateWorkflowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.UpdateWorkflowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateWorkflowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Workflow => {
                            if workflow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflow"));
                            }
                            workflow__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateWorkflowRequest {
                    workflow: workflow__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.UpdateWorkflowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateWorkflowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.workflow.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.UpdateWorkflowResponse", len)?;
        if let Some(v) = self.workflow.as_ref() {
            struct_ser.serialize_field("workflow", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateWorkflowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Workflow,
            Status,
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
                            "workflow" => Ok(GeneratedField::Workflow),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateWorkflowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.UpdateWorkflowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateWorkflowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Workflow => {
                            if workflow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflow"));
                            }
                            workflow__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateWorkflowResponse {
                    workflow: workflow__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.UpdateWorkflowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Version {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.Version", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Version {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
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
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.Version")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Version, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Version {
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.Version", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Workflow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.workflow_language != 0 {
            len += 1;
        }
        if !self.workflow_code.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.workflow_results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.Workflow", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.workflow_language != 0 {
            let v = WorkflowLanguage::try_from(self.workflow_language)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.workflow_language)))?;
            struct_ser.serialize_field("workflowLanguage", &v)?;
        }
        if !self.workflow_code.is_empty() {
            struct_ser.serialize_field("workflowCode", &self.workflow_code)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.workflow_results.is_empty() {
            struct_ser.serialize_field("workflowResults", &self.workflow_results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Workflow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "display_name",
            "displayName",
            "description",
            "workflow_language",
            "workflowLanguage",
            "workflow_code",
            "workflowCode",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "workflow_results",
            "workflowResults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            DisplayName,
            Description,
            WorkflowLanguage,
            WorkflowCode,
            CreatedAt,
            UpdatedAt,
            WorkflowResults,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            "workflowLanguage" | "workflow_language" => Ok(GeneratedField::WorkflowLanguage),
                            "workflowCode" | "workflow_code" => Ok(GeneratedField::WorkflowCode),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "workflowResults" | "workflow_results" => Ok(GeneratedField::WorkflowResults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Workflow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.Workflow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Workflow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut workflow_language__ = None;
                let mut workflow_code__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut workflow_results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WorkflowLanguage => {
                            if workflow_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowLanguage"));
                            }
                            workflow_language__ = Some(map_.next_value::<WorkflowLanguage>()? as i32);
                        }
                        GeneratedField::WorkflowCode => {
                            if workflow_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowCode"));
                            }
                            workflow_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::WorkflowResults => {
                            if workflow_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowResults"));
                            }
                            workflow_results__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Workflow {
                    id: id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    workflow_language: workflow_language__.unwrap_or_default(),
                    workflow_code: workflow_code__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    workflow_results: workflow_results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.Workflow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkflowCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.code_revision != 0 {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if self.language != 0 {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if !self.result.is_empty() {
            len += 1;
        }
        if !self.plugin_packages.is_empty() {
            len += 1;
        }
        if !self.plugin_function_ids.is_empty() {
            len += 1;
        }
        if !self.allowed_permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.WorkflowCode", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.code_revision != 0 {
            struct_ser.serialize_field("codeRevision", &self.code_revision)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if self.language != 0 {
            let v = WorkflowLanguage::try_from(self.language)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.language)))?;
            struct_ser.serialize_field("language", &v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        if !self.plugin_packages.is_empty() {
            struct_ser.serialize_field("pluginPackages", &self.plugin_packages)?;
        }
        if !self.plugin_function_ids.is_empty() {
            struct_ser.serialize_field("pluginFunctionIds", &self.plugin_function_ids)?;
        }
        if !self.allowed_permissions.is_empty() {
            struct_ser.serialize_field("allowedPermissions", &self.allowed_permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkflowCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code_revision",
            "codeRevision",
            "code",
            "language",
            "created_at",
            "createdAt",
            "result",
            "plugin_packages",
            "pluginPackages",
            "plugin_function_ids",
            "pluginFunctionIds",
            "allowed_permissions",
            "allowedPermissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CodeRevision,
            Code,
            Language,
            CreatedAt,
            Result,
            PluginPackages,
            PluginFunctionIds,
            AllowedPermissions,
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
                            "codeRevision" | "code_revision" => Ok(GeneratedField::CodeRevision),
                            "code" => Ok(GeneratedField::Code),
                            "language" => Ok(GeneratedField::Language),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "result" => Ok(GeneratedField::Result),
                            "pluginPackages" | "plugin_packages" => Ok(GeneratedField::PluginPackages),
                            "pluginFunctionIds" | "plugin_function_ids" => Ok(GeneratedField::PluginFunctionIds),
                            "allowedPermissions" | "allowed_permissions" => Ok(GeneratedField::AllowedPermissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkflowCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.WorkflowCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkflowCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code_revision__ = None;
                let mut code__ = None;
                let mut language__ = None;
                let mut created_at__ = None;
                let mut result__ = None;
                let mut plugin_packages__ = None;
                let mut plugin_function_ids__ = None;
                let mut allowed_permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeRevision => {
                            if code_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeRevision"));
                            }
                            code_revision__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = Some(map_.next_value::<WorkflowLanguage>()? as i32);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PluginPackages => {
                            if plugin_packages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pluginPackages"));
                            }
                            plugin_packages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PluginFunctionIds => {
                            if plugin_function_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pluginFunctionIds"));
                            }
                            plugin_function_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedPermissions => {
                            if allowed_permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedPermissions"));
                            }
                            allowed_permissions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkflowCode {
                    id: id__.unwrap_or_default(),
                    code_revision: code_revision__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    language: language__.unwrap_or_default(),
                    created_at: created_at__,
                    result: result__.unwrap_or_default(),
                    plugin_packages: plugin_packages__.unwrap_or_default(),
                    plugin_function_ids: plugin_function_ids__.unwrap_or_default(),
                    allowed_permissions: allowed_permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.WorkflowCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkflowLanguage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "WORKFLOW_LANGUAGE_UNSPECIFIED",
            Self::Typescript => "WORKFLOW_LANGUAGE_TYPESCRIPT",
            Self::Javascript => "WORKFLOW_LANGUAGE_JAVASCRIPT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for WorkflowLanguage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "WORKFLOW_LANGUAGE_UNSPECIFIED",
            "WORKFLOW_LANGUAGE_TYPESCRIPT",
            "WORKFLOW_LANGUAGE_JAVASCRIPT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkflowLanguage;

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
                    "WORKFLOW_LANGUAGE_UNSPECIFIED" => Ok(WorkflowLanguage::Unspecified),
                    "WORKFLOW_LANGUAGE_TYPESCRIPT" => Ok(WorkflowLanguage::Typescript),
                    "WORKFLOW_LANGUAGE_JAVASCRIPT" => Ok(WorkflowLanguage::Javascript),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WorkflowResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.result.is_empty() {
            len += 1;
        }
        if self.ran_at.is_some() {
            len += 1;
        }
        if self.result_type != 0 {
            len += 1;
        }
        if self.exit_code != 0 {
            len += 1;
        }
        if self.workflow_result_revision != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.WorkflowResult", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        if let Some(v) = self.ran_at.as_ref() {
            struct_ser.serialize_field("ranAt", v)?;
        }
        if self.result_type != 0 {
            let v = WorkflowResultType::try_from(self.result_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.result_type)))?;
            struct_ser.serialize_field("resultType", &v)?;
        }
        if self.exit_code != 0 {
            struct_ser.serialize_field("exitCode", &self.exit_code)?;
        }
        if self.workflow_result_revision != 0 {
            struct_ser.serialize_field("workflowResultRevision", &self.workflow_result_revision)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkflowResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "display_name",
            "displayName",
            "description",
            "result",
            "ran_at",
            "ranAt",
            "result_type",
            "resultType",
            "exit_code",
            "exitCode",
            "workflow_result_revision",
            "workflowResultRevision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            DisplayName,
            Description,
            Result,
            RanAt,
            ResultType,
            ExitCode,
            WorkflowResultRevision,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            "result" => Ok(GeneratedField::Result),
                            "ranAt" | "ran_at" => Ok(GeneratedField::RanAt),
                            "resultType" | "result_type" => Ok(GeneratedField::ResultType),
                            "exitCode" | "exit_code" => Ok(GeneratedField::ExitCode),
                            "workflowResultRevision" | "workflow_result_revision" => Ok(GeneratedField::WorkflowResultRevision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkflowResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.WorkflowResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkflowResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut result__ = None;
                let mut ran_at__ = None;
                let mut result_type__ = None;
                let mut exit_code__ = None;
                let mut workflow_result_revision__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RanAt => {
                            if ran_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ranAt"));
                            }
                            ran_at__ = map_.next_value()?;
                        }
                        GeneratedField::ResultType => {
                            if result_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultType"));
                            }
                            result_type__ = Some(map_.next_value::<WorkflowResultType>()? as i32);
                        }
                        GeneratedField::ExitCode => {
                            if exit_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exitCode"));
                            }
                            exit_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::WorkflowResultRevision => {
                            if workflow_result_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowResultRevision"));
                            }
                            workflow_result_revision__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(WorkflowResult {
                    id: id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    result: result__.unwrap_or_default(),
                    ran_at: ran_at__,
                    result_type: result_type__.unwrap_or_default(),
                    exit_code: exit_code__.unwrap_or_default(),
                    workflow_result_revision: workflow_result_revision__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.WorkflowResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkflowResultType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::SuccessUnspecified => "WORKFLOW_RESULT_TYPE_SUCCESS_UNSPECIFIED",
            Self::Failure => "WORKFLOW_RESULT_TYPE_FAILURE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for WorkflowResultType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "WORKFLOW_RESULT_TYPE_SUCCESS_UNSPECIFIED",
            "WORKFLOW_RESULT_TYPE_FAILURE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkflowResultType;

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
                    "WORKFLOW_RESULT_TYPE_SUCCESS_UNSPECIFIED" => Ok(WorkflowResultType::SuccessUnspecified),
                    "WORKFLOW_RESULT_TYPE_FAILURE" => Ok(WorkflowResultType::Failure),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WorkflowSourceById {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workflow_id.is_empty() {
            len += 1;
        }
        if !self.workflow_code_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sapphillon.v1.WorkflowSourceById", len)?;
        if !self.workflow_id.is_empty() {
            struct_ser.serialize_field("workflowId", &self.workflow_id)?;
        }
        if !self.workflow_code_id.is_empty() {
            struct_ser.serialize_field("workflowCodeId", &self.workflow_code_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkflowSourceById {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workflow_id",
            "workflowId",
            "workflow_code_id",
            "workflowCodeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkflowId,
            WorkflowCodeId,
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
                            "workflowId" | "workflow_id" => Ok(GeneratedField::WorkflowId),
                            "workflowCodeId" | "workflow_code_id" => Ok(GeneratedField::WorkflowCodeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkflowSourceById;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sapphillon.v1.WorkflowSourceById")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkflowSourceById, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workflow_id__ = None;
                let mut workflow_code_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkflowId => {
                            if workflow_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowId"));
                            }
                            workflow_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WorkflowCodeId => {
                            if workflow_code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workflowCodeId"));
                            }
                            workflow_code_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkflowSourceById {
                    workflow_id: workflow_id__.unwrap_or_default(),
                    workflow_code_id: workflow_code_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sapphillon.v1.WorkflowSourceById", FIELDS, GeneratedVisitor)
    }
}
