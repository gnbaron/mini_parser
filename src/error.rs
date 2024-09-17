use winnow::error::{StrContext, StrContextValue};

pub fn expected(what: &'static str) -> StrContext {
    StrContext::Expected(StrContextValue::Description(what))
}
