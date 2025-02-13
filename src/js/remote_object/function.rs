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
    pub fn invoke(&self) -> FunctionInvoker<'static> {
        self.invoke_with_options(EvalOptions::default())
    }

    pub fn invoke_with_options(&self, options: EvalOptions) -> FunctionInvoker<'static> {
        helper::Evaluator::new_remote(
            self.page(),
            self.clone(),
            options
        ).invoke()
    }
}
