use super::*;

/// https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsIdleRequestOptions {
    /// A deadline by which the callback must be executed
    #[serde(default)]
    pub timeout: Option<u32>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsScrollToOptions {
    /// The number of pixels to scroll by vertically
    #[serde(default)]
    pub top: Option<f64>,

    /// The number of pixels to scroll by horizontally
    #[serde(default)]
    pub left: Option<f64>,

    /// The scroll behavior
    #[serde(default)]
    pub behavior: Option<String>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsDirectoryPickerOptions {
    /// The suggested directory to open
    #[serde(default)]
    pub start_in: Option<String>,

    /// The suggested directory to open
    #[serde(default)]
    pub id: Option<String>,

    /// Whether to allow multiple selections
    #[serde(default)]
    pub mode: Option<String>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsOpenFilePickerOptions {
    /// Whether to allow multiple selections
    #[serde(default)]
    pub multiple: Option<bool>,

    /// Whether to exclude all accept types
    #[serde(default)]
    pub exclude_accept_all_option: Option<bool>,

    /// The suggested directory to open
    #[serde(default)]
    pub start_in: Option<String>,

    /// The suggested directory to open
    #[serde(default)]
    pub id: Option<String>,

    /// The types of files to show
    #[serde(default)]
    pub types: Option<Vec<JsFilePickerAcceptType>>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsSaveFilePickerOptions {
    /// Whether to exclude all accept types
    #[serde(default)]
    pub exclude_accept_all_option: Option<bool>,

    /// The suggested directory to open
    #[serde(default)]
    pub start_in: Option<String>,

    /// The suggested directory to open
    #[serde(default)]
    pub id: Option<String>,

    /// The suggested file name
    #[serde(default)]
    pub suggested_name: Option<String>,

    /// The types of files to show
    #[serde(default)]
    pub types: Option<Vec<JsFilePickerAcceptType>>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsFilePickerAcceptType {
    /// The description of the type
    #[serde(default)]
    pub description: Option<String>,

    /// The MIME types to accept
    #[serde(default)]
    pub accept: std::collections::HashMap<String, Vec<String>>,
}

/// https://developer.mozilla.org/en-US/docs/Web/API/structuredClone#parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsStructuredSerializeOptions {
    /// The transfer list
    #[serde(default)]
    pub transfer: Option<Vec<JsTransferable>>,
} 
 