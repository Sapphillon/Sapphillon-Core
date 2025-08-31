// Sapphillon-Core
// Copyright 2025 Yuta Takahashi
//
// This file is part of Sapphillon-Core
//
// Sapphillon-Core is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;

use crate::proto::sapphillon::{self, v1 as sapphillon_v1};

impl std::fmt::Display for sapphillon_v1::Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let permission_type = self.permission_type;
        let perm = sapphillon_v1::PermissionType::try_from(permission_type).unwrap();
        let resources = self.resource.join(", ");
        write!(f, "Permission {{ type: {}, resources: [{}] }}",  perm.as_str_name(), resources)
    }
    
}

#[derive(Debug, Clone, PartialEq)]
pub struct Permissions {
    pub permissions: Vec<sapphillon_v1::Permission>
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = self.permissions.iter().map(|p| format!("{p}")).collect::<Vec<_>>().join(", ");
        write!(f, "Permissions: [{msg}]")
    }
    
}


impl Permissions {
    pub fn new(permissions: Vec<sapphillon_v1::Permission>) -> Self {
        Self { permissions }
    }
    
    pub fn merge(self) -> Self {
        let mut perm_map: HashMap<i32, sapphillon_v1::Permission> = HashMap::new();
        
        self.permissions.iter().for_each(
            |p| {
                match perm_map.get(&p.permission_type) {
                    Some(perm) =>  {
                        let new_permission = sapphillon_v1::Permission {
                            display_name: p.display_name.clone() + ", " + &perm.display_name.clone(),
                            description: p.description.clone() + ", " + &perm.description.clone(),
                            permission_type: p.permission_type,
                            resource: [p.resource.clone(), perm.resource.clone()].concat(),
                            permission_level: std::cmp::max(perm.permission_level, p.permission_level)
                        };
                        perm_map.insert(p.permission_type, new_permission);
                    },
                    None => {perm_map.insert(p.permission_type, p.clone());}
                }
            }
        );
        Permissions::new(perm_map.into_values().collect())
    }
}

