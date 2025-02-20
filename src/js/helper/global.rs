use std::sync::Arc;

use chromiumoxide_cdp::cdp::js_protocol::runtime::EvaluateReturns;

use crate::error::{CdpError, Result};
use crate::handler::PageInner;

use super::*;

pub async fn eval_global<'a, R: FromJs>(
    page: Arc<PageInner>,
    expr: impl Into<JsExpr<'a>>,
    execution_context_id: Option<ExecutionContextId>,
    options: EvalOptions,
) -> Result<R> {
    let schema = {
        let mut settings = schemars::generate::SchemaSettings::default();
        settings.inline_subschemas = true;
        settings.into_generator().into_root_schema_for::<R>()
    };
    let return_mode = schema::parse_json_schema(&schema);
    let return_by_value = match return_mode {
        ReturnMode::Null => true,
        ReturnMode::Undefined => true,
        ReturnMode::ByValue => true,
        _ => false,
    };
    let execution_context_id = {
        if let Some(execution_context_id) = execution_context_id {
            execution_context_id
        } else {
            page.execution_context().await?
                .ok_or(CdpError::msg("No execution context found"))?
        }
    };

    let params = EvaluateParams::builder()
        .expression(expr.into().into_inner())
        .return_by_value(return_by_value)
        .user_gesture(options.user_gesture)
        .await_promise(options.await_promise)
        .context_id(execution_context_id)
        .build()
        .unwrap();

    let EvaluateReturns {
        result,
        exception_details,
        ..
    } = page.execute(params).await?.result;
    if let Some(exception) = exception_details {
        return Err(CdpError::JavascriptException(Box::new(exception)));
    }
    if let ReturnMode::Complex = return_mode {
        if let Some(_) = &result.object_id {
            let remote_val = JsRemoteVal::from_remote_object(&page, result).await?;
            let remote_object = JsRemoteObject::new(
                JsRemoteObjectCtx::new(page.clone(), execution_context_id),
                remote_val
            );
            let evaluator = Evaluator::new_object(
                page.clone(),
                remote_object,
                options
            );
            return evaluator.eval().await;
        }
    }

    let ctx = JsRemoteObjectCtx::new(page.clone(), execution_context_id);
    let value = parse_remote_object(page, result).await?;
    Ok(
        serde::de::DeserializeSeed::deserialize(
            de::JsDeserializeSeed::new(ctx, std::marker::PhantomData),
            value
        )?
    )
}
