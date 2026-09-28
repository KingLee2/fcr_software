#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__msg__TFArray() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__msg__TFArray__init(msg: *mut TFArray) -> bool;
    fn tf2_web_republisher_interfaces__msg__TFArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFArray>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__msg__TFArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFArray>);
    fn tf2_web_republisher_interfaces__msg__TFArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFArray>, out_seq: *mut rosidl_runtime_rs::Sequence<TFArray>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__msg__TFArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub transforms: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::TransformStamped>,

}



impl Default for TFArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__msg__TFArray__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__msg__TFArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__msg__TFArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__msg__TFArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__msg__TFArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFArray where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/msg/TFArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__msg__TFArray() }
  }
}


