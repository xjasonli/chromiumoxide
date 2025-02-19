use std::sync::Arc;

use crate::error::{CdpError, Result};
use crate::handler::PageInner;

use super::*;

pub async fn eval_global<'a, R: FromJs>(
    page: Arc<PageInner>,
    expr: impl Into<JsExpr<'a>>,
    execution_context_id: Option<ExecutionContextId>,
    options: EvalOptions,
) -> Result<R> {
    let mut params = EvaluateParams::builder()
        .expression(expr.into().into_inner())
        .return_by_value(false)
        .user_gesture(options.user_gesture)
        .await_promise(options.await_promise)
        .build()
        .unwrap();
    let execution_context_id = {
        if let Some(execution_context_id) = execution_context_id {
            execution_context_id
        } else {
            page.execution_context().await?
                .ok_or(CdpError::msg("No execution context found"))?
        }
    };
    params.context_id = Some(execution_context_id);

    let resp = page.execute(params).await?.result;
    if let Some(exception) = resp.exception_details {
        return Err(CdpError::JavascriptException(Box::new(exception)));
    }

    let ctx = JsRemoteObjectCtx::new(page.clone(), execution_context_id);

    let remote_object = resp.result;
    if let Some(json) = remote_object.value {
        Ok(
            serde::de::DeserializeSeed::deserialize(
                de::JsDeserializeSeed::new(ctx, std::marker::PhantomData),
                json
            )?
        )
    } else if let Ok(special) = SpecialValue::from_remote_object(&page, remote_object).await {
        match special {
            SpecialValue::Remote(remote) => {
                let remote_object = JsRemoteObject::new(ctx, remote);
                let evaluator = Evaluator::new_object(
                    page.clone(),
                    remote_object,
                    options
                );
                return evaluator.eval().await;
            }
            SpecialValue::BigInt(big_int) => {
                page.invoke_function("(x) => x")
                    .argument(big_int)
                    .invoke().await
            }
            SpecialValue::Undefined(undefined) => {
                page.invoke_function("(x) => x")
                    .argument(undefined)
                    .invoke().await
            }
        }
    } else {
        return Err(CdpError::msg("No value found"));
    }
}
