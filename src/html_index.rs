use crate::consts::*;
use crate::query::Query;
use crate::utils::{node_id_to_u64, u64_to_node_id};
use ego_tree::NodeId;
use polars::prelude::*;
use scraper::{Html, Node};

/// An index into `Html` document tree backed by polars' `DataFrame`.
#[derive(Debug, Clone)]
pub struct HTMLIndex<'html> {
    pub(crate) df: DataFrame,
    pub html: &'html Html,
}

impl<'html> HTMLIndex<'html> {
    pub fn new(html: &'html Html) -> Self {
        let node_num = html.tree.nodes().count();
        let mut node_ids = Vec::with_capacity(node_num);
        let mut classes = Vec::with_capacity(node_num);
        let mut ids = Vec::with_capacity(node_num);
        let mut tags = Vec::with_capacity(node_num);
        html.tree.nodes()
            .filter(|n| matches!(n.value(), Node::Comment(_) | Node::Text(_) | Node::Element(_) | Node::ProcessingInstruction(_)))
            .for_each(|node| {
                let node_id = node_id_to_u64(node.id());
                node_ids.push(node_id);
                let (class, id, tag) = match node.value() {
                    Node::Comment(_) => (None, None, COMMENT.to_string()),
                    Node::Text(_) => (None, None, TEXT.to_string()),
                    Node::ProcessingInstruction(_) => (None, None, PROCESSING_INSTRUCTION.to_string()),
                    Node::Element(e) => {
                        let class: Vec<_> = e.classes().map(|s| s.to_string()).collect();
                        let class = if class.is_empty() { None } else { Some(class.join(" ")) };
                        let id = e.id().map(|s| s.to_string());
                        let tag = e.name.local.to_string();
                        (class, id, tag)
                    }
                    _ => unreachable!("Already filtered")
                };
                classes.push(class);
                ids.push(id);
                tags.push(tag);
            });
        let raw_df = df! {
            NODE_ID => node_ids,
            TAG => tags,
            CLASS => classes,
            ID => ids
        }.unwrap();
        let df = raw_df
            .lazy()
            .with_column(col(CLASS).str().split(lit(" ")))
            .collect()
            .expect("failed to collect");
        Self {
            df,
            html,
        }
    }

    /// Query the index with a query expression
    pub fn query(&self, query: impl Into<Query>) -> Vec<NodeId> {
        let query = query.into();
        let df = self.df.clone()
            .lazy()
            .filter(query.to_exp())
            .select([col(NODE_ID)])
            .collect()
            .unwrap();
        let node_ids = df.column(NODE_ID).unwrap();
        let node_ids: Vec<u64> = node_ids.u64().unwrap().into_no_null_iter().collect();
        let node_ids = node_ids.into_iter().map(u64_to_node_id).collect();
        node_ids
    }
}

#[cfg(test)]
mod tests {
    use scraper::Html;

    #[test]
    fn test_convert() {
        let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

        let document = Html::parse_document(html);
        let queryable = super::HTMLIndex::new(&document);
        println!("{}", queryable.df);
    }
}