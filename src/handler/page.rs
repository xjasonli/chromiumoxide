use std::sync::Arc;

use chromiumoxide_cdp::cdp::IntoEventKind;
use futures::channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use futures::channel::oneshot::channel as oneshot_channel;
use futures::stream::Fuse;
use futures::{SinkExt, StreamExt};

use chromiumoxide_cdp::cdp::browser_protocol::browser::{GetVersionParams, GetVersionReturns};
use chromiumoxide_cdp::cdp::browser_protocol::dom::Rgba;
use chromiumoxide_cdp::cdp::browser_protocol::emulation::{
    ClearDeviceMetricsOverrideParams, SetDefaultBackgroundColorOverrideParams,
    SetDeviceMetricsOverrideParams,
};
use chromiumoxide_cdp::cdp::browser_protocol::input::{
    DispatchMouseEventParams, DispatchMouseEventType,
    MouseButton,
};
use chromiumoxide_cdp::cdp::browser_protocol::page::{
    FrameId, GetLayoutMetricsParams, GetLayoutMetricsReturns, Viewport,
};
use chromiumoxide_cdp::cdp::browser_protocol::target::{ActivateTargetParams, SessionId, TargetId};
use chromiumoxide_cdp::cdp::js_protocol::runtime::{
    CallFunctionOnParams, EvaluateParams, ExecutionContextId,
};
use chromiumoxide_types::{Command, CommandResponse};

use crate::cmd::{to_command_response, CommandMessage};
use crate::error::{CdpError, Result};
use crate::handler::commandfuture::CommandFuture;
use crate::handler::domworld::DOMWorldKind;
use crate::handler::httpfuture::HttpFuture;
use crate::handler::target::{GetExecutionContext, TargetMessage};
use crate::handler::target_message_future::TargetMessageFuture;
use crate::js::{self, EvaluationResult};
use crate::layout::Point;
use crate::listeners::{EventListenerRequest, EventStream};
use crate::page::ScreenshotParams;
use crate::{utils, ArcHttpRequest};

#[derive(Debug)]
pub struct PageHandle {
    pub(crate) rx: Fuse<UnboundedReceiver<TargetMessage>>,
    page: Arc<PageInner>,
}

impl PageHandle {
    pub fn new(target_id: TargetId, session_id: SessionId, opener_id: Option<TargetId>) -> Self {
        let (commands, rx) = unbounded();
        let page = PageInner {
            target_id,
            session_id,
            opener_id,
            sender: commands,
        };
        Self {
            rx: rx.fuse(),
            page: Arc::new(page),
        }
    }

    pub(crate) fn inner(&self) -> &Arc<PageInner> {
        &self.page
    }
}

#[derive(Debug)]
pub(crate) struct PageInner {
    target_id: TargetId,
    session_id: SessionId,
    opener_id: Option<TargetId>,
    sender: UnboundedSender<TargetMessage>,
}

impl PageInner {
    /// Execute a PDL command and return its response
    pub(crate) async fn execute<T: Command>(&self, cmd: T) -> Result<CommandResponse<T::Response>> {
        execute(cmd, self.sender.clone(), Some(self.session_id.clone()))?.await
    }

    pub(crate) async fn execute_no_wait<T: Command>(&self, cmd: T) -> Result<()> {
        execute(cmd, self.sender.clone(), Some(self.session_id.clone()))
            .map(|_fut| ())
    }

    /// Create a PDL command future
    pub(crate) fn command_future<T: Command>(&self, cmd: T) -> Result<CommandFuture<T>> {
        CommandFuture::new(cmd, self.sender.clone(), Some(self.session_id.clone()))
    }

    /// This creates navigation future with the final http response when the page is loaded
    pub(crate) fn wait_for_navigation(&self) -> TargetMessageFuture<ArcHttpRequest> {
        TargetMessageFuture::<ArcHttpRequest>::wait_for_navigation(self.sender.clone())
    }

    /// This creates HTTP future with navigation and responds with the final
    /// http response when the page is loaded
    pub(crate) fn http_future<T: Command>(&self, cmd: T) -> Result<HttpFuture<T>> {
        Ok(HttpFuture::new(
            self.sender.clone(),
            self.command_future(cmd)?,
        ))
    }

