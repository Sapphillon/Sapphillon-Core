# ext_plugin Call Stack Analysis

## Module Overview

The `ext_plugin` module provides a JavaScript execution environment using Deno's runtime. It enables Sapphillon to run plugin scripts with controlled permissions.

## Module Structure

```mermaid
graph TD
    subgraph "ext_plugin crate"
        lib["lib.rs\n(Module exports)"]
        runner["runner.rs\n(Public API)"]
        package["package.rs\n(Package Schema)"]
        parse_package["parse_package.rs\n(Package Parser)"]
        worker["worker.rs\n(Worker Factory)"]
        permissions["permissions.rs\n(Permission Utils)"]
        module_loader["module_loader.rs\n(Noop Loader)"]
        npm["npm.rs\n(Noop NPM)"]
        cert_store["cert_store.rs\n(TLS Certs)"]
        rust_js_bridge["rust_js_bridge.rs\n(Bridge Types)"]
    end

    lib --> runner
    lib --> package
    lib --> parse_package
    lib --> worker
    lib --> permissions
    lib --> module_loader
    lib --> npm
    lib --> cert_store
    lib --> rust_js_bridge
```

---

## Primary Call Stacks

### 1. `run_js` - Execute JavaScript Code

```mermaid
flowchart TB
    subgraph "runner.rs"
        run_js["run_js(script, permissions_options)"]
    end

    subgraph "worker.rs"
        create_main_worker["create_main_worker(permissions_options)"]
    end

    subgraph "permissions.rs"
        create_descriptor_parser["create_descriptor_parser()"]
        create_permissions["create_permissions(permissions_options)"]
    end

    subgraph "cert_store.rs"
        SapphillonRootCertStoreProvider_new["SapphillonRootCertStoreProvider::new()"]
    end

    subgraph "module_loader.rs"
        NoopModuleLoader["NoopModuleLoader"]
    end

    subgraph "npm.rs"
        NoopInNpmPackageChecker["NoopInNpmPackageChecker"]
        NoopNpmPackageFolderResolver["NoopNpmPackageFolderResolver"]
    end

    subgraph "Deno Runtime"
        MainWorker["MainWorker::bootstrap_from_options()"]
        execute_script["worker.execute_script()"]
        dispatch_load["worker.dispatch_load_event()"]
        run_event_loop["worker.run_event_loop()"]
        dispatch_beforeunload["worker.dispatch_beforeunload_event()"]
        dispatch_unload["worker.dispatch_unload_event()"]
    end

    run_js --> create_main_worker
    create_main_worker --> create_descriptor_parser
    create_main_worker --> create_permissions
    create_main_worker --> SapphillonRootCertStoreProvider_new
    create_main_worker --> NoopModuleLoader
    create_main_worker --> NoopInNpmPackageChecker
    create_main_worker --> NoopNpmPackageFolderResolver
    create_main_worker --> MainWorker
    run_js --> execute_script
    run_js --> dispatch_load
    run_js --> run_event_loop
    run_js --> dispatch_beforeunload
    run_js --> dispatch_unload
```

---

### 2. `run_js_with_string_arg` - Execute JS with String I/O

```mermaid
flowchart TB
    subgraph "runner.rs"
        run_js_with_string_arg["run_js_with_string_arg(script, arg, permissions_options)"]
    end

    subgraph "worker.rs"
        create_main_worker["create_main_worker(permissions_options)"]
    end

    subgraph "Deno Runtime"
        MainWorker["MainWorker::bootstrap_from_options()"]
        execute_script["worker.execute_script()"]
        dispatch_load["worker.dispatch_load_event()"]
        call_with_args["worker.js_runtime.call_with_args()"]
        with_event_loop_promise["worker.js_runtime.with_event_loop_promise()"]
        run_event_loop["worker.run_event_loop()"]
        dispatch_unload["worker.dispatch_unload_event()"]
    end

    subgraph "V8 Operations"
        resolve_entrypoint["Resolve globalThis.entrypoint"]
        create_string_arg["Create V8 string argument"]
        convert_result["Convert result to Rust String"]
    end

    run_js_with_string_arg --> create_main_worker
    create_main_worker --> MainWorker

    run_js_with_string_arg --> execute_script
    run_js_with_string_arg --> dispatch_load
    run_js_with_string_arg --> resolve_entrypoint
    run_js_with_string_arg --> create_string_arg
    run_js_with_string_arg --> call_with_args
    call_with_args --> with_event_loop_promise
    run_js_with_string_arg --> convert_result
    run_js_with_string_arg --> run_event_loop
    run_js_with_string_arg --> dispatch_unload
```

---

### 3. `SapphillonPackage::new` - Parse Plugin Package

