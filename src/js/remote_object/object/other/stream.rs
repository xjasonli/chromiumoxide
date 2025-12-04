use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream>
    class ReadableStream extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "ReadableStream";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/locked>
            locked: bool [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel>
            cancel() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel>
            #[rename = + withReason]
            cancel(reason: impl IntoJsAny) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader>
            getReader() -> JsReadableStreamDefaultReader;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader>
            getReaderByob() -> JsReadableStreamBYOBReader {
                return this.getReader({mode: "byob"});
            }

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough>
            pipeThrough(transform: impl IntoJs<JsObject>, options?: JsStreamPipeOptions) -> JsReadableStream;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo>
            pipeTo(destination: JsWritableStream, options?: JsStreamPipeOptions) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/tee>
            tee() -> (JsReadableStream, JsReadableStream);
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/WritableStream>
    class WritableStream extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "WritableStream";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/locked>
            locked: bool [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort>
            abort(reason?: impl IntoJsAny) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/close>
            close() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/getWriter>
            getWriter() -> JsWritableStreamDefaultWriter;
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader>
    class ReadableStreamDefaultReader extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "ReadableStreamDefaultReader";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/closed>
            closed: bool [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel>
            cancel(reason?: impl IntoJsAny) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read>
            read() -> JsReadableStreamReadResult;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/releaseLock>
            releaseLock() -> ();
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader>
    class ReadableStreamBYOBReader extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "ReadableStreamBYOBReader";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/closed>
            closed: bool [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel>
            cancel(reason?: impl IntoJsAny) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read>
            read(view: impl IntoJs<JsObject>) -> JsReadableStreamReadResult;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/releaseLock>  
            releaseLock() -> ();
        }
    }
}

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter>
    class WritableStreamDefaultWriter extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "WritableStreamDefaultWriter";

        // todo
    }
}
/// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough#options>
/// 
/// The options that should be used when piping to the writable stream. Available options are:
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JsStreamPipeOptions {
    /// If this is set to true, closing the source ReadableStream will no longer cause the destination WritableStream to be closed.
    pub prevent_close: bool,

    /// If this is set to true, errors in the source ReadableStream will no longer abort the destination WritableStream.
    pub prevent_abort: bool,

    /// If this is set to true, errors in the destination WritableStream will no longer cancel the source ReadableStream.
    pub prevent_cancel: bool,

    /// If set to an AbortSignal object, ongoing pipe operations can then be aborted via the corresponding AbortController.
    pub signal: Optional<JsAbortSignal>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read#return_value>
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JsReadableStreamReadResult {
    /// A boolean value indicating whether the stream has been consumed
    pub done: bool,

    /// The next chunk in the stream
    pub value: Optional<JsObject>,
}