    /// The identifier of this page's target
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// The identifier of this page's target's session
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    /// The identifier of this page's target's opener target
    pub fn opener_id(&self) -> &Option<TargetId> {
        &self.opener_id
    }

    pub(crate) fn sender(&self) -> &UnboundedSender<TargetMessage> {
        &self.sender
    }

    pub async fn event_listener<T: IntoEventKind>(&self) -> Result<EventStream<T>> {
        let (tx, rx) = unbounded();
        self.sender()
            .clone()
            .send(TargetMessage::AddEventListener(
                EventListenerRequest::new::<T>(tx),
            ))
            .await?;

        Ok(EventStream::new(rx))
    }

    /// Activates (focuses) the target.
    pub async fn activate(&self) -> Result<&Self> {
        self.execute(ActivateTargetParams::new(self.target_id().clone()))
            .await?;
        Ok(self)
    }

    /// Version information about the browser
    pub async fn version(&self) -> Result<GetVersionReturns> {
        Ok(self.execute(GetVersionParams::default()).await?.result)
    }

    /// Moves the mouse to this point (dispatches a mouseMoved event)
    pub async fn move_mouse(&self, point: Point) -> Result<&Self> {
        self.execute(DispatchMouseEventParams::new(
            DispatchMouseEventType::MouseMoved,
            point.x,
            point.y,
        ))
        .await?;
        Ok(self)
    }

    /// Performs a mouse click event at the point's location
    pub async fn click(&self, point: Point) -> Result<&Self> {
        let cmd = DispatchMouseEventParams::builder()
            .x(point.x)
            .y(point.y)
            .button(MouseButton::Left)
            .click_count(1);

        self.move_mouse(point)
            .await?
            .execute(
                cmd.clone()
                    .r#type(DispatchMouseEventType::MousePressed)
                    .build()
                    .unwrap(),
            )
            .await?;

        self.execute(
            cmd.r#type(DispatchMouseEventType::MouseReleased)
                .build()
                .unwrap(),
        )
        .await?;
        Ok(self)
    }

//    /// This simulates pressing keys on the page.
//    ///
//    /// # Note The `input` is treated as series of `KeyDefinition`s, where each
//    /// char is inserted as a separate keystroke. So sending
//    /// `page.type_str("Enter")` will be processed as a series of single
//    /// keystrokes:  `["E", "n", "t", "e", "r"]`. To simulate pressing the
//    /// actual Enter key instead use `page.press_key(
//    /// keys::get_key_definition("Enter").unwrap())`.
//    pub async fn type_str(&self, input: impl AsRef<str>) -> Result<&Self> {
//        for c in input.as_ref().split("").filter(|s| !s.is_empty()) {
//            self.press_key(c).await?;
//        }
//        Ok(self)
//    }
//
//    /// Uses the `DispatchKeyEvent` mechanism to simulate pressing keyboard
//    /// keys.
//    pub async fn press_key(&self, key: impl AsRef<str>) -> Result<&Self> {
//        let key = key.as_ref();
//        let key_definition = keys::get_key_definition(key)
//            .ok_or_else(|| CdpError::msg(format!("Key not found: {key}")))?;
//        let mut cmd = DispatchKeyEventParams::builder();
//
//        // See https://github.com/GoogleChrome/puppeteer/blob/62da2366c65b335751896afbb0206f23c61436f1/lib/Input.js#L114-L115
//        // And https://github.com/GoogleChrome/puppeteer/blob/62da2366c65b335751896afbb0206f23c61436f1/lib/Input.js#L52
//        let key_down_event_type = if let Some(txt) = key_definition.text {
//            cmd = cmd.text(txt);
//            DispatchKeyEventType::KeyDown
//        } else if key_definition.key.len() == 1 {
//            cmd = cmd.text(key_definition.key);
//            DispatchKeyEventType::KeyDown
//        } else {
//            DispatchKeyEventType::RawKeyDown
//        };
//
//        cmd = cmd
//            .r#type(DispatchKeyEventType::KeyDown)
//            .key(key_definition.key)
//            .code(key_definition.code)
//            .windows_virtual_key_code(key_definition.key_code)
//            .native_virtual_key_code(key_definition.key_code);
//
//        self.execute(cmd.clone().r#type(key_down_event_type).build().unwrap())
//            .await?;
//        self.execute(cmd.r#type(DispatchKeyEventType::KeyUp).build().unwrap())
//            .await?;
//        Ok(self)
//    }
//
//    /// Calls function with given declaration on the remote object with the
//    /// matching id
//    pub async fn call_js_fn(
//        &self,
//        function_declaration: impl Into<String>,
//        await_promise: bool,
//        remote_object_id: RemoteObjectId,
//    ) -> Result<CallFunctionOnReturns> {
//        let resp = self
//            .execute(
//                CallFunctionOnParams::builder()
//                    .object_id(remote_object_id)
//                    .function_declaration(function_declaration)
//                    .generate_preview(true)
//                    .await_promise(await_promise)
//                    .build()
//                    .unwrap(),
//            )
//            .await?;
//        Ok(resp.result)
//    }

