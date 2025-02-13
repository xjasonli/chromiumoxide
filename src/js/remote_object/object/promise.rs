use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise
    class Promise extends Object {
        static #subtype: "promise";

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/catch
            catch(on_rejected: &JsFunction) -> Self;
            
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/finally
            finally(on_finally: &JsFunction) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/then
            then(on_fulfilled: &JsFunction, on_rejected: &JsFunction) -> Self;
        }
    }
);

impl JsPromise {
    pub fn into_future<T: FromJs>(self) -> impl futures::Future<Output = Result<T>> + Send
    {
        let evaluator = helper::Evaluator::new_remote(
            self.page(),
            self.clone(),
            EvalOptions {
                await_promise: true,
                ..Default::default()
            }
        );
        evaluator.eval()
    }
}
