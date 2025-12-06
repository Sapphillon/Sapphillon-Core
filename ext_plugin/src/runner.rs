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
use deno_runtime::FeatureChecker;
use deno_web::BlobStore;
use std::sync::Arc;
use deno_lib::worker::LibMainWorkerFactory;


async fn run() -> Result<()> {
    let blob_store = BlobStore::default();
    let code_cache = None;
    let deno_rt_native_addon_loader = None;
    let feature_checker = Arc::new({
        let mut checker = FeatureChecker::default();
        
    });

    Ok(())
}
