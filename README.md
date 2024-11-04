# scraper_query
[![crates.io](https://img.shields.io/crates/v/scraper_query?color=dark-green)][crate]

[crate]: https://crates.io/crates/scraper_query

`scraper_query` is a simple tool for you to query components in HTML documents with `scraper` so that you can easily do simple HTML manipulations, which are common in web crawling and web scraping and data cleaning.

## Usage

```rust
use scraper::Html;
use scraper_query::*; // use `HTMLIndex`, `Tag`, `class`, `id`
use markup5ever::interface::tree_builder::TreeSink;

let mut document = Html::parse_document(HTML);
let index = HTMLIndex::new(&document);
// find all nodes with class "foo" and "bar"
let node_ids = index.query(class("foo") & class("bar"));
// find all nodes with id "foo"
let node_ids = index.query(id("foo"));  
// find all nodes with tag "h1" and class "foo"
let node_ids = index.query(Tag::H1 & class("foo"));  // same as `Tag::H1.and(class("foo"))`
// find all nodes with tag "h1" and not class "foo"
let node_ids = index.query(Tag::H1 & (!class("foo")));
// simple manipulation
for id in node_ids {
    document.remove_from_parent(&id);
}
```

## License
[MIT](./LICENSE)