use std::sync::Arc;

use crate::error::{CdpError, Result};
use crate::handler::PageInner;

use super::*;

pub async fn eval_global<'a, R: FromJs>(
    page: Arc<PageInner>,
    expr: impl Into<JsExpr<'a>>,
    execution_context: Option<ExecutionContext>,
    options: EvalOptions,
) -> Result<R> {
    let mut params = EvaluateParams::builder()
        .expression(expr.into().into_inner())
        .return_by_value(false)
        .user_gesture(options.user_gesture)
        .await_promise(options.await_promise)
        .build()
        .unwrap();
    if let Some(context) = execution_context {
        match context {
            ExecutionContext::Id(id) => params.context_id = Some(id),
            ExecutionContext::UniqueId(id) => params.unique_context_id = Some(id),
        }
    }
    let resp = page.execute(params).await?.result;
    if let Some(exception) = resp.exception_details {
        return Err(CdpError::JavascriptException(Box::new(exception)));
    }
    let remote_object = resp.result;
    if let Some(json) = remote_object.value {
        Ok(
            serde::de::DeserializeSeed::deserialize(
                de::PageDeserializeSeed::new(page.clone(), std::marker::PhantomData),
                json
            )?
        )
    } else if let Ok(special) = SpecialValue::from_remote_object(&page, remote_object).await {
        match special {
            SpecialValue::Remote(remote) => {
                let remote_object = JsRemoteObject::new(page.clone(), remote);
                let evaluator = Evaluator::new_remote(
                    page.clone(),
                    remote_object,
                    options
                );
                return evaluator.eval().await;
            }
            SpecialValue::BigInt(big_int) => {
                page.invoke_function("(x) => x")
                    .argument(big_int)?
                    .invoke().await
            }
            SpecialValue::Undefined(undefined) => {
                page.invoke_function("(x) => x")
                    .argument(undefined)?
                    .invoke().await
            }
        }
    } else {
        return Err(CdpError::msg("No value found"));
    }
}
