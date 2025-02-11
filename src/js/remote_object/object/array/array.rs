use super::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
    class Array extends ArrayLike inherits Object {
        static #class: "Array";

        properties: {
            length: usize;
        }

        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/at
            at<T: NativeValueFromJs>(index: usize) -> T;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat
            concat<I, T>(...values: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: NativeValueIntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/copyWithin
            copyWithin(target: usize, start: usize, end?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/entries
            entries() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every
            every(callback_fn: &Function, this?: &JsObject) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fill
            fill<T: NativeValueIntoJs>(value: T, start?: usize, end?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter
            filter(callback_fn: &Function, this?: &JsObject) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find
            find<T: NativeValueFromJs>(callback_fn: &Function, this?: &JsObject) -> Option<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex
            findIndex(callback_fn: &Function, this?: &JsObject) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLast
            findLast<T: NativeValueFromJs>(callback_fn: &Function, this?: &JsObject) -> Option<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLastIndex
            findLastIndex(callback_fn: &Function, this?: &JsObject) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat
            flat(depth?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap
            flatMap(callback_fn: &Function, this?: &JsObject) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach
            forEach(callback_fn: &Function, this?: &JsObject) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes
            includes<T: NativeValueIntoJs>(search_element: T, from_index?: isize) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf
            indexOf<T: NativeValueIntoJs>(search_element: T, from_index?: isize) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/join
            join(separator?: &str) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/keys
            keys() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf
            lastIndexOf<T: NativeValueIntoJs>(search_element: T, from_index?: isize) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map
            map(callback_fn: &Function, this?: &JsObject) -> Self;


            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/pop
            pop<T: NativeValueFromJs>() -> T;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/push
            push<I, T>(...elements: I) -> usize
            where
                I: IntoIterator<Item = T>,
                T: NativeValueIntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce
            reduce(callback_fn: &Function, initial_value?: &JsRemoteObject) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduceRight
            reduceRight(callback_fn: &Function, initial_value?: &JsRemoteObject) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reverse
            reverse() -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/shift
            shift<T: NativeValueFromJs>() -> Option<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice
            slice(start?: usize, end?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some
            some(callback_fn: &Function, this?: &JsObject) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort
            sort(compare_function?: &Function) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/splice
            splice(start: usize, delete_count?: usize) -> Self;
            #[rename = +withItems]
            splice<I, T>(start: usize, delete_count?: usize, ...items: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: NativeValueIntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString
            toLocaleString(locales?: &str) -> String;
            ///#[overload = withOptions]
            ///toLocaleString(locales: &str, options: JsToLocaleStringOptions) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toReversed
            toReversed() -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSorted
            toSorted(compare_function?: &Function) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSpliced
            toSpliced(start: usize, delete_count?: usize) -> Self;
            #[rename = +withItems]
            toSpliced<I, T>(start: usize, delete_count?: usize, ...items: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: NativeValueIntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/unshift
            unshift<I, T>(...elements: I) -> usize
            where
                I: IntoIterator<Item = T>,
                T: NativeValueIntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/values
            values() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/with
            with<T: NativeValueIntoJs>(index: isize, value: T) -> Self;

            /// Extension method
            toVec<T: NativeValueFromJs>() -> Vec<T> {
                return this;
            }
        }
    }
);
