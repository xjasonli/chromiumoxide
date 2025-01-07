use std::{marker::PhantomData, sync::Arc};
use chromiumoxide_cdp::cdp::js_protocol::runtime::{CallFunctionOnParams, ExecutionContextId, RemoteObjectId};
use serde_json::Value;

use crate::{error::CdpError, handler::PageInner};

use super::{Js2RustValue, Js2Rust, Rust2Js};

#[derive(Debug, Clone)]
pub struct Function<A: Args = (), R: Js2Rust = Value> {
    tab: Arc<PageInner>,
    params: CallFunctionOnParams,
    signature: PhantomData<(A, R)>,
}

impl<A: Args, R: Js2Rust> Function<A, R> {
    pub(crate) fn new(tab: Arc<PageInner>, function: impl Into<String>) -> Self {
        Self {
            tab,
            params: CallFunctionOnParams::builder()
                .function_declaration(function)
                .build()
                .unwrap(),
            signature: PhantomData,
        }
    }

    pub fn await_promise(self, await_promise: bool) -> Self {
        let mut params = self.params;
        params.await_promise = Some(await_promise);
        Self {
            tab: self.tab,
            params,
            signature: PhantomData,
        }
    }

    pub fn execution_context(self, execution_context: impl Into<ExecutionContextId>) -> Self {
        let mut params = self.params;
        params.execution_context_id = Some(execution_context.into());
        Self {
            tab: self.tab,
            params,
            signature: PhantomData,
        }
    }

    pub fn return_by_value(self, return_by_value: bool) -> Self {
        let mut params = self.params;
        params.return_by_value = Some(return_by_value);
        Self {
            tab: self.tab,
            params,
            signature: PhantomData,
        }
    }

    pub fn object_id(self, this: impl Into<RemoteObjectId>) -> Self {
        let mut params = self.params;
        params.object_id = Some(this.into());
        Self {
            tab: self.tab,
            params,
            signature: PhantomData,
        }
    }

    pub fn arguments_type<AA: Args>(self) -> Function<AA, R> {
        Function {
            tab: self.tab,
            params: self.params,
            signature: PhantomData,
        }
    }

    pub fn return_type<RR: Js2Rust>(self) -> Function<A, RR> {
        Function {
            tab: self.tab,
            params: self.params,
            signature: PhantomData,
        }
    }

    async fn call_impl(&self, args: Vec<Rust2JsValue>) -> Result<R, CdpError> {
        let mut params = self.params.clone();
        if !args.is_empty() {
            params.arguments = Some(args.into_iter().map(|a| a.into()).collect());
        }
        let value: Js2RustValue = self.tab.evaluate_function(params).await?.try_into()?;
        Ok(R::from_value(value)?)
    }
}

impl<R: Js2Rust> Function<(), R> {
    pub async fn call(&self) -> Result<R, CdpError> {
        self.call_impl(vec![]).await
    }
}

pub trait Args {}
impl Args for () {}
impl<A1: Rust2Js> Args for (A1,) {}
impl<A1: Rust2Js, A2: Rust2Js> Args for (A1, A2) {}
impl<A1: Rust2Js, A2: Rust2Js, A3: Rust2Js> Args for (A1, A2, A3) {}
impl<A1: Rust2Js, A2: Rust2Js, A3: Rust2Js, A4: Rust2Js> Args for (A1, A2, A3, A4) {}
impl<A1: Rust2Js, A2: Rust2Js, A3: Rust2Js, A4: Rust2Js, A5: Rust2Js> Args for (A1, A2, A3, A4, A5) {}
impl<A1: Rust2Js, A2: Rust2Js, A3: Rust2Js, A4: Rust2Js, A5: Rust2Js, A6: Rust2Js> Args for (A1, A2, A3, A4, A5, A6) {}
impl<A1: Rust2Js, A2: Rust2Js, A3: Rust2Js, A4: Rust2Js, A5: Rust2Js, A6: Rust2Js, A7: Rust2Js> Args for (A1, A2, A3, A4, A5, A6, A7) {}
impl<A1: Rust2Js, A2: Rust2Js, A3: Rust2Js, A4: Rust2Js, A5: Rust2Js, A6: Rust2Js, A7: Rust2Js, A8: Rust2Js> Args for (A1, A2, A3, A4, A5, A6, A7, A8) {}
impl<A1: Rust2Js, A2: Rust2Js, A3: Rust2Js, A4: Rust2Js, A5: Rust2Js, A6: Rust2Js, A7: Rust2Js, A8: Rust2Js, A9: Rust2Js> Args for (A1, A2, A3, A4, A5, A6, A7, A8, A9) {}
impl<A1: Rust2Js, A2: Rust2Js, A3: Rust2Js, A4: Rust2Js, A5: Rust2Js, A6: Rust2Js, A7: Rust2Js, A8: Rust2Js, A9: Rust2Js, A10: Rust2Js> Args for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10) {}



#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    async fn test_function_call(page: Arc<PageInner>) {
        let func = Function::<(), Value>::new(page, "function(a, b) { return a + b; }")
            .arguments_type::<(i32, String)>()  
            .return_type::<String>();
        let result = func.call(1, "hello".to_string()).await.unwrap();
        assert_eq!(result, "hello1");
    }
}
