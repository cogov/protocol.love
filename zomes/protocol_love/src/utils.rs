use hdk::prelude::*;
/// Prepends `tag` to a `ZomeApiError`.
pub fn tag_error<T>(error: String, tag: &str) -> ExternResult<T> {
  let mut error_msg = "".to_owned();
  error_msg.push_str(tag);
  error_msg.push_str(&error);
  return Err(WasmError::Guest(error_msg.to_string()));
}
/// Prepends `tag` to a `ZomeApiError` resulting from `ExternResult`.
pub fn t<T>(result: ExternResult<T>, tag: &str) -> ExternResult<T> {
  match result {
    Ok(val) => Ok(val),
    Err(error) =>
      tag_error(error.into(), tag)
  }
}
