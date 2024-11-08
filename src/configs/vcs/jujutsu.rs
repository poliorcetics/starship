use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct JujutsuConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
    pub style: &'a str,

    pub change_id_length: usize,

    pub added: &'a str,
    pub deleted: &'a str,
    pub modified: &'a str,
    pub renamed: &'a str,
}

impl<'a> Default for JujutsuConfig<'a> {
    fn default() -> Self {
        Self {
            disabled: false,
            format: "[\\[$change_id [$all_status](yellow)\\]]($style) ",
            style: "purple",

            // Copy the git defaults
            change_id_length: 7,

            added: "+",
            deleted: "✘",
            modified: "!",
            renamed: "»",
        }
    }
}
