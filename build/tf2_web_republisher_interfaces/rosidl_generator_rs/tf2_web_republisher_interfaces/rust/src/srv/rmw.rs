#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__srv__RepublishTFs_Request() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__srv__RepublishTFs_Request__init(msg: *mut RepublishTFs_Request) -> bool;
    fn tf2_web_republisher_interfaces__srv__RepublishTFs_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RepublishTFs_Request>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__srv__RepublishTFs_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RepublishTFs_Request>);
    fn tf2_web_republisher_interfaces__srv__RepublishTFs_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RepublishTFs_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RepublishTFs_Request>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__srv__RepublishTFs_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RepublishTFs_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub source_frames: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_frame: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angular_thres: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trans_thres: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rate: f32,

    /// tf2_web_republisher will stop publishing the topic if it has zero subscribers for this amount of time
    pub timeout: builtin_interfaces::msg::rmw::Duration,

}



impl Default for RepublishTFs_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__srv__RepublishTFs_Request__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__srv__RepublishTFs_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RepublishTFs_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__srv__RepublishTFs_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__srv__RepublishTFs_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__srv__RepublishTFs_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RepublishTFs_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RepublishTFs_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/srv/RepublishTFs_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__srv__RepublishTFs_Request() }
  }
}


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__srv__RepublishTFs_Response() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__srv__RepublishTFs_Response__init(msg: *mut RepublishTFs_Response) -> bool;
    fn tf2_web_republisher_interfaces__srv__RepublishTFs_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RepublishTFs_Response>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__srv__RepublishTFs_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RepublishTFs_Response>);
    fn tf2_web_republisher_interfaces__srv__RepublishTFs_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RepublishTFs_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RepublishTFs_Response>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__srv__RepublishTFs_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RepublishTFs_Response {
    /// a topic of type geometry_msgs/TransformStamped[] that publishes the requested transforms
    pub topic_name: rosidl_runtime_rs::String,

}



impl Default for RepublishTFs_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__srv__RepublishTFs_Response__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__srv__RepublishTFs_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RepublishTFs_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__srv__RepublishTFs_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__srv__RepublishTFs_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__srv__RepublishTFs_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RepublishTFs_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RepublishTFs_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/srv/RepublishTFs_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__srv__RepublishTFs_Response() }
  }
}






#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tf2_web_republisher_interfaces__srv__RepublishTFs() -> *const std::ffi::c_void;
}

// Corresponds to tf2_web_republisher_interfaces__srv__RepublishTFs
#[allow(missing_docs, non_camel_case_types)]
pub struct RepublishTFs;

impl rosidl_runtime_rs::Service for RepublishTFs {
    type Request = RepublishTFs_Request;
    type Response = RepublishTFs_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tf2_web_republisher_interfaces__srv__RepublishTFs() }
    }
}


