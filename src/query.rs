use crate::consts::*;
use polars::prelude::*;
use std::ops::{BitAnd, BitOr};

pub struct Query {
    expression: Expr,
}

pub struct Class(String);
pub struct Id(String);
pub struct Tag(String);

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

macro_rules! impl_and_or {
    ($typ: ty) => {
        impl<T: Into<Query>> BitAnd<T> for $typ {
            type Output = Query;

            fn bitand(self, rhs: T) -> Self::Output {
                let lhs: Query = self.into();
                let rhs: Query = rhs.into();
                lhs & rhs
            }
        }

        impl<T: Into<Query>> BitOr<T> for $typ {
            type Output = Query;

            fn bitor(self, rhs: T) -> Self::Output {
                let lhs: Query = self.into();
                let rhs: Query = rhs.into();
                lhs | rhs
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
        }
    }
}

impl_and_or!(Class);
impl_and_or!(Class, explicit);
impl_and_or!(Id);
impl_and_or!(Id, explicit);
impl_and_or!(Tag);
impl_and_or!(Tag, explicit);
impl_and_or!(Query, explicit);

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

pub fn tag(tag: impl Into<String>) -> Tag {
    let tag = tag.into();
    assert!(!tag.is_empty(), "Tag cannot be empty");
    Tag(tag)
}

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

impl Into<Query> for Tag {
    fn into(self) -> Query {
        Query {
            expression: col(TAG).eq(lit(self.0))
        }
    }
}

macro_rules! tags {
    ($name: ident) => {
        pub fn $name() -> Tag {
            Tag(stringify!($name).to_string())
        }
    };
}

tags!(h1);
tags!(h2);
tags!(h3);
tags!(h4);
tags!(h5);
tags!(h6);
tags!(p);
tags!(div);
tags!(span);
tags!(a);
tags!(img);
tags!(comment);
tags!(text);
tags!(processing_instruction);

#[cfg(test)]
mod tests {
    use crate::consts::*;
    use crate::html_index::HTMLIndex;
    use crate::query::{class, h1, id, tag};
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
        let node_ids = queryable.query(tag("h1"));
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
        let node_ids = queryable.query(h1());
        assert_eq!(node_ids.len(), 2);
        assert_eq!(get_id(&document, node_ids[0]), "1");
        assert_eq!(get_id(&document, node_ids[1]), "4");
    }
}