use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function
    class Function {
        static #type: "function";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/constructor
            constructor: JsFunction [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/name
            name: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/displayName
            displayName: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/length
            length: usize [readonly];

            //// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/prototype
            prototype: Option<JsObject> [readonly];
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/toString
            toString() -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/instanceOf
            instaceOf(value: &JsRemoteObject) -> bool;
        }
    }
);

impl JsFunction {
    pub fn invoke(&self, this: Option<&JsRemoteObject>, options: EvalOptions) -> FunctionInvoker {
        let evaluator = helper::Evaluator::new_remote(
            self.page(),
            self.clone(),
            options
        );
        evaluator.invoke(this)
    }
}
