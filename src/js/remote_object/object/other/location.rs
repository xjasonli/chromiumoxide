use super::*;

js_remote_object!(
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Location>
    class Location extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "Location";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/ancestorOrigins>
            ancestorOrigins: Vec<String> [readonly] {
                get() {
                    return Array.from(this.ancestorOrigins);
                }
            }

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/href>
            href: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol>
            protocol: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/host>
            host: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname>
            hostname: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/port>
            port: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname>
            pathname: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/search>
            search: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/hash>
            hash: String;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/origin>
            origin: String [readonly];
        }
        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/assign>
            assign(url: &str) -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/reload>
            reload() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/Location/replace>
            replace<T: IntoJsAny>(url: T) -> ();
        }
    }
);
