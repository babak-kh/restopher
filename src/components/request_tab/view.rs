pub enum Focus {
    None,
    NewHeaderKey,
    NewHeaderValue,

    NewParamKey,
    NewParamValue,

    Header(usize),
    Param(usize),
    Body,
}