    pub async fn evaluate_expression(
        &self,
        evaluate: impl Into<EvaluateParams>,
    ) -> Result<EvaluationResult> {
        let mut evaluate = evaluate.into();
        if evaluate.context_id.is_none() {
            evaluate.context_id = self.execution_context().await?;
        }
        if evaluate.await_promise.is_none() {
            evaluate.await_promise = Some(true);
        }
        if evaluate.return_by_value.is_none() {
            evaluate.return_by_value = Some(true);
        }

        let resp = self.execute(evaluate).await?.result;
        if let Some(exception) = resp.exception_details {
            return Err(CdpError::JavascriptException(Box::new(exception)));
        }

        Ok(EvaluationResult::new(resp.result))
    }

    pub async fn evaluate_function(
        &self,
        evaluate: impl Into<CallFunctionOnParams>,
    ) -> Result<EvaluationResult> {
        let mut evaluate = evaluate.into();
        if evaluate.execution_context_id.is_none() {
            evaluate.execution_context_id = self.execution_context().await?;
        }
        if evaluate.await_promise.is_none() {
            evaluate.await_promise = Some(true);
        }
        if evaluate.return_by_value.is_none() {
            evaluate.return_by_value = Some(true);
        }

        let resp = self.execute(evaluate).await?.result;
        if let Some(exception) = resp.exception_details {
            return Err(CdpError::JavascriptException(Box::new(exception)));
        }
        Ok(EvaluationResult::new(resp.result))
    }

    pub async fn execution_context(&self) -> Result<Option<ExecutionContextId>> {
        self.execution_context_for_world(None, DOMWorldKind::Main)
            .await
    }

    pub async fn secondary_execution_context(&self) -> Result<Option<ExecutionContextId>> {
        self.execution_context_for_world(None, DOMWorldKind::Secondary)
            .await
    }

    pub async fn frame_execution_context(
        &self,
        frame_id: FrameId,
    ) -> Result<Option<ExecutionContextId>> {
        self.execution_context_for_world(Some(frame_id), DOMWorldKind::Main)
            .await
    }

    pub async fn frame_secondary_execution_context(
        &self,
        frame_id: FrameId,
    ) -> Result<Option<ExecutionContextId>> {
        self.execution_context_for_world(Some(frame_id), DOMWorldKind::Secondary)
            .await
    }

    pub async fn execution_context_for_world(
        &self,
        frame_id: Option<FrameId>,
        dom_world: DOMWorldKind,
    ) -> Result<Option<ExecutionContextId>> {
        let (tx, rx) = oneshot_channel();
        self.sender
            .clone()
            .send(TargetMessage::GetExecutionContext(GetExecutionContext {
                dom_world,
                frame_id,
                tx,
            }))
            .await?;
        Ok(rx.await?)
    }

    /// Returns metrics relating to the layout of the page
    pub async fn layout_metrics(&self) -> Result<GetLayoutMetricsReturns> {
        Ok(self
            .execute(GetLayoutMetricsParams::default())
            .await?
            .result)
    }

