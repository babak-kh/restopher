pub type Section = &'static str;
pub type State = Vec<Section>;

pub const REQUESTS: Section = "requests";
pub const ENVIRONMENTS: Section = "environments";
pub const COLLECTIONS: Section = "collections";
