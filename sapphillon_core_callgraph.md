# Sapphillon Core Call Graph

This document illustrates the call graph and relationships between the main components of `sapphillon_core`.

```mermaid
graph TD
    subgraph "src/workflow.rs"
        CWC[CoreWorkflowCode]
        CWC_run[CoreWorkflowCode::run]
        CWC_extract[CoreWorkflowCode::extract_used_plugins]
        CWC_extract_fn[extract_used_plugins_from_code]
    end

    subgraph "src/runtime.rs"
        run_script[run_script]
        OSWD[OpStateWorkflowData]
        OSWD_new[OpStateWorkflowData::new]
        OSWD_add[OpStateWorkflowData::add_result]
    end

    subgraph "src/core.rs"
        op_print[op_print_wrapper]
    end

    subgraph "src/permission.rs"
        check_perm[check_permission]
        PFP[PluginFunctionPermissions]
        Perms[Permissions]
        Perms_merge[Permissions::merge]
    end

    subgraph "src/plugin.rs"
        CPP[CorePluginPackage]
        CPF[CorePluginFunction]
    end
    
    subgraph "src/utils"
        check_path[paths_cover_by_ancestor]
        check_url[urls_cover_by_ancestor]
    end

    %% CoreWorkflowCode Structure
    CWC -- contains --> CPP
    CPP -- contains --> CPF
    CWC -- contains --> PFP

    %% Execution Flow
    CWC_run -- 1. Collects Ops from --> CPP
    CWC_run -- 2. Creates --> OSWD
    CWC_run -- 3. Calls --> run_script

    %% Runtime Flow
    run_script -- Initializes --> JsRuntime[Deno JsRuntime]
    run_script -- Registers --> op_print
    run_script -- Stores --> OSWD
    run_script -- Checks Permissions --> check_perm
    run_script -- Executes --> PreRunScripts[Pre-run Scripts]
    run_script -- Executes --> WorkflowScript[Workflow Script]

    %% Permission Check Flow
    check_perm -- Merges --> Perms
    Perms -- calls --> Perms_merge
    check_perm -- Validates Filesystem --> check_path
    check_perm -- Validates Network --> check_url

    %% Op Execution Flow (Runtime)
    WorkflowScript -- calls console.log --> op_print
    op_print -- Accesses --> OSWD
    op_print -- calls --> OSWD_add

    %% Plugin Extraction
    CWC_extract -- calls --> CWC_extract_fn
    CWC_extract_fn -- reads --> CPP
```

## Component Description

### CoreWorkflowCode (`src/workflow.rs`)
The main entry point for executing a workflow. It holds the workflow code, associated plugins, and permissions.
- `run()`: Orchestrates the execution. It prepares the environment, collects plugin operations, and delegates execution to `run_script`.

### Runtime (`src/runtime.rs`)
Handles the Deno JavaScript runtime environment.
- `run_script()`: Sets up the `JsRuntime`, registers extensions (like `console.log` wrapper), enforces permissions, and executes the JavaScript code.
- `OpStateWorkflowData`: A shared state object stored in the Deno `OpState`. It holds execution results (stdout), permission configurations, and the workflow ID.

### Core (`src/core.rs`)
Contains core operations.
- `op_print_wrapper`: A Deno operation that intercepts `console.log` and `console.error`. It redirects output to `OpStateWorkflowData` if capturing is enabled, or to standard streams otherwise.

### Permission (`src/permission.rs`)
Manages permission logic.
- `check_permission()`: Verifies if the granted permissions are sufficient for the required permissions. It handles specific logic for filesystem paths and URLs (checking ancestor coverage).

### Plugin (`src/plugin.rs`)
Defines the structure of plugins.
- `CorePluginPackage` & `CorePluginFunction`: Represent the plugins and their functions available to the workflow. These provide the `OpDecl`s (native Rust functions callable from JS).
