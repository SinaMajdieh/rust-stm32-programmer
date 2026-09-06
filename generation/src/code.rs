//! Helpers for normalizing generated source code.
//!
//! These helpers handle formatting artifacts commonly introduced when an LLM
//! returns source code inside Markdown code fences.

/// Removes Markdown code fences and surrounding whitespace from generated code.
///
/// If `code` starts with a C-specific Markdown fence (` ```c `), that fence is
/// removed. Generic Markdown code fences are handled as well. If a closing
/// fence is missing, the remaining content is returned unchanged apart from
/// surrounding whitespace.
pub(crate) fn clean_generated_code(code: &str) -> &str {
    let code = code.trim();

    if let Some(code) = code.strip_prefix("```c") {
        return code.strip_suffix("```").unwrap_or(code).trim();
    }

    if let Some(code) = code.strip_prefix("```") {
        return code.strip_suffix("```").unwrap_or(code).trim();
    }

    code
}
