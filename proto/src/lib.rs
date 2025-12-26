// Sapphillon-Core Proto
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

// Re-export dependencies for use by sapphillon_core
pub use prost;
pub use prost_types;
pub use tonic;

// Re-export the generated protobuf code

pub mod sapphillon {
    #![allow(clippy::all)]
    #![cfg(not(doctest))]

    pub mod v1 {
        include!("proto/sapphillon.v1.rs");

        impl std::fmt::Display for Permission {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let permission_type = self.permission_type;
                let perm = PermissionType::try_from(permission_type).unwrap();
                let resources = self.resource.join(", ");
                write!(
                    f,
                    "Permission {{{{ type: {}, resources: [{}] }}}}",
                    perm.as_str_name(),
                    resources
                )
            }
        }
    }

    pub mod ai {
        pub mod v1 {
            include!("proto/sapphillon.ai.v1.rs");
        }
    }
}

pub mod google {
    #![allow(clippy::all)]
    #![cfg(not(doctest))]

    pub mod api {
        include!("proto/google.api.rs");
        pub mod expr {
            pub mod v1alpha1 {
                include!("proto/google.api.expr.v1alpha1.rs");
            }
            pub mod v1beta1 {
                include!("proto/google.api.expr.v1beta1.rs");
            }
        }
    }
    pub mod bytestream {
        include!("proto/google.bytestream.rs");
    }
    pub mod longrunning {
        include!("proto/google.longrunning.rs");
    }
    pub mod geo {
        pub mod r#type {
            include!("proto/google.geo.type.rs");
        }
    }
    pub mod rpc {
        include!("proto/google.rpc.rs");
        pub mod context {
            include!("proto/google.rpc.context.rs");
        }
    }
    pub mod r#type {
        include!("proto/google.type.rs");
    }

    pub mod protobuf {
        include!("proto/google.protobuf.rs");
        pub mod compiler {
            include!("proto/google.protobuf.compiler.rs");
        }
    }
}
