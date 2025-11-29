// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

use crate::runtime::{OpStateWorkflowData, WorkflowStdout};
use deno_core::{OpState, op2};
use std::io::{Write, stderr, stdout};
use std::sync::{Arc, Mutex};

/// A Deno op to wrap the `console.log` and `console.error` calls.
///
/// This function intercepts print operations from JavaScript. If output capturing is enabled
/// in the `OpStateWorkflowData`, the message is stored in the state. Otherwise, it's printed
/// to the standard output or standard error.
///
/// # Arguments
///
/// * `state` - The Deno `OpState`, used to access shared workflow data.
/// * `msg` - The message string to be printed.
/// * `is_err` - A boolean flag indicating if the message is an error.
///
/// # Returns
///
/// * `Ok(())` on successful execution.
/// * `Err(std::io::Error)` if writing to `stdout` or `stderr` fails.
#[op2(fast)]
pub(crate) fn op_print_wrapper(
    state: &mut OpState,
    #[string] msg: &str,
    is_err: bool,
) -> Result<(), std::io::Error> {
    let mut data = state
        .borrow_mut::<Arc<Mutex<OpStateWorkflowData>>>()
        .lock()
        .unwrap();

    if is_err {
        if data.is_capture_stdout() {
            // data.add_result(WorkflowStdout::Stderr(msg.to_string()));
            data.add_result(WorkflowStdout::Stdout(msg.to_string()));
        } else {
            stderr().write_all(msg.as_bytes())?;
            stderr().flush().unwrap();
        }
    } else if data.is_capture_stdout() {
        data.add_result(WorkflowStdout::Stdout(msg.to_string()));
    } else {
        stdout().write_all(msg.as_bytes())?;
        stdout().flush().unwrap();
    }

    Ok(())
}