    pub async fn screenshot(&self, params: impl Into<ScreenshotParams>) -> Result<Vec<u8>> {
        self.activate().await?;
        let params = params.into();
        let full_page = params.full_page();
        let omit_background = params.omit_background();

        let mut cdp_params = params.cdp_params;

        if full_page {
            let metrics = self.layout_metrics().await?;
            let width = metrics.css_content_size.width;
            let height = metrics.css_content_size.height;

            cdp_params.clip = Some(Viewport {
                x: 0.,
                y: 0.,
                width,
                height,
                scale: 1.,
            });

            self.execute(SetDeviceMetricsOverrideParams::new(
                width as i64,
                height as i64,
                1.,
                false,
            ))
            .await?;
        }

        if omit_background {
            self.execute(SetDefaultBackgroundColorOverrideParams {
                color: Some(Rgba {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: Some(0.),
                }),
            })
            .await?;
        }

        let res = self.execute(cdp_params).await?.result;

        if omit_background {
            self.execute(SetDefaultBackgroundColorOverrideParams { color: None })
                .await?;
        }

        if full_page {
            self.execute(ClearDeviceMetricsOverrideParams {}).await?;
        }

        Ok(utils::base64::decode(&res.data)?)
    }

    pub async fn eval_global<'a, R: js::FromJs>(
        self: &Arc<Self>,
        params: impl Into<js::GlobalEvalParams<'a>>,
    ) -> Result<R> {
        let params = params.into();
        js::helper::eval_global(
            self.clone(),
            params.expr,
            params.execution_context,
            params.options
        ).await
    }

    pub async fn eval<'a, T: js::FromJs>(
        self: &Arc<Self>,
        params: impl Into<js::ScopedEvalParams<'a>>,
    ) -> Result<T> {
        let params = params.into();
        let evaluator = js::helper::Evaluator::new_expr(
            self.clone(),
            params.expr,
            params.expr_context,
            params.options
        );
        evaluator.eval().await
    }

    pub fn invoke_function<'a>(
        self: &Arc<Self>,
        params: impl Into<js::ScopedEvalParams<'a>>,
    ) -> js::FunctionInvoker<'a> {
        let params = params.into();
        let evaluator = js::helper::Evaluator::new_expr(
            self.clone(),
            params.expr,
            params.expr_context,
            params.options
        );
        evaluator.invoke()
    }

    pub async fn expose_function<'f, F, M, E, R, A>(
        self: &Arc<Self>,
        name: impl Into<String>,
        function: F,
    ) -> Result<js::ExposedFunction<'f>>
    where 
        F: js::ExposableFn<M, E, R, A> + 'f,
        M: 'f,
        E: js::ExposableFnError + 'f,
        R: js::IntoJs + 'f,
        for<'a> A: js::FromJsArgs + 'a,
    {
        js::ExposedFunction::new(
            name.into(),
            self.clone(),
            function
        ).await
    }
}

/*
async fn execute<T: Command>(
    cmd: T,
    mut sender: UnboundedSender<TargetMessage>,
    session: Option<SessionId>,
) -> Result<CommandResponse<T::Response>> {
    let (tx, rx) = oneshot_channel();
    let method = cmd.identifier();
    let msg = CommandMessage::with_session(cmd, tx, session)?;

    sender.send(TargetMessage::Command(msg)).await?;
    let resp = rx.await??;
    to_command_response::<T>(resp, method)
}
*/

fn execute<T: Command>(
    cmd: T,
    mut sender: UnboundedSender<TargetMessage>,
    session: Option<SessionId>,
) -> Result<impl futures::Future<Output = Result<CommandResponse<T::Response>>>> {
    let (tx, rx) = oneshot_channel();
    let method = cmd.identifier();
    let msg = CommandMessage::with_session(cmd, tx, session)?;

    sender.start_send(TargetMessage::Command(msg))?;
    let fut = async move {
        let resp = rx.await??;
        to_command_response::<T>(resp, method)
    };
    Ok(fut)
}
