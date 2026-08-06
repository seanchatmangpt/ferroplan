//! Common wire, every channel routed through it. One relay station for the
//! outbound signal.
//!
//! The `Result<Value, String>` convention is the dispatch format shared by
//! `session`, `admission`, and the stateless-planning tools; `to_result` is
//! the single relay that maps it onto rmcp's `CallToolResult`. A clean
//! signal carries the JSON payload twice — pretty text for the model reading
//! `content`, and `structuredContent` for callers parsing the object
//! straight. A dead signal carries the failure message as text only, no
//! structured payload riding along — a broken transmission doesn't get
//! dressed up as data.
//!
//! Note: this module's `pretty` is a different unit from `crate::pretty`,
//! which is a generic fallible `Serialize` → `Result<String, String>`
//! helper. This one never fails, and only speaks `Value`.

use rmcp::model::{CallToolResult, ContentBlock, ErrorData as McpError};
use serde_json::Value;

/// Route the `Result<Value, String>` signal onto rmcp's `CallToolResult`.
/// `structuredContent` rides along only when the transmission is clean.
pub(crate) fn to_result(result: Result<Value, String>) -> Result<CallToolResult, McpError> {
    Ok(match result {
        Ok(value) => {
            let mut r = CallToolResult::success(vec![ContentBlock::text(pretty(&value))]);
            r.structured_content = Some(value);
            r
        }
        Err(message) => CallToolResult::error(vec![ContentBlock::text(message)]),
    })
}

/// Pretty-print a JSON value. If the printer chokes, fall back to the
/// compact form — the signal still gets through, just flatter.
pub(crate) fn pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}
