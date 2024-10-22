pub mod html_index;
pub mod query;
pub(crate) mod consts;
pub(crate) mod utils;

pub use html_index::HTMLIndex;
pub use query::{Query, Tag, class, id};
pub use scraper;

