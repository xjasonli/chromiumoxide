//! Types for managing JavaScript execution contexts in Chrome DevTools Protocol.
//!
//! This module provides types that represent different ways to identify and scope
//! JavaScript execution contexts when evaluating code in the browser.

use chromiumoxide_cdp::cdp::js_protocol::runtime::ExecutionContextId;

/// Represents a JavaScript execution context in the browser.
///
/// An execution context is an environment where JavaScript code can be evaluated.
/// Each frame (main document or iframe) has its own execution context, and additional
/// contexts can be created by extensions or web workers.
///
/// This type supports two ways to identify a context:
/// - By its numeric ID (`ExecutionContextId`)
/// - By a unique string identifier (`UniqueId`)
#[derive(Debug, Clone)]
pub enum ExecutionContext {
    /// Identifies a context by its numeric ID
    Id(ExecutionContextId),

    /// Identifies a context by a unique string identifier
    UniqueId(String),
}

impl ExecutionContext {
    /// Returns the numeric context ID if this is an ID-based context
    pub fn id(&self) -> Option<ExecutionContextId> {
        match self {
            ExecutionContext::Id(id) => Some(id.clone()),
            _ => None,
        }
    }

    /// Returns the unique identifier if this is a name-based context
    pub fn unique_id(&self) -> Option<String> {
        match self {
            ExecutionContext::UniqueId(unique_id) => Some(unique_id.clone()),
            _ => None,
        }
    }
}

impl From<ExecutionContextId> for ExecutionContext {
    fn from(id: ExecutionContextId) -> Self {
        ExecutionContext::Id(id)
    }
}

impl From<&ExecutionContextId> for ExecutionContext {
    fn from(id: &ExecutionContextId) -> Self {
        ExecutionContext::Id(*id)
    }
}

impl From<String> for ExecutionContext {
    fn from(unique_id: String) -> Self {
        ExecutionContext::UniqueId(unique_id)
    }
}

impl From<&String> for ExecutionContext {
    fn from(unique_id: &String) -> Self {
        ExecutionContext::UniqueId(unique_id.clone())
    }
}

impl From<&str> for ExecutionContext {
    fn from(unique_id: &str) -> Self {
        ExecutionContext::UniqueId(unique_id.to_string())
    }
}
