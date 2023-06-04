// These might work in a future version of rust (but don't on 1.68.0-dev)
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
