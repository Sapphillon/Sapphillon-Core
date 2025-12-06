// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

#[allow(unused_imports)]
use crate::permission::*;
use thiserror::Error;

/// Top-level error type for the Sapphillon Core library.
///
/// This enum is the crate's primary error wrapper and currently holds the
/// different error kinds emitted by the workflow runtime. Variants are
/// transparent where appropriate so that source errors and their `Display`
/// output are preserved for logging and error chaining.
///
/// # Examples
///
/// ```rust,ignore
/// // Constructing and inspecting the top-level error (example only).
/// let _ = Error::WorkflowRuntimeError(/* WorkflowRuntimeError value here */);
/// ```
#[derive(Error, Debug)]
pub enum Error {
    /// Error originating from the workflow runtime.
    ///
    /// This variant wraps [`WorkflowRuntimeError`]. The `#[error(transparent)]`
    /// attribute preserves the inner error's `Display` text when this variant
    /// is formatted, and it enables `From<WorkflowRuntimeError>` conversions.
    #[error(transparent)]
    WorkflowRuntimeError(#[from] WorkflowRuntimeError),

    /// Permission denied error for permission checks.
    ///
    /// Wraps [`PermissionDeniedError`] so callers can return a unified `Error`.
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
}

/// Specific categories of workflow runtime failures.
///
/// Each variant describes a coarse-grained reason why the runtime failed.
/// Use this enum to programmatically branch on the kind of runtime error.
#[derive(Debug)]
pub enum WorkflowRuntimeErrorType {
    /// Failed while preparing a core plugin before execution.
    ///
    /// This occurs during plugin initialization or configuration steps that
    /// must succeed before the plugin can be executed by the workflow runtime.
    CorePluginPrepareError,

    /// The core plugin failed during its execution phase.
    ///
    /// This indicates an error that happened while the plugin was actively
    /// running (for example, runtime panics, bad returns, or plugin logic
    /// errors).
    CorePluginExecuteError,

    /// An error produced while executing the workflow script itself.
    ///
    /// This covers runtime failures originating from the user-provided
    /// workflow script (script exceptions, evaluation errors, etc.).
    WorkflowScriptExecuteError,
}

impl std::fmt::Display for WorkflowRuntimeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowRuntimeErrorType::CorePluginPrepareError => {
                write!(f, "Core plugin prepare error")
            }
            WorkflowRuntimeErrorType::CorePluginExecuteError => {
                write!(f, "Core plugin execute error")
            }
            WorkflowRuntimeErrorType::WorkflowScriptExecuteError => {
                write!(f, "Workflow script execute error")
            }
        }
    }
}

/// Error raised by the workflow runtime when JavaScript execution or plugin
/// orchestration fails.
///
/// This struct carries a human-readable message, a categorized error type, and
/// the original JavaScript error produced by `deno_core`. The type is used to
/// decide programmatic handling, while `message` gives a concise summary for
/// logs and user-facing output. Note that `js_error` is included in the
/// `Display` representation via the `thiserror` formatting string above but is
/// not annotated with `#[source]` in this file.
///
/// # Examples
///
/// ```rust,ignore
/// // Example shows how to inspect a workflow runtime error (illustrative).
/// let err = WorkflowRuntimeError {
///     message: "evaluation failed".into(),
///     error_type: WorkflowRuntimeErrorType::WorkflowScriptExecuteError,
///     js_error: /* deno_core::error::CoreError value here */ unimplemented!(),
/// };
/// assert_eq!(err.error_type.to_string(), "Workflow script execute error");
/// ```
#[derive(Error, Debug)]
#[error("Workflow runtime error: {message}, type: {error_type}, details: {js_error}")]
pub struct WorkflowRuntimeError {
    /// A concise, human-readable description of the error.
    ///
    /// This should summarize what failed and can be shown directly in logs or
    /// user-facing messages. Keep this short; for rich details prefer
    /// `js_error`.
    pub message: String,

    /// The category of the workflow runtime error.
    ///
    /// Use this to branch on the kind of failure without parsing text. The
    /// value is included in the struct's `Display` output.
    pub error_type: WorkflowRuntimeErrorType,

    /// The underlying JavaScript error produced by `deno_core`.
    ///
    /// This contains origin information, stack traces, and other JS-side
    /// details. It is interpolated into the struct's `Display` message but is
    /// not declared with `#[source]` here; callers that need source chaining
    /// should examine `js_error` directly.
    pub js_error: deno_core::error::CoreError,
}

#[derive(Error, Debug)]
#[error(
    "Permission denied: Requested Permissions: {}, Granted Permissions: {}",
    requested,
    granted
)]
pub struct PermissionDeniedError {
    /// The permissions that were requested.
    pub requested: Permissions,

    /// The permissions that were granted.
    pub granted: Permissions,
}
