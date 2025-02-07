use chromiumoxide_cdp::cdp::js_protocol::runtime::{CallFunctionOnParams, ExecutionContextId, RemoteObjectId};
use super::*;


#[derive(Debug, Clone)]
pub enum ExecutionContext {
    Id(ExecutionContextId),
    UniqueId(String),
}

impl From<ExecutionContextId> for ExecutionContext {
    fn from(id: ExecutionContextId) -> Self {
        Self::Id(id)
    }
}

impl From<String> for ExecutionContext {
    fn from(id: String) -> Self {
        Self::UniqueId(id)
    }
}

impl From<ExecutionContext> for ScopedExecutionContext {
    fn from(context: ExecutionContext) -> Self {
        match context {
            ExecutionContext::Id(id) => ScopedExecutionContext::Id(id),
            ExecutionContext::UniqueId(id) => ScopedExecutionContext::UniqueId(id),
        }
    }
}


#[derive(Debug, Clone)]
pub enum ScopedExecutionContext {
    Id(ExecutionContextId),
    UniqueId(String),
    ObjectId(RemoteObjectId),
}

impl ScopedExecutionContext {
    pub fn id(&self) -> Option<ExecutionContextId> {
        match self {
            ScopedExecutionContext::Id(id) => Some(id.clone()),
            _ => None,
        }
    }
    pub fn unique_id(&self) -> Option<String> {
        match self {
            ScopedExecutionContext::UniqueId(unique_id) => Some(unique_id.clone()),
            _ => None,
        }
    }
    pub fn object_id(&self) -> Option<RemoteObjectId> {
        match self {
            ScopedExecutionContext::ObjectId(object_id) => Some(object_id.clone()),
            _ => None,
        }
    }
    pub(crate) fn apply(self, mut params: CallFunctionOnParams) -> CallFunctionOnParams {
        match self {
            ScopedExecutionContext::Id(id) => params.execution_context_id = Some(id),
            ScopedExecutionContext::UniqueId(unique_id) => params.unique_context_id = Some(unique_id),
            ScopedExecutionContext::ObjectId(object_id) => params.object_id = Some(object_id),
        }
        params
    }
}

impl From<&JsRemoteObject> for ScopedExecutionContext {
    fn from(value: &JsRemoteObject) -> Self {
        ScopedExecutionContext::ObjectId(value.object_id())
    }
}
impl From<JsRemoteObject> for ScopedExecutionContext {
    fn from(value: JsRemoteObject) -> Self {
        ScopedExecutionContext::ObjectId(value.object_id())
    }
}
impl From<RemoteObjectId> for ScopedExecutionContext {
    fn from(object_id: RemoteObjectId) -> Self {
        ScopedExecutionContext::ObjectId(object_id)
    }
}
impl From<ExecutionContextId> for ScopedExecutionContext {
    fn from(id: ExecutionContextId) -> Self {
        ScopedExecutionContext::Id(id)
    }
}
impl From<String> for ScopedExecutionContext {
    fn from(unique_id: String) -> Self {
        ScopedExecutionContext::UniqueId(unique_id)
    }
}
