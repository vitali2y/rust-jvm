use std;

use jni_sys::*;

#[derive(Debug)]
pub struct String(jstring);

impl String {
  pub(crate) fn from_handle(environment: &::Environment, handle: jstring) -> String {
    return String(environment.retain(handle));
  }

  pub(crate) fn as_handle(&self) -> jstring {
    return self.0;
  }

  pub unsafe fn from_object(object: ::Object) -> String {
    return std::mem::transmute(object);
  }

  pub fn from_str(string: &str) -> String {
    let environment = ::get_env();
    let env = environment.as_handle();

    let string = std::ffi::CString::new(string).unwrap();

    let handle = unsafe { (**env).NewStringUTF.unwrap()(env, string.as_ptr()) };

    return String::from_handle(&environment, handle);
  }

  pub fn to_string(&self) -> std::string::String {
    let env = ::get_env().as_handle();

    return unsafe {
      let ptr = (**env).GetStringUTFChars.unwrap()(env, self.0, std::ptr::null_mut());

      let string = match std::ffi::CStr::from_ptr(ptr).to_str() {
        Ok(_string) => _string,
        Err(_err) => {
          //println!("utf8 decode error: {:?}", std::ffi::CStr::from_ptr(ptr));
          &""
        }
      }.to_string();

      (**env).ReleaseStringUTFChars.unwrap()(env, self.0, ptr);

      string
    };
  }
}

impl Drop for String {
  fn drop(&mut self) {
    let env = ::get_env().as_handle();

    unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.0) };
  }
}
