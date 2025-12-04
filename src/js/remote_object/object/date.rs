use super::*;

// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date
js_remote_object!(
    class Date extends Object {
        static #type: "object";
        static #subtype: "date";

        methods: {
            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDate
            getDate() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay
            getDay() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getFullYear
            getFullYear() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getHours
            getHours() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMilliseconds
            getMilliseconds() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMinutes
            getMinutes() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMonth
            getMonth() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getSeconds
            getSeconds() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTime
            getTime() -> Option<u64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset
            getTimezoneOffset() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDate
            #[rename = get_utc_date]
            getUTCDate() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDay
            #[rename = get_utc_day]
            getUTCDay() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCFullYear
            #[rename = get_utc_full_year]
            getUTCFullYear() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCHours
            #[rename = get_utc_hours]
            getUTCHours() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMilliseconds
            #[rename = get_utc_milliseconds]
            getUTCMilliseconds() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMinutes
            #[rename = get_utc_minutes]
            getUTCMinutes() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMonth
            #[rename = get_utc_month]
            getUTCMonth() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCSeconds
            #[rename = get_utc_seconds]
            getUTCSeconds() -> Option<u32>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setDate
            setDate(value: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear
            setFullYear(year: i32, month?: i32, date?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours
            setHours(hour: i32, min?: i32, sec?: i32, ms?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMilliseconds
            setMilliseconds(ms: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes
            setMinutes(min: i32, sec?: i32, ms?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMonth
            setMonth(month: i32, date?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setSeconds
            setSeconds(sec: i32, ms?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setTime
            setTime(time: i64) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCDate
            #[rename = set_utc_date]
            setUTCDate(date: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear
            #[rename = set_utc_full_year]
            setUTCFullYear(year: i32, month?: i32, date?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours
            #[rename = set_utc_hours]
            setUTCHours(hour: i32, min?: i32, sec?: i32, ms?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMilliseconds
            #[rename = set_utc_milliseconds]
            setUTCMilliseconds(ms: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes
            #[rename = set_utc_minutes]
            setUTCMinutes(min: i32, sec?: i32, ms?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMonth
            #[rename = set_utc_month]
            setUTCMonth(month: i32, date?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCSeconds
            #[rename = set_utc_seconds]
            setUTCSeconds(sec: i32, ms?: i32) -> Option<i64>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toDateString
            toDateString() -> String;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString
            #[rename = to_iso_string]
            toISOString() -> String;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toJSON
            #[rename = to_json]
            toJSON() -> Option<String>;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleDateString
            toLocaleDateString(locales?: &str) -> String;
            //#[overload = withOptions]
            //toLocaleDateString(locales: &str, options: JsObject) -> String;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString
            toLocaleString(locales?: &str) -> String;
            //#[overload = withOptions]
            //toLocaleString(locales: &str, options: JsObject) -> String;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleTimeString
            toLocaleTimeString(locales?: &str) -> String;
            //#[overload = withOptions]
            //toLocaleTimeString(locales: &str, options: JsObject) -> String;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toTimeString
            toTimeString() -> String;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString
            #[rename = to_utc_string]
            toUTCString() -> String;

            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/valueOf
            valueOf() -> Option<i64>;
        }
    }
);
