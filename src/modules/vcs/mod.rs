//! The Version Control System (VCS) module exposes informations from the currently active VCS,
//! trying them in a preconfigured order.
//!
//! This allows exposing informations from only one VCS when several are present, as can be the case
//! for [colocated Git repos in Jujutsu][coloc].
//! It also makes reusing
//!
//! [coloc]: https://martinvonz.github.io/jj/latest/git-compatibility/#co-located-jujutsugit-repos

use crate::configs::vcs::{Vcs, VcsConfig};
use crate::formatter::StringFormatter;
use crate::segment::Segment;

use super::{Context, Module, ModuleConfig};

pub mod jujutsu;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vcs");
    let config = VcsConfig::try_load(module.config);

    if config.disabled || config.order.is_empty() {
        return None;
    }

    for vcs in config.order {
        let (name, parsed) = match vcs {
            Vcs::Jujutsu => ("jujutsu", jujutsu::segments(context, &config.jujutsu)),
        };

        let Some(parsed) = parsed else {
            continue;
        };

        module.set_segments(match parsed {
            Ok(segments) if segments.is_empty() => return None,
            Ok(segments) => segments,
            Err(error) => {
                log::warn!("Error in module `vcs.{name}`:\n{error}");
                return None;
            }
        });

        return Some(module);
    }

    None
}

fn format_text<F>(
    format_str: &str,
    config_path: &str,
    context: &Context,
    mapper: F,
) -> Option<Vec<Segment>>
where
    F: Fn(&str) -> Option<String> + Send + Sync,
{
    if let Ok(formatter) = StringFormatter::new(format_str) {
        formatter
            .map(|variable| mapper(variable).map(Ok))
            .parse(None, Some(context))
            .ok()
    } else {
        log::warn!("Error parsing format string `vcs.{}`", config_path);
        None
    }
}

fn format_count(
    format_str: &str,
    config_path: &str,
    context: &Context,
    count: usize,
) -> Option<Vec<Segment>> {
    if count == 0 {
        return None;
    }

    format_text(
        format_str,
        config_path,
        context,
        |variable| match variable {
            "count" => Some(count.to_string()),
            _ => None,
        },
    )
}
