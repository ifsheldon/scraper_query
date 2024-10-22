use std::ops::{BitAnd, BitOr};
use polars::prelude::*;
use crate::consts::*;

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
    }
}

impl_and_or!(Class);
impl_and_or!(Id);
impl_and_or!(Tag);

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
        Query {
            expression: col(CLASS).list().contains(lit(self.0))
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

#[cfg(test)]
mod tests {
    use ego_tree::NodeId;
    use polars::prelude::*;
    use scraper::Html;
    use crate::consts::*;
    use crate::query::{class, id, tag};
    use crate::queryable_html::QueryableHTML;
    use crate::utils::u64_to_node_id;

    const HTML: &str = r#"
<!DOCTYPE html>
<meta charset="utf-8">
<title>Hello, world!</title>
<h1 class="foo">Hello, <i>world!</i></h1>
<h2 class="bar">Hello, <i>world!</i></h2>
<h3 class="foo bar">Hello, <i>world!</i></h3>
<h1 id="hello">你好</h1>
"#;

    #[test]
    fn test_class() {
        let document = Html::parse_document(HTML);
        let queryable = QueryableHTML::new(&document);
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
        let queryable = QueryableHTML::new(&document);
        let ref_df = queryable.df
            .clone()
            .lazy()
            .filter(col(ID).eq(lit("hello")))
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
        let node_ids = queryable.query(id("hello"));
        println!("{}", ref_df);
        println!("Ref NodeIDs: {:?}", ref_node_ids);
        println!("NodeIDs: {:?}", node_ids);
        assert_eq!(ref_node_ids, node_ids);
    }

    #[test]
    fn test_tag() {
        let document = Html::parse_document(HTML);
        let queryable = QueryableHTML::new(&document);
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
}