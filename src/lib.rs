//! NotionPad Core: Motor de documentos e estado compartilhável entre desktop e mobile.

pub mod document;
pub mod manager;
pub mod metadata;

pub use document::{Document, EditError};
pub use manager::{TabState, Workspace};
pub use metadata::{Cover, DocMetadata, MetadataStore};
