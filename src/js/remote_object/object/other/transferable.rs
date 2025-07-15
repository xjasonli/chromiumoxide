use super::*;

/// <https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Transferable_objects>
/// 
/// Transferable objects are objects that can be transferred between different execution contexts,
/// such as the main thread and a worker thread. The following types are transferable:
/// - ArrayBuffer
/// - MessagePort
/// - ImageBitmap
/// - OffscreenCanvas
/// - VideoFrame
/// - AudioData
/// - ReadableStream
/// - WritableStream
/// - TransformStream
/// - WebTransportReceiveStream
/// - WebTransportSendStream
/// - WebTransportBidirectionalStream
/// 
/// In our case, we'll use JsObject to represent any of these types.
pub type JsTransferable = JsObject; 
 