```mermaid
flowchart TB
    subgraph "package.rs"
        SapphillonPackage_new["SapphillonPackage::new(package_script)"]
        SapphillonPackage_new_async["SapphillonPackage::new_async(package_script)"]
        entrypoint_script["SapphillonPackage::entrypoint_script()"]
    end

    subgraph "parse_package.rs"
        parse_package_info["parse_package_info(package_script)"]
    end

    subgraph "Deno Core"
        JsRuntime["JsRuntime::new()"]
        execute_script["runtime.execute_script()"]
        serde_v8["serde_v8::from_v8()"]
    end

    subgraph "rust_js_bridge.rs"
        RsJsBridgeArgs["RsJsBridgeArgs"]
        RsJsBridgeReturns["RsJsBridgeReturns"]
    end

    SapphillonPackage_new --> SapphillonPackage_new_async
    SapphillonPackage_new_async --> parse_package_info
    parse_package_info --> JsRuntime
    parse_package_info --> execute_script
    parse_package_info --> serde_v8

    entrypoint_script --> RsJsBridgeArgs
    entrypoint_script --> RsJsBridgeReturns
```

---

### 4. `SapphillonPackage::execute` - Execute Plugin Function

```mermaid
flowchart TB
    subgraph "package.rs"
        execute["SapphillonPackage::execute(args, permissions_options)"]
        entrypoint_script["entrypoint_script()"]
    end

    subgraph "runner.rs"
        run_js_with_string_arg["run_js_with_string_arg(script, input, permissions)"]
    end

    subgraph "rust_js_bridge.rs"
        RsJsBridgeArgs_to_string["RsJsBridgeArgs::to_string()"]
        RsJsBridgeReturns_new_from_str["RsJsBridgeReturns::new_from_str()"]
    end

    subgraph "worker.rs"
        create_main_worker["create_main_worker(permissions_options)"]
    end

    subgraph "Deno Runtime"
        MainWorker["MainWorker::bootstrap_from_options()"]
        execute_script["worker.execute_script()"]
        call_with_args["worker.js_runtime.call_with_args()"]
    end

    execute --> entrypoint_script
    execute --> RsJsBridgeArgs_to_string
    execute --> run_js_with_string_arg
    run_js_with_string_arg --> create_main_worker
    create_main_worker --> MainWorker
    run_js_with_string_arg --> execute_script
    run_js_with_string_arg --> call_with_args
    execute --> RsJsBridgeReturns_new_from_str
```

---

## Complete Integration Flow

```mermaid
sequenceDiagram
    participant Caller as External Caller
    participant Runner as runner.rs
    participant Worker as worker.rs
    participant Perms as permissions.rs
    participant Cert as cert_store.rs
    participant Loader as module_loader.rs
    participant NPM as npm.rs
    participant Deno as Deno Runtime

    Caller->>Runner: run_js(script, permissions)
    Runner->>Worker: create_main_worker(permissions)
    Worker->>Perms: create_descriptor_parser()
    Worker->>Perms: create_permissions(permissions)
    Worker->>Cert: SapphillonRootCertStoreProvider::new()
    Worker->>Loader: NoopModuleLoader (struct)
    Worker->>NPM: NoopInNpmPackageChecker (struct)
    Worker->>NPM: NoopNpmPackageFolderResolver (struct)
    Worker->>Deno: MainWorker::bootstrap_from_options()
    Worker-->>Runner: MainWorker
    Runner->>Deno: execute_script(script)
    Runner->>Deno: dispatch_load_event()
    loop Event Loop
        Runner->>Deno: run_event_loop(false)
        Runner->>Deno: dispatch_beforeunload_event()
    end
    Runner->>Deno: dispatch_unload_event()
    Runner-->>Caller: Result<i32>
```

---

## Function Reference Table

| Module | Function | Description |
|--------|----------|-------------|
| `runner.rs` | `run_js()` | Execute JS code, return exit code |
| `runner.rs` | `run_js_with_string_arg()` | Execute JS with string input/output |
| `worker.rs` | `create_main_worker()` | Create configured Deno MainWorker |
| `package.rs` | `SapphillonPackage::new()` | Parse package from script (sync wrapper) |
| `package.rs` | `SapphillonPackage::new_async()` | Parse package from script (async) |
| `package.rs` | `SapphillonPackage::execute()` | Execute a plugin function with given arguments |
| `package.rs` | `entrypoint_script()` | Generate JS entrypoint wrapper |
| `parse_package.rs` | `parse_package_info()` | Execute package script and deserialize |
| `permissions.rs` | `create_descriptor_parser()` | Create Deno permission parser |
| `permissions.rs` | `create_permissions()` | Create Deno permissions from options |
| `permissions.rs` | `permissions_options_from_sapphillon_permissions()` | Convert Sapphillon permissions to Deno format |
| `cert_store.rs` | `SapphillonRootCertStoreProvider::new()` | Create TLS cert store provider |
| `cert_store.rs` | `get_or_try_init()` | Lazy-load root certificates |
| `module_loader.rs` | `NoopModuleLoader::resolve()` | Reject module resolution |
| `module_loader.rs` | `NoopModuleLoader::load()` | Reject module loading |
| `npm.rs` | `NoopInNpmPackageChecker::in_npm_package()` | Always returns false |
| `npm.rs` | `NoopNpmPackageFolderResolver::resolve_package_folder_from_package()` | Always returns error |
| `rust_js_bridge.rs` | `RsJsBridgeArgs::new_from_str()` | Deserialize args from JSON |
| `rust_js_bridge.rs` | `RsJsBridgeReturns::new_from_str()` | Deserialize returns from JSON |

