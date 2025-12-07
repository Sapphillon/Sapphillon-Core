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
//

use anyhow::Result;
use deno_runtime::{FeatureChecker, deno_permissions::PermissionsContainer};
use deno_web::BlobStore;
use std::sync::Arc;
use deno_lib::worker::{CreateModuleLoaderResult, LibMainWorkerFactory, ModuleLoaderFactory};

struct NoopModuleLoaderFactory;
impl ModuleLoaderFactory for NoopModuleLoaderFactory {
    fn create_for_main(&self, _root_permissions: PermissionsContainer) -> CreateModuleLoaderResult{
        // TODO: implement a proper module loader
        unimplemented!()
    }
    
    fn create_for_worker(
        &self,
        _parent_permissions: PermissionsContainer,
        _permissions: PermissionsContainer,
      ) -> CreateModuleLoaderResult {
        unimplemented!()
    }
}

async fn run() -> Result<()> {
    let blob_store = BlobStore::default();
    let feature_checker = Arc::new({
        let mut checker = FeatureChecker::default();
    });
    let fs = Arc::new(sys_traits::impls::RealSys);
    let module_loader_factory = Box::new(NoopModuleLoaderFactory);
    
    

    Ok(())
}
