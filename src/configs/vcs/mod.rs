use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

pub mod jujutsu;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct VcsConfig<'a> {
    pub disabled: bool,
    pub order: IndexSet<Vcs>,
    #[serde(borrow, default)]
    pub jujutsu: jujutsu::JujutsuConfig<'a>,
}

impl<'a> Default for VcsConfig<'a> {
    fn default() -> Self {
        Self {
            disabled: false,
            order: [Vcs::Jujutsu].into_iter().collect(),
            jujutsu: jujutsu::JujutsuConfig::default(),
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(rename_all = "lowercase")]
pub enum Vcs {
    Jujutsu,
}