---

## Function Dependency Graphs (Detailed)

### Internal Function Dependencies

```mermaid
graph TD
    subgraph "Public Entry Points"
        run_js["run_js()"]
        run_js_with_string_arg["run_js_with_string_arg()"]
        SapphillonPackage_new["SapphillonPackage::new()"]
        SapphillonPackage_execute["SapphillonPackage::execute()"]
    end

    subgraph "runner.rs"
        to_js_error["to_js_error()"]
    end

    subgraph "worker.rs"
        create_main_worker["create_main_worker()"]
    end

    subgraph "permissions.rs"
        create_descriptor_parser["create_descriptor_parser()"]
        create_permissions["create_permissions()"]
        permissions_options_from_sapphillon["permissions_options_from_sapphillon_permissions()"]
        merge_allow_list["merge_allow_list()"]
    end

    subgraph "package.rs"
        SapphillonPackage_new_async["SapphillonPackage::new_async()"]
        entrypoint_script["entrypoint_script()"]
        SapphillonPackage_execute_impl["execute()"]
    end

    subgraph "parse_package.rs"
        parse_package_info["parse_package_info()"]
    end

    subgraph "cert_store.rs"
        SapphillonRootCertStoreProvider_new["SapphillonRootCertStoreProvider::new()"]
        get_or_try_init["get_or_try_init()"]
    end

    subgraph "rust_js_bridge.rs"
        RsJsBridgeArgs_new_from_str["RsJsBridgeArgs::new_from_str()"]
        RsJsBridgeArgs_to_string["RsJsBridgeArgs::to_string()"]
        RsJsBridgeReturns_new_from_str["RsJsBridgeReturns::new_from_str()"]
        RsJsBridgeReturns_to_string["RsJsBridgeReturns::to_string()"]
    end

    %% run_js dependencies
    run_js --> create_main_worker
    run_js --> to_js_error

    %% run_js_with_string_arg dependencies
    run_js_with_string_arg --> create_main_worker
    run_js_with_string_arg --> to_js_error

    %% create_main_worker dependencies
    create_main_worker --> create_descriptor_parser
    create_main_worker --> create_permissions
    create_main_worker --> SapphillonRootCertStoreProvider_new

    %% create_permissions dependencies
    create_permissions --> create_descriptor_parser

    %% permissions_options conversion
    permissions_options_from_sapphillon --> merge_allow_list

    %% SapphillonPackage dependencies
    SapphillonPackage_new --> SapphillonPackage_new_async
    SapphillonPackage_new_async --> parse_package_info

    %% entrypoint_script dependencies
    entrypoint_script --> RsJsBridgeArgs_to_string
    entrypoint_script --> RsJsBridgeReturns_to_string

    %% execute() dependencies
    SapphillonPackage_execute --> entrypoint_script
    SapphillonPackage_execute --> RsJsBridgeArgs_to_string
    SapphillonPackage_execute --> run_js_with_string_arg
    SapphillonPackage_execute --> RsJsBridgeReturns_new_from_str
    SapphillonPackage_execute_impl --> entrypoint_script
    SapphillonPackage_execute_impl --> run_js_with_string_arg
```

---

### runner.rs Function Dependencies

```mermaid
graph LR
    subgraph "runner.rs"
        run_js["run_js(script, permissions_options)\n→ JsResult<i32>"]
        run_js_with_string_arg["run_js_with_string_arg(script, arg, permissions_options)\n→ JsResult<String>"]
        to_js_error["to_js_error<E>(err)\n→ Js_Error"]
    end

    subgraph "worker.rs"
        create_main_worker["create_main_worker()"]
    end

    run_js -->|"calls"| create_main_worker
    run_js -->|"uses"| to_js_error
    run_js_with_string_arg -->|"calls"| create_main_worker
    run_js_with_string_arg -->|"uses"| to_js_error
```

---

### worker.rs → permissions.rs Dependencies

