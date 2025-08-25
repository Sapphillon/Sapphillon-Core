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

use crate::proto::sapphillon::v1 as sapphillon_v1;

impl std::fmt::Display for sapphillon_v1::Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let permission_type = self.permission_type;
        let perm = sapphillon_v1::PermissionType::try_from(permission_type).unwrap();
        let resources = self.resource.join(", ");
        write!(f, "Permission {{ type: {}, resources: [{}] }}",  perm.as_str_name(), resources)
    }
    
}
