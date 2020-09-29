use nodejs_sys::{
  napi_async_work, napi_callback_info, napi_create_async_work, napi_create_error,
  napi_create_promise, napi_create_string_utf8, napi_deferred, napi_delete_async_work, napi_env,
  napi_get_boolean, napi_get_cb_info, napi_queue_async_work, napi_reject_deferred,
  napi_resolve_deferred, napi_status, napi_value, napi_get_value_double
};

use std::ffi::c_void;
use std::ffi::CString;
use std::{thread, time};

#[derive(Debug, Clone)]
struct Data {
  deferred: napi_deferred,
  work: napi_async_work,
  val: u64,
  result: Option<Result<bool, String>>,
}

pub unsafe extern "C" fn nd_sleep(env: napi_env, info: napi_callback_info) -> napi_value {
  let mut buffer: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
  let mut argc = 1 as usize;
  let mut result: napi_value = std::mem::zeroed();

  std::mem::forget(buffer);

  napi_get_cb_info(
    env,
    info,
    &mut argc,
    buffer.as_mut_ptr(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
  );

  let mut start = 0 as f64;
  napi_get_value_double(env, buffer[0], &mut start);

  let mut promise: napi_value = std::mem::zeroed();
  let mut deferred: napi_deferred = std::mem::zeroed();
  let mut work_name: napi_value = std::mem::zeroed();
  let mut work: napi_async_work = std::mem::zeroed();

  let async_name = CString::new("async timer").expect("Error creating string");
  napi_create_string_utf8(
    env,
    async_name.as_ptr(),
    async_name.as_bytes().len(),
    &mut work_name,
  );

  napi_create_promise(env, &mut deferred, &mut promise);

  let v = Data {
    deferred,
    work,
    val: start as u64,
    result: None,
  };

  let data = Box::new(v);
  let raw = Box::into_raw(data);

  napi_create_async_work(
    env,
    std::ptr::null_mut(),
    work_name,
    Some(perform),
    Some(complete),
    std::mem::transmute(raw),
    &mut work,
  );

  napi_queue_async_work(env, work);
  (*raw).work = work;

  promise
}

pub unsafe extern "C" fn perform(_env: napi_env, data: *mut c_void) {
  let mut t: Box<Data> = Box::from_raw(std::mem::transmute(data));
  t.result = Some(Ok(put_to_sleep(t.val)));
  Box::into_raw(t);
}

pub unsafe extern "C" fn complete(env: napi_env, _status: napi_status, data: *mut c_void) {
  let t: Box<Data> = Box::from_raw(std::mem::transmute(data));
  let v = match t.result {
    Some(d) => match d {
      Ok(result) => result,
      Err(_) => {
        let mut js_error: napi_value = std::mem::zeroed();
        napi_create_error(
          env,
          std::ptr::null_mut(),
          std::ptr::null_mut(),
          &mut js_error,
        );
        napi_reject_deferred(env, t.deferred, js_error);
        napi_delete_async_work(env, t.work);
        return;
      }
    },
    None => {
      let mut js_error: napi_value = std::mem::zeroed();
      napi_create_error(
        env,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        &mut js_error,
      );
      napi_reject_deferred(env, t.deferred, js_error);
      napi_delete_async_work(env, t.work);
      return;
    }
  };
  let mut obj: napi_value = std::mem::zeroed();

  napi_get_boolean(env, v, &mut obj);

  napi_resolve_deferred(env, t.deferred, obj);

  napi_delete_async_work(env, t.work);
}

pub fn put_to_sleep(s: u64) -> bool {
  let seconds = time::Duration::from_secs(s);
  thread::sleep(seconds);
  println!("Sleep for {} seconds", s);
  true
}
