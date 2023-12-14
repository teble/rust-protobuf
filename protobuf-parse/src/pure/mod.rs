//! Pure rust `.proto` file parser.

pub(crate) mod convert;
pub(crate) mod model;
pub(crate) mod parse_and_typecheck;
pub(crate) mod parse_dependencies;
mod parser;

pub use parse_and_typecheck::parse_and_typecheck_custom;
pub use parse_and_typecheck::ProtoPathResolver;
pub use parse_and_typecheck::ResolvedProtoFile;
pub use parse_dependencies::*;
