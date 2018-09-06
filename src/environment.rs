use std::ffi::CString;

use jni_sys::*;

#[derive(Clone, Debug)]
pub struct Environment(*mut JNIEnv);

impl Environment {
  pub(crate) fn from_handle(handle: *mut JNIEnv) -> Environment {
    return Environment(handle);
  }

  pub(crate) fn as_handle(&self) -> *mut JNIEnv {
    return self.0;
  }

  pub(crate) fn retain(&self, handle: jobject) -> jobject {
    if handle.is_null() {
      panic!("Handle was null, could not retain")
    };

    let global = unsafe { (**self.0).NewGlobalRef.unwrap()(self.0, handle) };

    if global.is_null() {
      panic!("Retaining failed")
    };

    let _ = unsafe { (**self.0).DeleteLocalRef.unwrap()(self.0, handle) };

    return global;
  }

  pub(crate) fn retain_unowned(&self, handle: jobject) -> jobject {
    if handle.is_null() {
      panic!("Handle was null, could not retain")
    };

    let global = unsafe { (**self.0).NewGlobalRef.unwrap()(self.0, handle) };

    if global.is_null() {
      panic!("Retaining failed")
    };

    return global;
  }

  pub fn ensure_local_capacity(&self, capacity: i32) -> Result<(), ::Object> {
    let result = unsafe { (**self.0).EnsureLocalCapacity.unwrap()(self.0, capacity) };

    return if result != 0 {
      match self.check_jvm_exception() {
        Some(e) => return Err(e),
        None => panic!("Error signalled, but no exception found"),
      }
    } else {
      Ok(())
    };
  }

  pub fn find_class(&self, name: &str) -> Result<::Class, ::Object> {
    let name = CString::new(name).unwrap();

    let handle = unsafe { (**self.0).FindClass.unwrap()(self.0, name.as_ptr()) };

    match self.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(::Class::from_handle(self, handle)),
    }
  }

  pub fn check_jvm_exception(&self) -> Option<::Object> {
    let exception = unsafe { (**self.0).ExceptionOccurred.unwrap()(self.0) };

    if !exception.is_null() {
      unsafe { (**self.0).ExceptionDescribe.unwrap()(self.0) };
      unsafe { (**self.0).ExceptionClear.unwrap()(self.0) };

      return Some(::Object::from_handle(self, exception));
    }

    return None;
  }
}
