//! Helpers for normalizing generated source code.

/// Removes Markdown code fences and surrounding whitespace from generated code.
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
