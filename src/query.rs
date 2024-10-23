use crate::consts::*;
use polars::prelude::*;
use std::ops::{BitAnd, BitOr, Not};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Query {
    expression: Expr,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Class(String);
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Id(String);

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Tag {
    // html tags
    A,
    Abbr,
    Acronym,
    Address,
    Area,
    Article,
    Aside,
    Audio,
    B,
    Base,
    Bdi,
    Bdo,
    Bgsound,
    Big,
    Blockquote,
    Body,
    Br,
    Button,
    Canvas,
    Caption,
    Center,
    Cite,
    Code,
    Col,
    Colgroup,
    Content,
    Data,
    Datalist,
    Dd,
    Del,
    Details,
    Dfn,
    Dialog,
    Dir,
    Div,
    Dl,
    Dt,
    Em,
    Embed,
    Fencedframe,
    Fieldset,
    Figcaption,
    Figure,
    Font,
    Footer,
    Form,
    Frame,
    Frameset,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Head,
    Header,
    Hgroup,
    Hr,
    Html,
    I,
    Iframe,
    Image,
    Img,
    Input,
    Ins,
    Kbd,
    Label,
    Legend,
    Li,
    Link,
    Main,
    Map,
    Mark,
    Marquee,
    Math,
    Menu,
    Menuitem,
    Meta,
    Meter,
    Nav,
    Nobr,
    Noembed,
    Noframes,
    Noscript,
    Object,
    Ol,
    Optgroup,
    Option,
    Output,
    P,
    Param,
    Picture,
    Plaintext,
    Portal,
    Pre,
    Progress,
    Q,
    Rb,
    Rp,
    Rt,
    Rtc,
    Ruby,
    S,
    Samp,
    Script,
    Search,
    Section,
    Select,
    Shadow,
    Slot,
    Small,
    Source,
    Span,
    Strike,
    Strong,
    Style,
    Sub,
    Summary,
    Sup,
    Svg,
    Table,
    Tbody,
    Td,
    Template,
    Textarea,
    Tfoot,
    Th,
    Thead,
    Time,
    Title,
    Tr,
    Track,
    Tt,
    U,
    Ul,
    Var,
    Video,
    Wbr,
    Xmp,
    // extra tags
    Comment,
    Text,
    ProcessingInstruction,
}


pub fn class(class: impl Into<String>) -> Class {
    let class = class.into();
    assert!(!class.is_empty(), "Class cannot be empty");
    Class(class)
}

pub fn id(id: impl Into<String>) -> Id {
    let id = id.into();
    assert!(!id.is_empty(), "Id cannot be empty");
    Id(id)
}

impl Tag {
    fn name(&self) -> &str {
        match self {
            Tag::A => "a",
            Tag::Abbr => "abbr",
            Tag::Acronym => "acronym",
            Tag::Address => "address",
            Tag::Area => "area",
            Tag::Article => "article",
            Tag::Aside => "aside",
            Tag::Audio => "audio",
            Tag::B => "b",
            Tag::Base => "base",
            Tag::Bdi => "bdi",
            Tag::Bdo => "bdo",
            Tag::Bgsound => "bgsound",
            Tag::Big => "big",
            Tag::Blockquote => "blockquote",
            Tag::Body => "body",
            Tag::Br => "br",
            Tag::Button => "button",
            Tag::Canvas => "canvas",
            Tag::Caption => "caption",
            Tag::Center => "center",
            Tag::Cite => "cite",
            Tag::Code => "code",
            Tag::Col => "col",
            Tag::Colgroup => "colgroup",
            Tag::Content => "content",
            Tag::Data => "data",
            Tag::Datalist => "datalist",
            Tag::Dd => "dd",
            Tag::Del => "del",
            Tag::Details => "details",
            Tag::Dfn => "dfn",
            Tag::Dialog => "dialog",
            Tag::Dir => "dir",
            Tag::Div => "div",
            Tag::Dl => "dl",
            Tag::Dt => "dt",
            Tag::Em => "em",
            Tag::Embed => "embed",
            Tag::Fencedframe => "fencedframe",
            Tag::Fieldset => "fieldset",
            Tag::Figcaption => "figcaption",
            Tag::Figure => "figure",
            Tag::Font => "font",
            Tag::Footer => "footer",
            Tag::Form => "form",
            Tag::Frame => "frame",
            Tag::Frameset => "frameset",
            Tag::H1 => "h1",
            Tag::H2 => "h2",
            Tag::H3 => "h3",
            Tag::H4 => "h4",
            Tag::H5 => "h5",
            Tag::H6 => "h6",
            Tag::Head => "head",
            Tag::Header => "header",
            Tag::Hgroup => "hgroup",
            Tag::Hr => "hr",
            Tag::Html => "html",
            Tag::I => "i",
            Tag::Iframe => "iframe",
            Tag::Image => "image",
            Tag::Img => "img",
            Tag::Input => "input",
            Tag::Ins => "ins",
            Tag::Kbd => "kbd",
            Tag::Label => "label",
            Tag::Legend => "legend",
            Tag::Li => "li",
            Tag::Link => "link",
            Tag::Main => "main",
            Tag::Map => "map",
            Tag::Mark => "mark",
            Tag::Marquee => "marquee",
            Tag::Math => "math",
            Tag::Menu => "menu",
            Tag::Menuitem => "menuitem",
            Tag::Meta => "meta",
            Tag::Meter => "meter",
            Tag::Nav => "nav",
            Tag::Nobr => "nobr",
            Tag::Noembed => "noembed",
            Tag::Noframes => "noframes",
            Tag::Noscript => "noscript",
            Tag::Object => "object",
            Tag::Ol => "ol",
            Tag::Optgroup => "optgroup",
            Tag::Option => "option",
            Tag::Output => "output",
            Tag::P => "p",
            Tag::Param => "param",
            Tag::Picture => "picture",
            Tag::Plaintext => "plaintext",
            Tag::Portal => "portal",
            Tag::Pre => "pre",
            Tag::Progress => "progress",
            Tag::Q => "q",
            Tag::Rb => "rb",
            Tag::Rp => "rp",
            Tag::Rt => "rt",
            Tag::Rtc => "rtc",
            Tag::Ruby => "ruby",
            Tag::S => "s",
            Tag::Samp => "samp",
            Tag::Script => "script",
            Tag::Search => "search",
            Tag::Section => "section",
            Tag::Select => "select",
            Tag::Shadow => "shadow",
            Tag::Slot => "slot",
            Tag::Small => "small",
            Tag::Source => "source",
            Tag::Span => "span",
            Tag::Strike => "strike",
            Tag::Strong => "strong",
            Tag::Style => "style",
            Tag::Sub => "sub",
            Tag::Summary => "summary",
            Tag::Sup => "sup",
            Tag::Svg => "svg",
            Tag::Table => "table",
            Tag::Tbody => "tbody",
            Tag::Td => "td",
            Tag::Template => "template",
            Tag::Textarea => "textarea",
            Tag::Tfoot => "tfoot",
            Tag::Th => "th",
            Tag::Thead => "thead",
            Tag::Time => "time",
            Tag::Title => "title",
            Tag::Tr => "tr",
            Tag::Track => "track",
            Tag::Tt => "tt",
            Tag::U => "u",
            Tag::Ul => "ul",
            Tag::Var => "var",
            Tag::Video => "video",
            Tag::Wbr => "wbr",
            Tag::Xmp => "xmp",
            Tag::Comment => "comment",
            Tag::Text => "text",
            Tag::ProcessingInstruction => "processingInstruction",
        }
    }
}

impl Into<Query> for Tag {
    fn into(self) -> Query {
        let name = self.name();
        Query {
            expression: col(TAG).eq(lit(name))
        }
    }
}

impl Query {
    pub(crate) fn to_exp(self) -> Expr { self.expression }
}

impl<T: Into<Query>> BitAnd<T> for Query {
    type Output = Query;

    fn bitand(self, rhs: T) -> Self::Output {
        let rhs_q = rhs.into();
        Query {
            expression: self.expression.and(rhs_q.expression)
        }
    }
}

impl<T: Into<Query>> BitOr<T> for Query {
    type Output = Query;

    fn bitor(self, rhs: T) -> Self::Output {
        let rhs_q = rhs.into();
        Query {
            expression: self.expression.or(rhs_q.expression)
        }
    }
}

impl Not for Query {
    type Output = Query;

    fn not(self) -> Self::Output {
        Query {
            expression: self.expression.not()
        }
    }
}

macro_rules! impl_simple_logics {
    ($typ: ty) => {
        impl<T: Into<Query>> BitAnd<T> for $typ {
            type Output = Query;

            fn bitand(self, rhs: T) -> <Self as BitAnd>::Output {
                let lhs: Query = self.into();
                let rhs: Query = rhs.into();
                lhs & rhs
            }
        }

        impl<T: Into<Query>> BitOr<T> for $typ {
            type Output = Query;

            fn bitor(self, rhs: T) -> <Self as BitOr>::Output {
                let lhs: Query = self.into();
                let rhs: Query = rhs.into();
                lhs | rhs
            }
        }

        impl Not for $typ {
            type Output = Query;

            fn not(self) -> <Self as Not>::Output {
                let q: Query = self.into();
                !q
            }
        }
    };
    ($typ: ty, explicit) => {
        impl $typ {
            pub fn and(self, rhs: impl Into<Query>) -> Query {
                let lhs: Query = self.into();
                let rhs: Query = rhs.into();
                lhs & rhs
            }

            pub fn or(self, rhs: impl Into<Query>) -> Query {
                let lhs: Query = self.into();
                let rhs: Query = rhs.into();
                lhs | rhs
            }

            pub fn not(self) -> Query {
                let q: Query = self.into();
                !q
            }
        }
    }
}

impl_simple_logics!(Class);
impl_simple_logics!(Class, explicit);
impl_simple_logics!(Id);
impl_simple_logics!(Id, explicit);
impl_simple_logics!(Tag);
impl_simple_logics!(Tag, explicit);
impl_simple_logics!(Query, explicit);

impl Into<Query> for Class {
    fn into(self) -> Query {
        let expression = if self.0.contains(" ") {
            let mut exps: Vec<Expr> = self.0.split(' ')
                .map(|s| col(CLASS).list().contains(lit(s)))
                .collect();
            let first = exps.pop().unwrap();
            exps.into_iter().fold(first, |a, b| a.and(b))
        } else {
            col(CLASS).list().contains(lit(self.0))
        };
        Query {
            expression
        }
    }
}

impl Into<Query> for Id {
    fn into(self) -> Query {
        Query {
            expression: col(ID).eq(lit(self.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    use crate::html_index::HTMLIndex;
    use crate::query::{class, id, Tag};
    use crate::utils::u64_to_node_id;
    use ego_tree::NodeId;
    use polars::prelude::*;
    use scraper::Html;

    const HTML: &str = r#"
<!DOCTYPE html>
<meta charset="utf-8">
<title>Hello, world!</title>
<h1 class="foo" id="1">Hello, <i>world!</i></h1>
<h2 class="bar" id="2">Hello, <i>world!</i></h2>
<h3 class="foo bar" id="3">Hello, <i>world!</i></h3>
<h1 id="4">你好</h1>
"#;

    fn get_id(document: &Html, node_id: NodeId) -> &str {
        document.tree
            .get(node_id).unwrap()
            .value()
            .as_element().unwrap()
            .id().unwrap()
    }

    #[test]
    fn test_class() {
        let document = Html::parse_document(HTML);
        let queryable = HTMLIndex::new(&document);
        let ref_df = queryable.df
            .clone()
            .lazy()
            .filter(col(CLASS).list().contains(lit("foo")))
            .collect()
            .unwrap();
        assert_eq!(ref_df.height(), 2);
        let ref_node_ids: Vec<NodeId> = ref_df.column(NODE_ID)
            .unwrap()
            .u64()
            .unwrap()
            .into_no_null_iter()
            .map(u64_to_node_id)
            .collect();
        let node_ids = queryable.query(class("foo"));
        println!("{}", ref_df);
        println!("Ref NodeIDs: {:?}", ref_node_ids);
        println!("NodeIDs: {:?}", node_ids);
        assert_eq!(ref_node_ids, node_ids);
    }

    #[test]
    fn test_id() {
        let document = Html::parse_document(HTML);
        let queryable = HTMLIndex::new(&document);
        let ref_df = queryable.df
            .clone()
            .lazy()
            .filter(col(ID).eq(lit("3")))
            .collect()
            .unwrap();
        assert_eq!(ref_df.height(), 1);
        let ref_node_ids: Vec<NodeId> = ref_df.column(NODE_ID)
            .unwrap()
            .u64()
            .unwrap()
            .into_no_null_iter()
            .map(u64_to_node_id)
            .collect();
        let node_ids = queryable.query(id("3"));
        println!("{}", ref_df);
        println!("Ref NodeIDs: {:?}", ref_node_ids);
        println!("NodeIDs: {:?}", node_ids);
        assert_eq!(ref_node_ids, node_ids);
    }

    #[test]
    fn test_tag() {
        let document = Html::parse_document(HTML);
        let queryable = HTMLIndex::new(&document);
        let ref_df = queryable.df
            .clone()
            .lazy()
            .filter(col(TAG).eq(lit("h1")))
            .collect()
            .unwrap();
        assert_eq!(ref_df.height(), 2);
        let ref_node_ids: Vec<NodeId> = ref_df.column(NODE_ID)
            .unwrap()
            .u64()
            .unwrap()
            .into_no_null_iter()
            .map(u64_to_node_id)
            .collect();
        let node_ids = queryable.query(Tag::H1);
        println!("{}", ref_df);
        println!("Ref NodeIDs: {:?}", ref_node_ids);
        println!("NodeIDs: {:?}", node_ids);
        assert_eq!(ref_node_ids, node_ids);
    }

    #[test]
    fn test_query() {
        let document = Html::parse_document(HTML);
        let queryable = HTMLIndex::new(&document);
        let node_ids = queryable.query(class("foo") & class("bar"));
        assert_eq!(node_ids.len(), 1);
        let node_id = node_ids[0];
        assert_eq!(get_id(&document, node_id), "3");

        let node_ids = queryable.query(class("foo bar"));
        assert_eq!(node_ids.len(), 1);
        let node_id = node_ids[0];
        assert_eq!(get_id(&document, node_id), "3");
    }

    #[test]
    fn test_query1() {
        let document = Html::parse_document(HTML);
        let queryable = HTMLIndex::new(&document);
        let node_ids = queryable.query(class("bar") | id("3"));
        assert_eq!(node_ids.len(), 2);
        assert_eq!(get_id(&document, node_ids[0]), "2");
        assert_eq!(get_id(&document, node_ids[1]), "3");
    }

    #[test]
    fn test_query2() {
        let document = Html::parse_document(HTML);
        let queryable = HTMLIndex::new(&document);
        let node_ids = queryable.query(Tag::H1);
        assert_eq!(node_ids.len(), 2);
        assert_eq!(get_id(&document, node_ids[0]), "1");
        assert_eq!(get_id(&document, node_ids[1]), "4");
    }

    #[test]
    fn test_not() {
        let document = Html::parse_document(HTML);
        let queryable = HTMLIndex::new(&document);
        let node_ids = queryable.query(Tag::H1 & (!class("foo")));
        assert_eq!(node_ids.len(), 1);
        assert_eq!(get_id(&document, node_ids[0]), "4");
    }
}