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
            at<T: FromJs>(index: usize) -> T;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat
            concat<I, T>(...values: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: IntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/copyWithin
            copyWithin(target: usize, start: usize, end?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/entries
            entries() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every
            every(callback_fn: impl IntoJs<JsFunction>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every
            #[rename = + withThis]
            every(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fill
            fill<T: IntoJs>(value: T, start?: usize, end?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter
            filter(callback_fn: impl IntoJs<JsFunction>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter
            #[rename = + withThis]
            filter(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find
            find<T: FromJs>(callback_fn: impl IntoJs<JsFunction>) -> Option<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find
            #[rename = + withThis]
            find<T: FromJs>(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> Option<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex
            findIndex(callback_fn: impl IntoJs<JsFunction>) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex
            #[rename = + withThis]
            findIndex(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLast
            findLast<T: FromJs>(callback_fn: impl IntoJs<JsFunction>) -> Option<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLast
            #[rename = + withThis]
            findLast<T: FromJs>(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> Option<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLastIndex
            findLastIndex(callback_fn: impl IntoJs<JsFunction>) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLastIndex
            #[rename = + withThis]
            findLastIndex(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat
            flat(depth?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap
            flatMap(callback_fn: impl IntoJs<JsFunction>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap
            #[rename = + withThis]
            flatMap(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach
            forEach(callback_fn: impl IntoJs<JsFunction>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach
            #[rename = + withThis]
            forEach(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes
            includes<T: IntoJs>(search_element: T, from_index?: isize) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf
            indexOf<T: IntoJs>(search_element: T, from_index?: isize) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/join
            join(separator?: &str) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/keys
            keys() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf
            lastIndexOf<T: IntoJs>(search_element: T, from_index?: isize) -> isize;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map
            map(callback_fn: impl IntoJs<JsFunction>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map
            #[rename = + withThis]
            map(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/pop
            pop<T: FromJs>() -> T;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/push
            push<I, T>(...elements: I) -> usize
            where
                I: IntoIterator<Item = T>,
                T: IntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce
            reduce(callback_fn: impl IntoJs<JsFunction>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce
            #[rename = + withInitial]
            reduce(callback_fn: impl IntoJs<JsFunction>, initial_value: impl IntoJs<JsRemoteObject>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduceRight
            reduceRight(callback_fn: impl IntoJs<JsFunction>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduceRight
            #[rename = + withInitial]
            reduceRight(callback_fn: impl IntoJs<JsFunction>, initial_value: impl IntoJs<JsRemoteObject>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reverse
            reverse() -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/shift
            shift<T: FromJs>() -> Option<T>;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice
            slice(start?: usize, end?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some
            some(callback_fn: impl IntoJs<JsFunction>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some
            #[rename = + withThis]
            some(callback_fn: impl IntoJs<JsFunction>, this_arg: impl IntoJs<JsRemoteObject>) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort
            sort() -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort
            #[rename = + withCompareFn]
            sort(compare_function: impl IntoJs<JsFunction>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/splice
            splice(start: usize, delete_count?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/splice
            #[rename = +withItems]
            splice<I, T>(start: usize, delete_count?: usize, ...items: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: IntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString
            toLocaleString() -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString
            #[rename = + withLocales]
            toLocaleString(locales: impl IntoJs<str>) -> String;

            ///#[overload = + withLocaleAndOptions]
            ///toLocaleString(locales: &str, options: JsToLocaleStringOptions) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toReversed
            toReversed() -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSorted
            toSorted() -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSorted
            #[rename = + withCompareFn]
            toSorted(compare_function: impl IntoJs<JsFunction>) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSpliced
            toSpliced(start: usize, delete_count?: usize) -> Self;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSpliced
            #[rename = +withItems]
            toSpliced<I, T>(start: usize, delete_count?: usize, ...items: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: IntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/unshift
            unshift<I, T>(...elements: I) -> usize
            where
                I: IntoIterator<Item = T>,
                T: IntoJs;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/values
            values() -> JsIterator;

            /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/with
            with<T: IntoJs>(index: isize, value: T) -> Self;

            /// Extension method
            toVec<T: FromJs>() -> Vec<T> {
                return this;
            }
        }
    }
);
