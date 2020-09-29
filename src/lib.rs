mod timer;

use nodejs_sys::{
  napi_callback_info, napi_create_function, napi_env, napi_set_named_property, napi_value,
};

use std::ffi::CString;
use timer::nd_sleep::nd_sleep;

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
  env: napi_env,
  exports: napi_value,
) -> nodejs_sys::napi_value {
  create_function(env, exports, "sleep", nd_sleep);
  exports
}

type CallbackFn = unsafe extern "C" fn(napi_env, napi_callback_info) -> napi_value;

unsafe fn create_function(env: napi_env, exports: napi_value, name: &str, func: CallbackFn) {
  let cname = CString::new(name).expect("CString::new failed");
  let mut result: napi_value = std::mem::zeroed();
  napi_create_function(
    env,
    cname.as_ptr(),
    cname.as_bytes().len(),
    Some(func),
    std::ptr::null_mut(),
    &mut result,
  );
  napi_set_named_property(env, exports, cname.as_ptr(), result);
}
