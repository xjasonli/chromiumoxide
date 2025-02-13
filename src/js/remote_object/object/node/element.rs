use super::*;
use std::collections::HashMap;
use crate::error::Result;

pub mod html;
pub mod svg;
pub mod math_ml;

pub use html::*;
pub use svg::*;
pub use math_ml::*;

js_remote_object!(
    /// https://developer.mozilla.org/en-US/docs/Web/API/Element
    class Element extends Node inherits Object {
        static #class: "*Element";

        properties: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/assignedSlot
            assignedSlot: Option<JsHtmlElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes
            attributes: Vec<JsAttr> [readonly] {
                get() {
                    Array.from(this.attributes)
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/childElementCount
            childElementCount: usize [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/children
            children: JsHtmlCollection [readonly];

            /// Extension property
            childrenVec: Vec<JsElement> [readonly] {
                get() {
                    return Array.from(this.children);
                }
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/classList
            classList: JsObject [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/className
            className: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/clientHeight
            clientHeight: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/clientLeft
            clientLeft: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/clientTop
            clientTop: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/clientWidth
            clientWidth: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/currentCSSZoom
            #[rename = current_css_zoom]
            currentCSSZoom: f64 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/elementTiming
            elementTiming: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/firstElementChild
            firstElementChild: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/id
            id: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML
            #[rename = inner_html]
            innerHTML: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/lastElementChild
            lastElementChild: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/localName
            localName: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI
            #[rename = namespace_uri]
            namespaceURI: Option<String> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/nextElementSibling
            nextElementSibling: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML
            #[rename = outer_html]
            outerHTML: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/part
            part: JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/prefix
            prefix: Option<String> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/previousElementSibling
            previousElementSibling: Option<JsElement> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight
            scrollHeight: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft
            scrollLeft: f64;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop
            scrollTop: f64;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollWidth
            scrollWidth: u32 [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot
            shadowRoot: Option<JsShadowRoot> [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/slot
            slot: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName
            tagName: String [readonly];

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/textContent
            textContent: String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/title
            title: String;
        }
        methods: {
            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/after
            after<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/after
            #[rename = +text]
            after<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/animate
            animate(keyframes: &[JsKeyframe], duration: u32) -> JsObject;

            #[rename = +withOptions]
            animate(keyframes: &[JsKeyframe], options: &JsAnimateOptions) -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/append
            append<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/append
            #[rename = +text]
            append<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow
            attachShadow(options: &JsAttachShadowOptions) -> JsShadowRoot;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/before
            before<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/before
            #[rename = +text]
            before<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/checkVisibility
            checkVisibility(options: &JsCheckVisibilityOptions) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/closest
            closest(selectors: &str) -> Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/computedStyleMap
            computedStyleMap() -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getAnimations
            getAnimations(subtree: bool) -> Vec<JsObject>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute
            getAttribute(attribute_name: &str) -> Option<String>;
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames
            getAttributeNames() -> Vec<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode
            getAttributeNode(attr_name: &str) -> Option<JsAttr>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS
            #[rename = get_attribute_node_ns]
            getAttributeNodeNS(namespace: &str, node_name: &str) -> Option<JsAttr>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNS
            #[rename = get_attribute_ns]
            getAttributeNS(namespace: &str, name: &str) -> Option<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect
            getBoundingClientRect() -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects
            getClientRects() -> JsObject;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByClassName
            getElementsByClassName(names: &str) -> JsHtmlCollection;

            /// Extension method
            getElementsByClassNameVec(names: &str) -> Vec<JsElement> {
                return Array.from(this.getElementsByClassName(names));
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagName
            getElementsByTagName(tag_name: &str) -> JsHtmlCollection;

            /// Extension method
            getElementsByTagNameVec(tag_name: &str) -> Vec<JsElement> {
                return Array.from(this.getElementsByTagName(tag_name));
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagNameNS
            #[rename = get_elements_by_tag_name_ns]
            getElementsByTagNameNS(namespace_uri: &str, local_name: &str) -> JsHtmlCollection;

            /// Extension method
            #[rename = get_elements_by_tag_name_ns_vec]
            getElementsByTagNameNSVec(namespace_uri: &str, local_name: &str) -> Vec<JsElement> {
                return Array.from(this.getElementsByTagNameNS(namespace_uri, local_name));
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/getHTML
            #[rename = get_html]
            getHTML(options: &JsGetHTMLOptions) -> String;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute
            hasAttribute(attribute_name: &str) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributeNS
            #[rename = has_attribute_ns]
            hasAttributeNS(namespace: &str, local_name: &str) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributes
            hasAttributes() -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/hasPointerCapture
            hasPointerCapture(pointer_id: i32) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentElement
            insertAdjacentElement(position: &str, element: &JsElement) -> Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML
            #[rename = insert_adjacent_html]
            insertAdjacentHTML(position: &str, html: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentText
            insertAdjacentText(position: &str, text: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/matches
            matches(selectors: &str) -> bool;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend
            prepend<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend
            #[rename = +text]
            prepend<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector
            querySelector(selectors: &str) -> Option<JsElement>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll
            querySelectorAll(selectors: &str) -> Vec<JsElement> {
                const result = this.querySelectorAll(selectors);
                return Array.from(result);
            }

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/releasePointerCapture
            releasePointerCapture(pointer_id: i32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/remove
            remove() -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute
            removeAttribute(attribute_name: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNode
            removeAttributeNode(attr: &JsAttr) -> JsAttr;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNS
            #[rename = remove_attribute_ns]
            removeAttributeNS(namespace: &str, attr_name: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren
            replaceChildren<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren
            #[rename = +text]
            replaceChildren<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith
            replaceWith<I, T>(...nodes: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<JsNode>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith
            #[rename = +text]
            replaceWith<I, T>(...texts: I) -> ()
            where
                I: IntoIterator<Item = T>,
                T: IntoJs<String>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen
            requestFullscreen(options: &JsRequestFullscreenOptions) -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock
            requestPointerLock() -> JsPromise;
            #[rename = +withOptions]
            requestPointerLock(options: &JsRequestPointerLockOptions) -> JsPromise;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll
            scroll(x: f64, y: f64) -> ();
            #[rename = +withOptions]
            scroll(options: &JsScrollOptions) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy
            scrollBy(x: f64, y: f64) -> ();
            #[rename = +withOptions]
            scrollBy(options: &JsScrollOptions) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView
            scrollIntoView(align_to_top?: bool) -> ();
            #[rename = +withOptions]
            scrollIntoView(options: &JsScrollIntoViewOptions) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo
            scrollTo(x: f64, y: f64) -> ();
            #[rename = +withOptions]
            scrollTo(options: &JsScrollOptions) -> ();
            
            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute
            setAttribute(name: &str, value: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNode
            setAttributeNode(attr: &JsAttr) -> Option<JsAttr>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNodeNS
            #[rename = set_attribute_node_ns]
            setAttributeNodeNS(attr: &JsAttr) -> Option<JsAttr>;

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNS
            #[rename = set_attribute_ns]
            setAttributeNS(namespace: &str, name: &str, value: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/setHTMLUnsafe
            #[rename = set_html_unsafe]
            setHTMLUnsafe(html: &str) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture
            setPointerCapture(pointer_id: i32) -> ();

            /// https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute
            toggleAttribute(name: &str, force?: bool) -> bool;

            /// Extension
            queryXpath(xpath: &str) -> Option<JsNode> {
                let document = this.ownerDocument;
                let result = document.evaluate(xpath, this, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);
                for (let i = 0; i < result.snapshotLength; i++) {
                    let node = result.snapshotItem(i);
                    if (node) {
                        return node;
                    }
                }
                return null;
            }

            /// Extension
            queryXpathAll(xpath: &str) -> Vec<JsNode> {
                let document = this.ownerDocument;
                let result = document.evaluate(xpath, this, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);
                let nodes = [];
                for (let i = 0; i < result.snapshotLength; i++) {
                    let node = result.snapshotItem(i);
                    if (node) {
                        nodes.push(node);
                    }
                }
                return nodes;
            }
        }
    }
);


/// https://developer.mozilla.org/en-US/docs/Web/API/Web_Animations_API/Keyframe_Formats
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsKeyframe {
    /// The offset of the keyframe specified as a number between 0.0 and 1.0
    /// inclusive or null. This is equivalent to specifying start and end states
    /// in percentages in CSS stylesheets using @keyframes. If this value is
    /// null or missing, the keyframe will be evenly spaced between adjacent
    /// keyframes.
    #[serde(rename = "offset")]
    pub offset: Option<f64>,

    /// The easing function used from this keyframe until the next keyframe in
    /// the series.
    #[serde(rename = "easing")]
    pub easing: Option<String>,

    /// The KeyframeEffect.composite operation used to combine the values
    /// specified in this keyframe with the underlying value. This will be
    /// auto if the composite operation specified on the effect is being used.
    #[serde(rename = "composite")]
    pub composite: Option<String>,

    /// Keyframes specify property-value pairs of the CSS properties to be
    /// animated. The property names are specified using camel case so for
    /// example background-color becomes backgroundColor and background-position-x
    /// becomes backgroundPositionX. Shorthand values such as margin are also
    /// permitted.
    ///
    /// Two exceptional CSS properties are:
    ///
    /// * float, which must be written as cssFloat since "float" is a reserved
    /// word in JavaScript. It's just for reference here, this will have no
    /// effect on animation since "float" is not an animatable CSS property.
    ///
    /// * offset, which must be written as cssOffset since "offset" represents
    /// the keyframe offset as described below.
    #[serde(flatten)]
    pub properties: HashMap<String, String>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/getKeyframes#property_value_pairs
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsComputedKeyframe {
    /// The offset of the keyframe specified as a number between 0.0 and 1.0
    /// inclusive or null. This is equivalent to specifying start and end states
    /// in percentages in CSS stylesheets using @keyframes. This will be null if
    /// the keyframe is automatically spaced.
    #[serde(rename = "offset")]
    pub offset: Option<f64>,

    /// The computed offset for this keyframe, calculated when the list of
    /// computed keyframes was produced. Unlike offset, above, the computedOffset
    /// is never null.
    #[serde(rename = "computedOffset")]
    pub computed_offset: f64,

    /// The easing function used from this keyframe until the next keyframe in
    /// the series.
    #[serde(rename = "easing")]
    pub easing: Option<String>,

    /// The KeyframeEffect.composite operation used to combine the values
    /// specified in this keyframe with the underlying value. This will be
    /// absent if the composite operation specified on the effect is being used.
    #[serde(rename = "composite")]
    pub composite: Option<String>,

    /// The property value pairs for this keyframe.
    #[serde(flatten)]
    pub properties: HashMap<String, String>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect#options
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsKeyframeEffectOptions {
    /// The number of milliseconds to delay the start of the animation.
    /// Defaults to 0.
    #[serde(rename = "delay")]
    pub delay: Option<u32>,

    /// Whether the animation runs forwards (normal), backwards (reverse),
    /// switches direction after each iteration (alternate), or runs backwards
    /// and switches direction after each iteration (alternate-reverse).
    /// Defaults to "normal".
    #[serde(rename = "direction")]
    pub direction: Option<String>,

    /// The number of milliseconds each iteration of the animation takes to
    /// complete. Defaults to 0.
    #[serde(rename = "duration")]
    pub duration: Option<u32>,

    /// The rate of the animation's change over time.
    #[serde(rename = "easing")]
    pub easing: Option<String>,

    /// The number of milliseconds to delay after the end of an animation.
    /// This is primarily of use when sequencing animations based on the end
    /// time of another animation. Defaults to 0.
    #[serde(rename = "endDelay")]
    pub end_delay: Option<u32>,

    /// Dictates whether the animation's effects should be reflected by the
    /// element(s) prior to playing ("backwards"), retained after the animation
    /// has completed playing ("forwards"), or both. Defaults to "none".
    #[serde(rename = "fill")]
    pub fill: Option<String>,

    /// Describes at what point in the iteration the animation should start.
    /// 0.5 would indicate starting halfway through the first iteration for
    /// example, and with this value set, an animation with 2 iterations would
    /// end halfway through a third iteration. Defaults to 0.0.
    #[serde(rename = "iterationStart")]
    pub iteration_start: Option<f64>,

    /// The number of times the animation should repeat. Defaults to 1, and can
    /// also take a value of Infinity to make it repeat for as long as the
    /// element exists.
    #[serde(rename = "iterations")]
    pub iterations: Option<f64>,

    /// Determines how values are combined between this animation and other,
    /// separate animations that do not specify their own specific composite
    /// operation. Defaults to replace.
    #[serde(rename = "composite")]
    pub composite: Option<String>,

    /// Determines how values build from iteration to iteration in this
    /// animation. Can be set to accumulate or replace (see above). Defaults
    /// to replace.
    #[serde(rename = "iterationComposite")]
    pub iteration_composite: Option<String>,

    /// A string containing a pseudo-element selector, such as "::before".
    /// If present, the effect is applied to the selected pseudo-element of
    /// target, rather than to target itself.
    #[serde(rename = "pseudoElement")]
    pub pseudo_element: Option<String>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/animate#options
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsAnimateOptions {
    /// A property unique to animate(): A string with which to reference the
    /// animation.
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Specifies the end of an animation's attachment range along its timeline,
    /// i.e. where along the timeline an animation will end. The JavaScript
    /// equivalent of the CSS animation-range-end property. rangeEnd can take
    /// several different value types, as follows:
    #[serde(rename = "rangeEnd")]
    pub range_end: Option<String>,

    /// Specifies the start of an animation's attachment range along its timeline,
    /// i.e. where along the timeline an animation will start. The JavaScript
    /// equivalent of the CSS animation-range-start property. rangeStart can take
    /// the same value types as rangeEnd.
    #[serde(rename = "rangeStart")]
    pub range_start: Option<String>,

    /// A property unique to animate(): The AnimationTimeline to associate with
    /// the animation. Defaults to Document.timeline. The JavaScript equivalent
    /// of the CSS animation-timeline property.
    #[serde(rename = "timeline")]
    pub timeline: Option<String>,

    /// The options for the keyframe effect.
    #[serde(flatten)]
    pub effect_options: JsKeyframeEffectOptions,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow#options
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsAttachShadowOptions {
    /// A string specifying the encapsulation mode for the shadow DOM tree.
    /// This can be one of:
    ///
    /// * open
    /// Elements of the shadow root are accessible from JavaScript outside the
    /// root, for example using Element.shadowRoot:
    /// ```js
    /// element.attachShadow({ mode: "open" });
    /// element.shadowRoot; /// Returns a ShadowRoot obj
    /// ```
    ///
    /// * closed
    /// Denies access to the node(s) of a closed shadow root from JavaScript
    ///   outside it:
    /// ```js
    /// element.attachShadow({ mode: "closed" });
    /// element.shadowRoot; /// Returns null
    /// ```
    #[serde(rename = "mode")]
    pub mode: String,

    /// A boolean that specifies whether the shadow root is clonable:
    /// When set to true, the shadow root is included in the copy of the
    /// shadow host when it is cloned with Node.cloneNode() or
    /// Document.importNode().
    #[serde(rename = "clonable")]
    pub clonable: Option<bool>,

    /// A boolean that, when set to true, specifies behavior that mitigates
    /// custom element issues around focusability. When a non-focusable part of
    /// the shadow DOM is clicked, the first focusable part is given focus,
    /// and the shadow host is given any available :focus styling.
    #[serde(rename = "delegatesFocus")]
    pub delegates_focus: Option<bool>,

    /// A boolean that, when set to true, specifies that the shadow root is
    /// serializable. If set, the shadow root may be serialized by calling the
    /// Element.getHTML() or ShadowRoot.getHTML() methods with the
    /// options.serializableShadowRoots parameter set true.
    #[serde(rename = "serializable")]
    pub serializable: Option<bool>,

    /// A string specifying the slot assignment mode for the shadow DOM tree.
    /// This can be one of:
    ///
    /// * named
    /// Elements are automatically assigned to <slot> elements within this
    /// shadow root. Any descendants of the host with a slot attribute which
    /// matches the name attribute of a <slot> within this shadow root will be
    /// assigned to that slot. Any top-level children of the host with no slot
    /// attribute will be assigned to a <slot> with no name attribute (the
    /// "default slot") if one is present.
    ///
    /// * manual
    /// Elements are not automatically assigned to <slot> elements. Instead,
    /// they must be manually assigned with HTMLSlotElement.assign().
    #[serde(rename = "slotAssignment")]
    pub slot_assignment: Option<String>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/checkVisibility
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsCheckVisibilityOptions {
    /// true to check if the element content-visibility property has(or
    /// inherits) the value auto, and it is currently skipping its rendering.
    /// false by default.
    #[serde(rename = "contentVisibilityAuto")]
    pub content_visibility_auto: Option<bool>,

    /// true to check if the element opacity property has (or inherits) a
    /// value of 0. false by default.
    #[serde(rename = "opacityProperty")]
    pub opacity_property: Option<bool>,

    /// true to check if the element is invisible due to the value of its
    /// visibility property. false by default.
    ///
    /// Note: Invisible elements include those that have visibility: hidden, and
    /// some element types that have visibility: collapse.
    #[serde(rename = "visibilityProperty")]
    pub visibility_property: Option<bool>,

    /// A historic alias for opacityProperty.
    #[serde(rename = "checkOpacity")]
    pub check_opacity: Option<bool>,

    /// A historic alias for visibilityProperty.
    #[serde(rename = "checkVisibilityCSS")]
    pub check_visibility_css: Option<bool>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/getHTML#options
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsGetHTMLOptions {
    /// A boolean value that specifies whether to include serializable shadow
    /// roots. The default value is false.
    #[serde(rename = "serializableShadowRoots")]
    pub serializable_shadow_roots: Option<bool>,

    /// A boolean value that specifies whether to include serializable
    /// shadow roots. The default value is false.
    #[serde(rename = "shadowRoots")]
    pub shadow_roots: Option<Vec<JsShadowRoot>>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen#options
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsRequestFullscreenOptions {
    /// Controls whether or not to show navigation UI while the element is in
    /// fullscreen mode. The default value is "auto", which indicates that the
    /// browser should decide what to do.
    ///
    /// "hide"
    /// The browser's navigation interface will be hidden and the entire
    /// dimensions of the screen will be allocated to the display of the element.
    ///
    /// "show"
    /// The browser will present page navigation controls and possibly other
    /// user interface; the dimensions of the element (and the perceived size
    /// of the screen) will be clamped to leave room for this user interface.
    ///
    /// "auto"
    /// The browser will choose which of the above settings to apply. This is the
    /// default value.
    #[serde(rename = "navigationUI")]
    pub navigation_ui: Option<String>,

    /// Specifies on which screen you want to put the element in fullscreen mode.
    /// This takes a ScreenDetailed object as a value, representing the chosen
    /// screen.
    #[serde(rename = "screen")]
    pub screen: Option<JsObject>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock#options
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsRequestPointerLockOptions {
    /// Disables OS-level adjustment for mouse acceleration, and accesses raw
    /// mouse input instead. The default value is false; setting it to true
    /// will disable mouse acceleration.
    #[serde(rename = "unadjustedMovement")]
    pub unadjusted_movement: Option<bool>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll#options
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsScrollOptions {
    /// Specifies the number of pixels along the Y axis to scroll the window
    /// or element.
    #[serde(rename = "top")]
    pub top: Option<f64>,

    /// Specifies the number of pixels along the X axis to scroll the window
    /// or element.
    #[serde(rename = "left")]
    pub left: Option<f64>,

    /// Determines whether scrolling is instant or animates smoothly. This
    /// option is a string which must take one of the following values:
    ///
    /// * smooth:
    /// scrolling should animate smoothly
    ///
    /// * instant:
    /// scrolling should happen instantly in a single jump
    ///
    /// * auto:
    /// scroll behavior is determined by the computed value of scroll-behavior
    #[serde(rename = "behavior")]
    pub behavior: Option<String>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView#scrollintoviewoptions
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsScrollIntoViewOptions {
    /// Determines whether scrolling is instant or animates smoothly. This
    /// option is a string which must take one of the following values:
    ///
    /// * smooth:
    /// scrolling should animate smoothly
    ///
    /// * instant:
    /// scrolling should happen instantly in a single jump
    ///
    /// * auto:
    /// scroll behavior is determined by the computed value of scroll-behavior
    #[serde(rename = "behavior")]
    pub behavior: Option<String>,

    /// Defines the vertical alignment of the element within the scrollable
    /// ancestor container. This option is a string and accepts one of the
    /// following values:
    ///
    /// * start:
    /// Aligns the element's top edge with the top of the scrollable
    /// container, making the element appear at the start of the visible area
    /// vertically.
    ///
    /// * center:
    /// Aligns the element vertically at the center of the scrollable
    /// container, positioning it in the middle of the visible area.
    ///
    /// * end:
    /// Aligns the element's bottom edge with the bottom of the scrollable
    /// container, placing the element at the end of the visible area vertically.
    ///
    /// * nearest:
    /// Scrolls the element to the nearest edge in the vertical direction. If
    /// the element is closer to the top edge of the scrollable container, it
    /// will align to the top; if it's closer to the bottom edge, it will align
    /// to the bottom. This minimizes the scrolling distance.
    ///
    /// Defaults to start.
    #[serde(rename = "block")]
    pub block: Option<String>,

    /// Defines the horizontal alignment of the element within the scrollable
    /// ancestor container. This option is a string and accepts one of the
    /// following values:
    ///
    /// * start:
    /// Aligns the element's left edge with the left of the scrollable
    /// container, making the element appear at the start of the visible area
    /// horizontally.
    ///
    /// * center:
    /// Aligns the element horizontally at the center of the scrollable
    /// container, positioning it in the middle of the visible area.
    ///
    /// * end:
    /// Aligns the element's right edge with the right of the scrollable
    /// container, placing the element at the end of the visible area
    /// horizontally.
    ///
    /// * nearest:
    /// Scrolls the element to the nearest edge in the horizontal direction. If
    /// the element is closer to the left edge of the scrollable container, it
    /// will align to the left; if it's closer to the right edge, it will align
    /// to the right. This minimizes the scrolling distance.
    ///
    /// Defaults to nearest.
    #[serde(rename = "inline")]
    pub inline: Option<String>,
}
