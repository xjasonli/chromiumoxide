//! Types for managing JavaScript execution contexts in Chrome DevTools Protocol.
//!
//! This module provides types that represent different ways to identify and scope
//! JavaScript execution contexts when evaluating code in the browser.

use chromiumoxide_cdp::cdp::js_protocol::runtime::{CallFunctionOnParams, ExecutionContextId};
use super::*;

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
pub enum GlobalExecutionContext {
    /// Identifies a context by its numeric ID
    Id(ExecutionContextId),
    /// Identifies a context by a unique string identifier
    UniqueId(String),
}

impl From<ExecutionContextId> for GlobalExecutionContext {
    fn from(id: ExecutionContextId) -> Self {
        Self::Id(id)
    }
}

impl From<String> for GlobalExecutionContext {
    fn from(id: String) -> Self {
        Self::UniqueId(id)
    }
}

impl From<GlobalExecutionContext> for ExecutionContext {
    fn from(context: GlobalExecutionContext) -> Self {
        match context {
            GlobalExecutionContext::Id(id) => ExecutionContext::Id(id),
            GlobalExecutionContext::UniqueId(id) => ExecutionContext::UniqueId(id),
        }
    }
}

/// Represents a scoped JavaScript execution context in the browser.
///
/// This type extends `ExecutionContext` by adding the ability to scope execution
/// to a specific JavaScript object. This is useful when evaluating code that needs
/// to access or modify a particular object's properties or methods.
///
/// The three ways to scope execution are:
/// - By context ID (execute in a specific frame/worker)
/// - By unique ID (execute in a named context)
/// - By object ID (execute with a specific object as scope)
#[derive(Debug, Clone)]
pub enum ExecutionContext {
    /// Execute in the context identified by this ID
    Id(ExecutionContextId),
    /// Execute in the context with this unique identifier
    UniqueId(String),
    /// Execute with this object as the scope
    RemoteObject(JsRemoteObject),
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

    /// Returns the object ID if this is an object-scoped context
    pub fn remote_object(&self) -> Option<&JsRemoteObject> {
        match self {
            ExecutionContext::RemoteObject(object) => Some(object),
            _ => None,
        }
    }

    pub(crate) fn apply(self, mut params: CallFunctionOnParams) -> CallFunctionOnParams {
        match self {
            ExecutionContext::Id(id) => params.execution_context_id = Some(id),
            ExecutionContext::UniqueId(unique_id) => params.unique_context_id = Some(unique_id),
            ExecutionContext::RemoteObject(remote_object) => params.object_id = Some(remote_object.remote_id()),
        }
        params
    }
}

impl From<&JsRemoteObject> for ExecutionContext {
    fn from(value: &JsRemoteObject) -> Self {
        ExecutionContext::RemoteObject(value.clone())
    }
}

impl From<JsRemoteObject> for ExecutionContext {
    fn from(value: JsRemoteObject) -> Self {
        ExecutionContext::RemoteObject(value)
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
