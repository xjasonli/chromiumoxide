use super::*;

pub mod arguments;
pub mod array;
pub mod dom_token_list;
pub mod html_collection;
pub mod node_list;

pub use arguments::*;
pub use array::*;
pub use dom_token_list::*;
pub use html_collection::*;
pub use node_list::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Indexed_collections#working_with_array-like_objects
    class ArrayLike extends Object {
        static #subtype: "array";

        properties: {
            length: usize [readonly];
        }

        methods: {
            // Extension method
            toArray() -> JsArray {
                if Array.isArray(this) {
                    return this;
                } else {
                    return Array.from(this);
                }
            }

            // Extension method
            toVec<T: NativeValueFromJs>() -> Vec<T> {
                if Array.isArray(this) {
                    return this;
                } else {
                    return Array.from(this);
                }
            }
        }
    }
);

impl JsArrayLike {
    pub async fn get_element<T>(&self, index: usize) -> Result<T>
    where
        T: NativeValueFromJs,
    {
        self.get_property(format!("{}", index)).await
    }

    pub async fn set_element<T>(&self, index: usize, value: T) -> Result<()>
    where
        T: NativeValueIntoJs,
    {
        self.set_property(format!("{}", index), value).await
    }
}
