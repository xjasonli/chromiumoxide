use super::*;

define_js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement
    class HtmlAnchorElement extends HtmlElement inherits Element, Node, Object {
        static #class: "HTMLAnchorElement";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/attributionSrc
            attributionSrc: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download
            download: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash
            hash: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host
            host: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname
            hostname: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href
            href: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang
            hreflang: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/origin
            origin: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password
            password: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname
            pathname: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping
            ping: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port
            port: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol
            protocol: String;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy
            referrerPolicy: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel
            rel: String;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/relList
            relList: JsDomTokenList [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search
            search: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target
            target: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text
            text: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type
            #[rename = typ]
            type: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username
            username: String;
        }
    }
);
