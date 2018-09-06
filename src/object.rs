use jni_sys::*;

#[derive(Debug)]
pub struct Object(jobject);

impl Object {
  pub(crate) fn from_handle(environment: &::Environment, handle: jobject) -> Object {
    return Object(environment.retain(handle));
  }

  pub(crate) fn from_unowned_handle(environment: &::Environment, handle: jobject) -> Object {
    return Object(environment.retain_unowned(handle));
  }

  pub(crate) fn as_handle(&self) -> jobject {
    return self.0;
  }

  pub unsafe fn enter_monitor(&self) {
    let env = ::get_env().as_handle();

    match (**env).MonitorEnter.unwrap()(env, self.0) {
      0 => (),
      e => panic!("Failed to enter monitor with return value {}", e),
    }
  }

  pub unsafe fn exit_monitor(&self) {
    let env = ::get_env().as_handle();

    match (**env).MonitorExit.unwrap()(env, self.0) {
      0 => (),
      e => panic!("Failed to enter monitor with return value {}", e),
    }
  }

  pub unsafe fn call_void_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<(), Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    (**env).CallVoidMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(()),
    }
  }

  pub unsafe fn call_bool_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<bool, Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    let result =
      (**env).CallBooleanMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result != 0),
    }
  }

  pub unsafe fn call_byte_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<i8, Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    let result = (**env).CallByteMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result),
    }
  }

  pub unsafe fn call_short_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<i16, Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    let result = (**env).CallShortMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result),
    }
  }

  pub unsafe fn call_int_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<i32, Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    let result = (**env).CallIntMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result),
    }
  }

  pub unsafe fn call_long_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<i64, Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    let result = (**env).CallLongMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result),
    }
  }

  pub unsafe fn call_float_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<f32, Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    let result = (**env).CallFloatMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result),
    }
  }

  pub unsafe fn call_double_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<f64, Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    let result = (**env).CallDoubleMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result),
    }
  }

  pub unsafe fn call_object_method(
    &self,
    method: &::Method,
    arguments: &[&::Value],
  ) -> Result<Option<Object>, Object> {
    let environment = ::get_env();
    let env = environment.as_handle();

    let args: Vec<jvalue> = arguments.iter().map(|x| x.as_handle()).collect();

    let handle = (**env).CallObjectMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => if handle.is_null() {
        return Ok(None);
      } else {
        return Ok(Some(Object::from_handle(&environment, handle)));
      },
    }
  }
}

unsafe impl Send for Object {}

impl Clone for Object {
  fn clone(&self) -> Object {
    return Object::from_unowned_handle(&::get_env(), self.0);
  }
}

impl Drop for Object {
  fn drop(&mut self) {
    let env = ::get_env().as_handle();

    unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.0) };
  }
}
