use crate::configs::vcs::jujutsu;
use crate::formatter::string_formatter::StringFormatterError;

use super::{format_count, Context, Segment, StringFormatter};

const ALL_STATUS_FORMAT: &str = "$deleted$renamed$modified$added";

#[must_use]
pub(super) fn segments<'a>(
    context: &Context<'a>,
    config: &jujutsu::JujutsuConfig<'a>,
) -> Option<Result<Vec<Segment>, StringFormatterError>> {
    if config.disabled {
        return None;
    }

    let repo_root = context
        .current_dir
        .ancestors()
        .find(|p| p.join(".jj").exists())?;

    // Prints something of the form:
    //
    // vnyuwku
    // M src/configs/mod.rs
    // A src/configs/vcs/jujutsu.rs
    let template = format!(
        "self.diff().summary() ++ '@ ' ++ change_id.shortest({})",
        config.change_id_length
    );
    let out = context.exec_cmd(
        "jj",
        &[
            "--repository".as_ref(),
            repo_root.as_os_str(),
            "log".as_ref(),
            "--ignore-working-copy".as_ref(),
            "--no-graph".as_ref(),
            "--color".as_ref(),
            "never".as_ref(),
            "--revisions".as_ref(),
            "@".as_ref(), // Only display the current revision
            "--template".as_ref(),
            template.as_ref(),
        ],
    )?;

    let mut added = 0;
    let mut deleted = 0;
    let mut modified = 0;
    let mut renamed = 0;
    let mut change_id = None;

    for line in out.stdout.lines() {
        if line.is_empty() {
            continue;
        }

        let (indic, rest) = line.split_once(' ')?;
        match indic {
            "A" => added += 1,
            "D" => deleted += 1,
            "M" => modified += 1,
            "R" => renamed += 1,
            "@" => change_id = Some(rest),
            _ => (),
        }
    }

    let change_id = change_id?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "all_status" => Some(ALL_STATUS_FORMAT),
                _ => None,
            })
            .map_style(|variable: &str| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| (variable == "change_id").then_some(Ok(change_id)))
            .map_variables_to_segments(|variable| {
                let segments = match variable {
                    "added" => format_count(config.added, "jujutsu.added", context, added),
                    "deleted" => format_count(config.deleted, "jujutsu.deleted", context, deleted),
                    "modified" => {
                        format_count(config.modified, "jujutsu.modified", context, modified)
                    }
                    "renamed" => format_count(config.renamed, "jujutsu.renamed", context, renamed),
                    _ => None,
                };
                segments.map(Ok)
            })
            .parse(None, Some(context))
    });

    Some(parsed)
}
