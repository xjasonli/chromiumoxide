use super::*;

pub mod objects;
pub mod array;
pub mod node;
pub mod reg_exp;
pub mod date;
pub mod map;
pub mod set;
pub mod weak_map;
pub mod weak_set;
pub mod iterator;
pub mod generator;
pub mod error;
pub mod proxy;
pub mod promise;
pub mod typed_array;
pub mod array_buffer;
pub mod data_view;
pub mod wasm_memory;
pub mod wasm_value;

pub use objects::*;
pub use array::*;
pub use node::*;
pub use reg_exp::*;
pub use date::*;
pub use map::*;
pub use set::*;
pub use weak_map::*;
pub use weak_set::*;
pub use iterator::*;
pub use generator::*;
pub use error::*;
pub use proxy::*;
pub use promise::*;
pub use typed_array::*;
pub use array_buffer::*;
pub use data_view::*;
pub use wasm_memory::*;
pub use wasm_value::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object
    class Object {
        static #type: ["object", "function"];

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/constructor
            constructor: JsFunction [readonly];
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign
            objectAssign<I, T>(...sources: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: IntoJs {
                return Object.assign(this, ...sources);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/create
            objectCreate() -> Self {
                return Object.create(this);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/create
            /// 
            #[rename = + withProperties]
            objectCreate<T: IntoJs>(properties: T) -> Self {
                return Object.create(this, properties);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperties
            /// 
            objectDefineProperties<T: IntoJs>(properties: T) -> Self {
                return Object.defineProperties(this, properties);
            }


            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty
            hasOwnProperty(key: String) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isPrototypeOf
            isPrototypeOf(value: &JsObject) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable
            propertyIsEnumerable(key: AnyOf2<&str, &JsSymbol>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
            toString() -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/valueOf
            valueOf() -> Self;
        }
    }
);

impl JsObject {
    pub fn remote_object_subtype(&self) -> JsObjectSubtype {
        self.remote_object_type()
            .object_subtype()
            .expect("JsObject is not an object")
    }
}


pub type JsPropertyDescriptor<T> = AnyOf!(JsDataDescriptor<T>, JsAccessorDescriptor);

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsDataDescriptor<T> {
    value: Option<T>,
    writable: bool,
    configurable: bool,
    enumerable: bool,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsAccessorDescriptor {    
    get: JsFunction,
    set: JsFunction,
    configurable: bool,
    enumerable: bool,
}
