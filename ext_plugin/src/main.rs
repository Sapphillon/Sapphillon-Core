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

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let script_path = if args.len() > 1 {
        &args[1]
    } else {
        // Default to test.js in the ext_plugin directory
        "ext_plugin/test.js"
    };

    println!("Running JavaScript file: {script_path}");

    match ext_plugin::run(script_path).await {
        Ok(_) => println!("\nScript execution completed successfully."),
        Err(e) => eprintln!("Error executing script: {e}"),
    }
}
