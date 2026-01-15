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

function add(a, b) {
  return a + b;
}

function mul(a, b) {
  return a * b;
}

function readSecretFile(path) {
  // This function requires file system read permission
  return Deno.readTextFileSync(path);
}

function getMultipleValues(a, b) {
  // Returns an array of multiple values for testing multi-return handling
  return [a + b, a * b, a - b];
}

Sapphillon.Package = {
  // Information from package.toml
  meta: {
    name: "math-plugin",
    version: "1.0.0",
    description: "desc",
    author_id: "com.example",
    package_id: "com.example.math-plugin"
  },
  // Schema information generated from JSDoc
  functions: {
    add: {       // Function name
      handler: add,         // Actual function reference
      permissions: [{ type: "FileSystemRead", resource: "/etc" }],
      description: "Adds two numbers.", // JSDoc summary
      parameters: [         // Parsed result of @param
        { name: "a", idx: 0, type: "number", description: "The number to be added to" },
        { name: "b", idx: 1, type: "number", description: "The number to add" }
      ],
      returns: [ // Parsed result of @returns
        { type: "number", idx: 0, description: "The sum" }
      ]
    },
    mul: {       // Function name
      handler: mul,         // Actual function reference
      permissions: [{ type: "FileSystemRead", resource: "/etc" }],
      description: "Multiplies two numbers.", // JSDoc summary
      parameters: [         // Parsed result of @param
        { name: "a", idx: 0, type: "number", description: "The first factor" },
        { name: "b", idx: 1, type: "number", description: "The second factor" }
      ],
      returns: [ // Parsed result of @returns
        { type: "number", idx: 0, description: "The product" }
      ]
    },
    readSecretFile: {
      handler: readSecretFile,
      permissions: [{ type: "FileSystemRead", resource: "*" }],
      description: "Reads a file from the filesystem.",
      parameters: [
        { name: "path", idx: 0, type: "string", description: "The file path to read" }
      ],
      returns: [
        { type: "string", idx: 0, description: "The file contents" }
      ]
    },
    getMultipleValues: {
      handler: getMultipleValues,
      permissions: [],
      description: "Returns multiple computed values as an array.",
      parameters: [
        { name: "a", idx: 0, type: "number", description: "First number" },
        { name: "b", idx: 1, type: "number", description: "Second number" }
      ],
      returns: [
        { type: "number", idx: 0, description: "Sum (a + b)" },
        { type: "number", idx: 1, description: "Product (a * b)" },
        { type: "number", idx: 2, description: "Difference (a - b)" }
      ]
    }
  }
};