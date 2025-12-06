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
use deno_runtime::worker::{
    MainWorker, WorkerOptions,
};
use deno_runtime::deno_permissions::{PermissionsContainer, PermissionDescriptorParser};
use deno_core::ModuleSpecifier;
use std::sync::Arc;

async fn run(script: String) -> anyhow::Result<()> {

    let options = WorkerOptions {
        ..Default::default()
    };
    let main_module = deno_core::resolve_url_or_path("./main.js", std::path::Path::new("."))?;
    let main_worker_service_option = deno_runtime::worker::MainWorkerServiceOption {
        ..Default::default()
    };
    

    
    let main_worker = MainWorker::bootstrap_from_options(
        &main_module,
        main_worker_service_option,
        options,
    );

    Ok(())
}
