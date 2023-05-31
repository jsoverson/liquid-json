use std::sync::Arc;

use liquid_core::{parser, Language};
use liquid_lib::stdlib;

#[derive(Default)]
pub(crate) struct OptionsBuilder {
    blocks: parser::PluginRegistry<Box<dyn parser::ParseBlock>>,
    tags: parser::PluginRegistry<Box<dyn parser::ParseTag>>,
    filters: parser::PluginRegistry<Box<dyn parser::ParseFilter>>,
}

impl OptionsBuilder {
    /// Create an empty Liquid parser
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl OptionsBuilder {
    /// Create a Liquid parser with built-in Liquid features
    pub(crate) fn stdlib(self) -> Self {
        self.tag(stdlib::AssignTag)
            .tag(stdlib::BreakTag)
            .tag(stdlib::ContinueTag)
            .tag(stdlib::CycleTag)
            .tag(stdlib::IncludeTag)
            .tag(stdlib::IncrementTag)
            .tag(stdlib::DecrementTag)
            .block(stdlib::RawBlock)
            .block(stdlib::IfBlock)
            .block(stdlib::UnlessBlock)
            .block(stdlib::IfChangedBlock)
            .block(stdlib::ForBlock)
            .block(stdlib::TableRowBlock)
            .block(stdlib::CommentBlock)
            .block(stdlib::CaptureBlock)
            .block(stdlib::CaseBlock)
            .filter(stdlib::Abs)
            .filter(stdlib::Append)
            .filter(stdlib::AtLeast)
            .filter(stdlib::AtMost)
            .filter(stdlib::Capitalize)
            .filter(stdlib::Ceil)
            .filter(stdlib::Compact)
            .filter(stdlib::Concat)
            .filter(stdlib::Date)
            .filter(stdlib::Default)
            .filter(stdlib::DividedBy)
            .filter(stdlib::Downcase)
            .filter(stdlib::Escape)
            .filter(stdlib::EscapeOnce)
            .filter(stdlib::First)
            .filter(stdlib::Floor)
            .filter(stdlib::Join)
            .filter(stdlib::Last)
            .filter(stdlib::Lstrip)
            .filter(stdlib::Map)
            .filter(stdlib::Minus)
            .filter(stdlib::Modulo)
            .filter(stdlib::NewlineToBr)
            .filter(stdlib::Plus)
            .filter(stdlib::Prepend)
            .filter(stdlib::Remove)
            .filter(stdlib::RemoveFirst)
            .filter(stdlib::Replace)
            .filter(stdlib::ReplaceFirst)
            .filter(stdlib::Reverse)
            .filter(stdlib::Round)
            .filter(stdlib::Rstrip)
            .filter(stdlib::Size)
            .filter(stdlib::Slice)
            .filter(stdlib::Sort)
            .filter(stdlib::SortNatural)
            .filter(stdlib::Split)
            .filter(stdlib::Strip)
            .filter(stdlib::StripHtml)
            .filter(stdlib::StripNewlines)
            .filter(stdlib::Times)
            .filter(stdlib::Truncate)
            .filter(stdlib::TruncateWords)
            .filter(stdlib::Uniq)
            .filter(stdlib::Upcase)
            .filter(stdlib::UrlDecode)
            .filter(stdlib::UrlEncode)
            .filter(stdlib::Where)
    }

    /// Inserts a new custom block into the parser
    pub(crate) fn block<B: Into<Box<dyn parser::ParseBlock>>>(mut self, block: B) -> Self {
        let block = block.into();
        self.blocks
            .register(block.reflection().start_tag().to_owned(), block);
        self
    }

    /// Inserts a new custom tag into the parser
    pub(crate) fn tag<T: Into<Box<dyn parser::ParseTag>>>(mut self, tag: T) -> Self {
        let tag = tag.into();
        self.tags.register(tag.reflection().tag().to_owned(), tag);
        self
    }

    /// Inserts a new custom filter into the parser
    pub(crate) fn filter<F: Into<Box<dyn parser::ParseFilter>>>(mut self, filter: F) -> Self {
        let filter = filter.into();
        self.filters
            .register(filter.reflection().name().to_owned(), filter);
        self
    }

    /// Create a parser
    pub(crate) fn build(self) -> Arc<Language> {
        let Self {
            blocks,
            tags,
            filters,
        } = self;

        let mut options = parser::Language::empty();
        options.blocks = blocks;
        options.tags = tags;
        options.filters = filters;
        Arc::new(options)
    }
}
