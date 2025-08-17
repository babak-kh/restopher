#[derive(Debug, Clone)]
pub enum Focus {
    /// Schema objects list
    Objects,
    /// Object fields list
    ObjectsFields,
    /// Queries list
    Queries,
    /// Query fields list
    QueriesFields,
    /// Mutations list
    Mutations,
    /// Mutation fields list
    MutationsFields,
    /// Request builder
    RequestBuilder,
    /// Field selection screen
    FieldSelection,
    /// Input for arguments
    ArgumentInput,
    /// Preview of the generated query
    ResultPreview,
}
