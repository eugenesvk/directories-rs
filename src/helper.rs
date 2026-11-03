/// Apply an attribute to all items in a block without adding a block (to avoid scope issues)
#[macro_export] macro_rules! apply_attr {
    { #$attr:tt $($it:item)* } => { $(#$attr $it)* }  }
/// Same as `apply_attr`, but for `!attr`
#[macro_export] macro_rules! apply_attrx {
    { #!$attr:tt $($it:item)* } => { $(#$attr $it)* }  }
