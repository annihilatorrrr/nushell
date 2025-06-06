use super::{SemanticSuggestion, completion_options::NuMatcher};
use crate::{
    SuggestionKind,
    completions::{Completer, CompletionOptions},
};
use nu_protocol::{
    Span,
    engine::{Stack, StateWorkingSet},
};
use reedline::Suggestion;

pub struct AttributeCompletion;
pub struct AttributableCompletion;

impl Completer for AttributeCompletion {
    fn fetch(
        &mut self,
        working_set: &StateWorkingSet,
        _stack: &Stack,
        prefix: impl AsRef<str>,
        span: Span,
        offset: usize,
        options: &CompletionOptions,
    ) -> Vec<SemanticSuggestion> {
        let mut matcher = NuMatcher::new(prefix, options);

        let attr_commands =
            working_set.find_commands_by_predicate(|s| s.starts_with(b"attr "), true);

        for (decl_id, name, desc, ty) in attr_commands {
            let name = name.strip_prefix(b"attr ").unwrap_or(&name);
            matcher.add_semantic_suggestion(SemanticSuggestion {
                suggestion: Suggestion {
                    value: String::from_utf8_lossy(name).into_owned(),
                    description: desc,
                    span: reedline::Span {
                        start: span.start - offset,
                        end: span.end - offset,
                    },
                    append_whitespace: false,
                    ..Default::default()
                },
                kind: Some(SuggestionKind::Command(ty, Some(decl_id))),
            });
        }

        matcher.results()
    }
}

impl Completer for AttributableCompletion {
    fn fetch(
        &mut self,
        working_set: &StateWorkingSet,
        _stack: &Stack,
        prefix: impl AsRef<str>,
        span: Span,
        offset: usize,
        options: &CompletionOptions,
    ) -> Vec<SemanticSuggestion> {
        let mut matcher = NuMatcher::new(prefix, options);

        for s in ["def", "extern", "export def", "export extern"] {
            let decl_id = working_set
                .find_decl(s.as_bytes())
                .expect("internal error, builtin declaration not found");
            let cmd = working_set.get_decl(decl_id);
            matcher.add_semantic_suggestion(SemanticSuggestion {
                suggestion: Suggestion {
                    value: cmd.name().into(),
                    description: Some(cmd.description().into()),
                    span: reedline::Span {
                        start: span.start - offset,
                        end: span.end - offset,
                    },
                    append_whitespace: false,
                    ..Default::default()
                },
                kind: Some(SuggestionKind::Command(cmd.command_type(), None)),
            });
        }

        matcher.results()
    }
}
