use std::collections::{HashMap, HashSet};

use pulldown_cmark::escape::{escape_href, escape_html};
use pulldown_cmark::{Alignment, CodeBlockKind, CowStr, Event, LinkType, Options, Parser, Tag};
use quick_xml::events::Event as XmlEvent;
use quick_xml::Reader as XmlReader;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode, VTag, VText};

use crate::states::active_release::use_active_release_tracker;
use crate::states::manifest::{use_manifest, Manifest};
use crate::states::release_citations::use_release_citations;
use crate::try_html;
use crate::utils::irc::Irc;

#[derive(PartialEq, Properties)]
pub struct MdRenderProps {
    pub content: String,
    pub continuity: String,
}

#[derive(Debug)]
struct MdRenderer<'a, I> {
    iter: I,
    observed_tags: HashSet<&'a str>,
    release_citations: bool,
    manifest: Irc<Manifest>,
    continuity_reference: &'a str,

    table_alignments: Vec<Alignment>,
    table_in_body: bool,
    table_cell_index: usize,
    numbers: HashMap<CowStr<'a>, usize>,
    hard_collapse: bool,

    tag_buf: Vec<VTag>,
    finished: Vec<VNode>,
}
impl<'a, I> MdRenderer<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn new(
        iter: I,
        observed_tags: HashSet<&'a str>,
        release_citations: bool,
        manifest: Irc<Manifest>,
        continuity_reference: &'a str,
    ) -> Self {
        MdRenderer {
            iter,
            observed_tags,
            release_citations,
            manifest,
            continuity_reference,
            table_alignments: Default::default(),
            table_in_body: Default::default(),
            table_cell_index: Default::default(),
            numbers: Default::default(),
            hard_collapse: Default::default(),
            tag_buf: Default::default(),
            finished: Default::default(),
        }
    }

    fn start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::Paragraph => self.nest_tag(VTag::new("p")),
            Tag::Heading(level, id, classes) => {
                let mut tag = VTag::new(format!("{}", level));
                if let Some(id) = id {
                    tag.add_attribute("id", id.to_string())
                }
                let classes = classes.join(" ");
                if !classes.is_empty() {
                    tag.add_attribute("class", classes);
                }
                self.nest_tag(tag);
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                self.nest_tag(VTag::new("table"));
            }
            Tag::TableHead => {
                self.table_in_body = false;
                self.table_cell_index = 0;
                self.nest_tag(VTag::new("thead"));
                self.nest_tag(VTag::new("tr"));
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.nest_tag(VTag::new("tr"));
            }
            Tag::TableCell => {
                let mut tag = if self.table_in_body {
                    VTag::new("td")
                } else {
                    VTag::new("th")
                };
                match self.table_alignments.get(self.table_cell_index) {
                    Some(Alignment::Left) => tag.add_attribute("style", "text-align: left"),
                    Some(Alignment::Center) => tag.add_attribute("style", "text-align: center"),
                    Some(Alignment::Right) => tag.add_attribute("style", "text-align: right"),
                    _ => {}
                }
                self.nest_tag(tag);
            }
            Tag::BlockQuote => self.nest_tag(VTag::new("blockquote")),
            Tag::CodeBlock(kind) => {
                self.nest_tag(VTag::new("pre"));
                let mut code_tag = VTag::new("code");
                if let CodeBlockKind::Fenced(lang) = kind {
                    let lang = lang.split(' ').next().unwrap();
                    if !lang.is_empty() {
                        let mut escaped_lang = String::new();
                        escape_html(&mut escaped_lang, lang).unwrap();
                        let class = format!("language-{escaped_lang}");
                        code_tag.add_attribute("class", class);
                    }
                }
                self.nest_tag(code_tag);
            }
            Tag::List(Some(start)) => {
                let mut ol_tag = VTag::new("ol");
                if start != 1 {
                    ol_tag.add_attribute("start", start.to_string());
                }
                self.nest_tag(ol_tag);
            }
            Tag::List(None) => self.nest_tag(VTag::new("ul")),
            Tag::Item => self.nest_tag(VTag::new("li")),
            Tag::Emphasis => self.nest_tag(VTag::new("em")),
            Tag::Strong => self.nest_tag(VTag::new("strong")),
            Tag::Strikethrough => self.nest_tag(VTag::new("del")),
            Tag::Link(kind, dest, title) => {
                let mut a_tag = VTag::new("a");
                let mut escaped_dest = String::new();
                escape_href(&mut escaped_dest, &dest).unwrap();
                let dest = if matches!(kind, LinkType::Email) {
                    format!("mailto:{escaped_dest}")
                } else {
                    escaped_dest
                };
                a_tag.add_attribute("href", dest);
                if !title.is_empty() {
                    let mut escaped_title = String::new();
                    escape_html(&mut escaped_title, &title).unwrap();
                    a_tag.add_attribute("title", escaped_title);
                }
                self.nest_tag(a_tag);
            }
            Tag::Image(_, dest, title) => {
                let mut img_tag = VTag::new("img");
                let mut escaped_dest = String::new();
                escape_href(&mut escaped_dest, &dest).unwrap();
                img_tag.add_attribute("src", escaped_dest);
                let alt = self.raw_text();
                if !alt.is_empty() {
                    img_tag.add_attribute("alt", alt);
                }
                if !title.is_empty() {
                    let mut escaped_title = String::new();
                    escape_html(&mut escaped_title, &title).unwrap();
                    img_tag.add_attribute("title", escaped_title);
                }
                self.nest_tag(img_tag);
                self.collapse_tag();
            }
            Tag::FootnoteDefinition(name) => {
                let mut footnote_tag = VTag::new("div");
                footnote_tag.add_attribute("class", "footnote-definition");
                let mut id = String::new();
                escape_html(&mut id, &name).unwrap();
                footnote_tag.add_attribute("id", id);
                let mut sup_tag = VTag::new("sup");
                sup_tag.add_attribute("class", "footnote-definition-label");
                let len = self.numbers.len() + 1;
                let number = *self.numbers.entry(name).or_insert(len);
                sup_tag.add_child(VNode::VText(VText::new(number.to_string())));
                footnote_tag.add_child(VNode::VTag(Box::new(sup_tag)));
                self.nest_tag(footnote_tag);
            }
        }
    }

    fn nest_tag(&mut self, tag: VTag) {
        self.tag_buf.push(tag);
        self.hard_collapse = false;
    }

    fn collapse_tag(&mut self) {
        if let Some(last) = self.tag_buf.pop() {
            if !self.hard_collapse || !last.children().is_empty() {
                let node = VNode::VTag(Box::new(last));
                self.push_finished_node(node);
            }
        }
    }

    fn push_finished_node(&mut self, node: VNode) {
        if let Some(target) = self.tag_buf.last_mut() {
            self.hard_collapse = false;
            target.add_child(node);
        } else {
            self.finished.push(node);
        }
    }

    fn end_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::Heading(_, _, _)
            | Tag::Paragraph
            | Tag::BlockQuote
            | Tag::List(_)
            | Tag::Item
            | Tag::TableRow
            | Tag::Emphasis
            | Tag::Strong
            | Tag::Strikethrough
            | Tag::Link(_, _, _)
            | Tag::FootnoteDefinition(_) => self.collapse_tag(),
            Tag::Table(_) | Tag::CodeBlock(_) => {
                self.collapse_tag();
                self.collapse_tag();
            }
            Tag::TableHead => {
                self.collapse_tag();
                self.collapse_tag();
                self.nest_tag(VTag::new("tbody"));
            }
            Tag::TableCell => {
                self.collapse_tag();
                self.table_cell_index += 1;
            }
            Tag::Image(_, _, _) => {}
        }
    }

    fn raw_text(&mut self) -> String {
        let mut nest = 0;
        let mut out = String::new();
        for event in self.iter.by_ref() {
            match event {
                Event::Start(_) => nest += 1,
                Event::End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Event::Html(text) | Event::Code(text) | Event::Text(text) => {
                    escape_html(&mut out, &text).unwrap();
                }
                Event::SoftBreak | Event::HardBreak | Event::Rule => {
                    out.push(' ');
                }
                Event::FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name).or_insert(len);
                    out.push_str(&format!("[{number}]"));
                }
                Event::TaskListMarker(true) => out.push_str("[x]"),
                Event::TaskListMarker(false) => out.push_str("[ ]"),
            }
        }
        out
    }

    fn arbitrary_html(&mut self, html: CowStr) {
        let mut reader = XmlReader::from_str(&html);
        reader.check_end_names(false);
        reader.trim_text(true);
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(event) => {
                    match event {
                        XmlEvent::Start(start) => {
                            let name = to_string(start.name());
                            let mut vtag = VTag::new(name);
                            for attribute in start.attributes() {
                                let attribute = match attribute {
                                    Ok(attribute) => attribute,
                                    Err(e) => {
                                        log::error!("Malformed attribute {e}");
                                        panic!("{e:?}")
                                    }
                                };
                                let key = to_string(attribute.key);
                                let key = Box::leak(key.into_boxed_str());
                                let value = to_string(&attribute.value);
                                vtag.add_attribute(key, value);
                            }
                            self.nest_tag(vtag);
                        }
                        XmlEvent::End(end) => {
                            let name_bytes = end.name();
                            enum ShouldShow {
                                AsIs,
                                Flatten,
                                None,
                                CiteFrom(String),
                                CiteHide(String),
                            }
                            let should_show = if name_bytes.starts_with(b"x-") {
                                let release = to_string(&name_bytes[2..]);
                                if self.observed_tags.contains(&release.as_str()) {
                                    if self.release_citations {
                                        ShouldShow::CiteHide(release)
                                    } else {
                                        ShouldShow::None
                                    }
                                } else {
                                    ShouldShow::Flatten
                                }
                            } else if name_bytes.starts_with(b"o-") {
                                let release = to_string(&name_bytes[2..]);
                                if self.observed_tags.contains(&release.as_str()) {
                                    if self.release_citations {
                                        ShouldShow::CiteFrom(release)
                                    } else {
                                        ShouldShow::Flatten
                                    }
                                } else {
                                    ShouldShow::None
                                }
                            } else {
                                ShouldShow::AsIs
                            };
                            match should_show {
                                ShouldShow::AsIs => self.collapse_tag(),
                                ShouldShow::Flatten => {
                                    if let Some(last) = self.tag_buf.pop() {
                                        let tag_children: VList = last.into_children();
                                        if let Some(penultimate) = self.tag_buf.last_mut() {
                                            // currently no way to do this without cloning
                                            self.hard_collapse = false;
                                            penultimate
                                                .add_children((*tag_children).iter().cloned());
                                        } else {
                                            self.push_finished_node(VNode::VList(tag_children));
                                        }
                                    }
                                }
                                ShouldShow::None => {
                                    self.tag_buf.pop();
                                    if let Some(next_parent) = self.tag_buf.last() {
                                        self.hard_collapse = next_parent.children().is_empty();
                                    }
                                }
                                ShouldShow::CiteFrom(mut tag) => {
                                    if let Some(release) =
                                        self.manifest.release(self.continuity_reference, &tag)
                                    {
                                        tag = release.display_name().to_string()
                                    }
                                    if let Some(last) = self.tag_buf.pop() {
                                        let tag_children: VList = last.into_children();
                                        let mut span = VTag::new("span");
                                        span.add_attribute("data-cite-kind", "observed-content");
                                        span.add_attribute("data-cite-tag", tag.to_string());
                                        span.add_children((*tag_children).iter().cloned());
                                        self.hard_collapse = false;
                                        let mut span_cite = VTag::new("span");
                                        span_cite.add_attribute("data-cite-kind", "observed-tag");
                                        span_cite.add_attribute("data-cite-tag", tag.to_string());
                                        span_cite
                                            .add_child(VNode::VText(VText::new(tag.to_string())));
                                        span.add_child(VNode::VTag(Box::new(span_cite)));
                                        self.push_finished_node(VNode::VTag(Box::new(span)));
                                    }
                                }
                                ShouldShow::CiteHide(mut tag) => {
                                    if let Some(release) =
                                        self.manifest.release(self.continuity_reference, &tag)
                                    {
                                        tag = release.display_name().to_string()
                                    }
                                    if let Some(last) = self.tag_buf.pop() {
                                        let tag_children: VList = last.into_children();
                                        let mut span = VTag::new("span");
                                        span.add_attribute("data-cite-kind", "excluded-content");
                                        span.add_attribute("data-cite-tag", tag.to_string());
                                        span.add_children((*tag_children).iter().cloned());
                                        self.hard_collapse = false;
                                        let mut span_cite = VTag::new("span");
                                        span_cite.add_attribute("data-cite-kind", "excluded-tag");
                                        span_cite.add_attribute("data-cite-tag", tag.to_string());
                                        span_cite
                                            .add_child(VNode::VText(VText::new(tag.to_string())));
                                        span.add_child(VNode::VTag(Box::new(span_cite)));
                                        self.push_finished_node(VNode::VTag(Box::new(span)));
                                    }
                                }
                            }
                        }
                        XmlEvent::Empty(_) => {}
                        XmlEvent::Text(text) => {
                            let text = to_string(text.escaped());
                            self.push_finished_node(VNode::VText(VText::new(text)));
                        }
                        XmlEvent::Eof => break,
                        event => {
                            log::warn!("Unkown XML in markdown: {event:?}");
                        }
                    }
                }
                Err(e) => log::error!("xml error: {e}"),
            }
            buf.clear();
        }
    }

    fn run(&mut self) {
        while let Some(event) = self.iter.next() {
            match event {
                Event::Start(tag) => self.start_tag(tag),
                Event::End(tag) => self.end_tag(tag),
                Event::Text(text) => {
                    self.push_finished_node(VNode::VText(VText::new(text.to_string())));
                }
                Event::Code(code) => {
                    let mut tag_code = VTag::new("code");
                    tag_code.add_child(VNode::VText(VText::new(code.to_string())));
                    self.push_finished_node(VNode::VTag(Box::new(tag_code)));
                }
                Event::HardBreak => self.push_finished_node(VNode::VTag(Box::new(VTag::new("br")))),
                Event::SoftBreak => self.push_finished_node(VNode::VText(VText::new("\n"))),
                Event::Rule => self.push_finished_node(VNode::VTag(Box::new(VTag::new("hr")))),
                Event::TaskListMarker(task_list_marker) => {
                    let mut input_tag = VTag::new("input");
                    input_tag.add_attribute("disabled", "disabled");
                    input_tag.add_attribute("type", "checkbox");
                    if task_list_marker {
                        input_tag.add_attribute("checked", "checked");
                    }
                    self.push_finished_node(VNode::VTag(Box::new(input_tag)));
                }
                Event::FootnoteReference(footnote_reference) => {
                    let len = self.numbers.len() + 1;
                    let mut sup_tag = VTag::new("sup");
                    sup_tag.add_attribute("class", "footnote-reference");
                    let mut a_tag = VTag::new("a");
                    let mut href = String::new();
                    escape_html(&mut href, &footnote_reference).unwrap();
                    a_tag.add_attribute("href", href);
                    let number = *self.numbers.entry(footnote_reference).or_insert(len);
                    a_tag.add_child(VNode::VText(VText::new(number.to_string())));
                    sup_tag.add_child(VNode::VTag(Box::new(a_tag)));
                    self.push_finished_node(VNode::VTag(Box::new(sup_tag)));
                }
                Event::Html(html) => self.arbitrary_html(html),
            }
        }
    }
    fn node(mut self) -> VNode {
        while !self.tag_buf.is_empty() {
            self.collapse_tag();
        }
        VNode::VList(VList::with_children(self.finished, None))
    }
}

#[function_component]
pub fn MdRender(props: &MdRenderProps) -> Html {
    let manifest = use_manifest();
    let active_release_tracker = use_active_release_tracker();
    let release_citations = use_release_citations();

    let manifest = try_html!(manifest.opt());

    let observed_tags = active_release_tracker.observed_releases_references(&manifest);

    let parser = Parser::new_ext(&props.content, {
        // Options::ENABLE_TABLES &
        // Options::ENABLE_STRIKETHROUGH &
        // Options::ENABLE_SMART_PUNCTUATION
        Options::all()
    });

    let mut render = MdRenderer::new(
        parser,
        observed_tags,
        release_citations,
        manifest.clone(),
        &props.continuity,
    );
    render.run();

    let node = render.node();

    html! { {node} }
}

fn to_string(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(string) => string.to_string(),
        Err(e) => {
            log::error!("Invalid unicode sequence {e}");
            panic!("Invalid unicode sequence {e:?}")
        }
    }
}