```mermaid
graph LR
    subgraph "worker.rs"
        create_main_worker["create_main_worker(permissions_options)\n→ Result<MainWorker>"]
    end

    subgraph "permissions.rs"
        create_descriptor_parser["create_descriptor_parser()\n→ Arc<RuntimePermissionDescriptorParser>"]
        create_permissions["create_permissions(permissions_options)\n→ Result<Permissions>"]
    end

    subgraph "cert_store.rs"
        SapphillonRootCertStoreProvider_new["SapphillonRootCertStoreProvider::new()\n→ Self"]
    end

    create_main_worker -->|"calls"| create_descriptor_parser
    create_main_worker -->|"calls"| create_permissions
    create_main_worker -->|"calls"| SapphillonRootCertStoreProvider_new
    create_permissions -->|"calls"| create_descriptor_parser
```

---

### package.rs → parse_package.rs Dependencies

```mermaid
graph LR
    subgraph "package.rs"
        SapphillonPackage_new["SapphillonPackage::new(package_script)\n→ Result<SapphillonPackage>"]
        SapphillonPackage_new_async["SapphillonPackage::new_async(package_script)\n→ Result<SapphillonPackage>"]
        entrypoint_script["entrypoint_script(&self)\n→ serde_json::Result<String>"]
        execute["execute(&self, args, permissions)\n→ Result<RsJsBridgeReturns>"]
    end

    subgraph "parse_package.rs"
        parse_package_info["parse_package_info(package_script)\n→ Result<SapphillonPackage>"]
    end

    subgraph "runner.rs"
        run_js_with_string_arg["run_js_with_string_arg()"]
    end

    subgraph "rust_js_bridge.rs"
        RsJsBridgeArgs["RsJsBridgeArgs (type)"]
        RsJsBridgeReturns["RsJsBridgeReturns (type)"]
    end

    SapphillonPackage_new -->|"calls"| SapphillonPackage_new_async
    SapphillonPackage_new_async -->|"calls"| parse_package_info
    entrypoint_script -->|"uses"| RsJsBridgeArgs
    entrypoint_script -->|"uses"| RsJsBridgeReturns
    execute -->|"calls"| entrypoint_script
    execute -->|"calls"| run_js_with_string_arg
    execute -->|"uses"| RsJsBridgeArgs
    execute -->|"uses"| RsJsBridgeReturns
```

---

### permissions.rs Internal Dependencies

```mermaid
graph TD
    subgraph "permissions.rs"
        create_descriptor_parser["create_descriptor_parser()\n→ Arc<RuntimePermissionDescriptorParser>"]
        create_permissions["create_permissions(permissions_options)\n→ Result<Permissions>"]
        permissions_options_from_sapphillon["permissions_options_from_sapphillon_permissions(permissions)\n→ PermissionsOptions"]
        merge_allow_list["merge_allow_list(target, resources)\n(internal helper)"]
    end

    create_permissions -->|"calls"| create_descriptor_parser
    permissions_options_from_sapphillon -->|"calls"| merge_allow_list
```

---

### Struct/Type Dependencies

```mermaid
graph TD
    subgraph "package.rs Types"
        SapphillonPackage["SapphillonPackage"]
        Meta["Meta"]
        FunctionSchema["FunctionSchema"]
        Permission["Permission"]
        Parameter["Parameter"]
        ReturnInfo["ReturnInfo"]
    end

    subgraph "rust_js_bridge.rs Types"
        RsJsBridgeArgs["RsJsBridgeArgs"]
        RsJsBridgeReturns["RsJsBridgeReturns"]
    end

    subgraph "Noop Implementations"
        NoopModuleLoader["NoopModuleLoader\n(module_loader.rs)"]
        NoopInNpmPackageChecker["NoopInNpmPackageChecker\n(npm.rs)"]
        NoopNpmPackageFolderResolver["NoopNpmPackageFolderResolver\n(npm.rs)"]
        SapphillonRootCertStoreProvider["SapphillonRootCertStoreProvider\n(cert_store.rs)"]
    end

    SapphillonPackage --> Meta
    SapphillonPackage --> FunctionSchema
    FunctionSchema --> Permission
    FunctionSchema --> Parameter
    FunctionSchema --> ReturnInfo

    SapphillonPackage -.->|"uses for entrypoint"| RsJsBridgeArgs
    SapphillonPackage -.->|"uses for entrypoint"| RsJsBridgeReturns
    SapphillonPackage -.->|"uses for execute"| RsJsBridgeArgs
    SapphillonPackage -.->|"uses for execute"| RsJsBridgeReturns
```

---

## Key Dependencies

```mermaid
graph LR
    ext_plugin --> deno_runtime
    ext_plugin --> deno_permissions
    ext_plugin --> deno_tls
    ext_plugin --> deno_error
    ext_plugin --> node_resolver
    ext_plugin --> serde_v8
    ext_plugin --> sapphillon_core

    deno_runtime --> v8
```
