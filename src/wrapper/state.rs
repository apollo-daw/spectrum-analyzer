use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamValue {
    F32(f32),
    I32(i32),
    Bool(bool),
    String(String)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginState {
    /// The plugin version the state was saved with.
    #[serde(default)]
    pub version: String,

    /// The plugin's parameter values. These are stored un-normalized. This means the old value
    /// will be recalled when the parameter's range gets increased.
    pub params: BTreeMap<String, ParamValue>,

    /// Arbitrary key-value pairs. This can be used to store additional state information.
    pub fields: BTreeMap<String, String>

}