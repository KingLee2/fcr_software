#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__DeviceInfo_Request__init(msg: *mut DeviceInfo_Request) -> bool;
    fn realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Request>);
    fn realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeviceInfo_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__DeviceInfo_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceInfo_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DeviceInfo_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__DeviceInfo_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__DeviceInfo_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeviceInfo_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeviceInfo_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeviceInfo_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/DeviceInfo_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__DeviceInfo_Response__init(msg: *mut DeviceInfo_Response) -> bool;
    fn realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Response>);
    fn realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeviceInfo_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__DeviceInfo_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceInfo_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub device_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_number: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_version: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub usb_type_descriptor: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_update_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sensors: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub physical_port: rosidl_runtime_rs::String,

}



impl Default for DeviceInfo_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__DeviceInfo_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__DeviceInfo_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeviceInfo_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeviceInfo_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeviceInfo_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/DeviceInfo_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__CalibConfigRead_Request__init(msg: *mut CalibConfigRead_Request) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Request>);
    fn realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalibConfigRead_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for CalibConfigRead_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__CalibConfigRead_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__CalibConfigRead_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalibConfigRead_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalibConfigRead_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalibConfigRead_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/CalibConfigRead_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__CalibConfigRead_Response__init(msg: *mut CalibConfigRead_Response) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Response>);
    fn realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalibConfigRead_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_config: rosidl_runtime_rs::String,

}



impl Default for CalibConfigRead_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__CalibConfigRead_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__CalibConfigRead_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalibConfigRead_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalibConfigRead_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalibConfigRead_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/CalibConfigRead_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Request__init(msg: *mut CalibConfigWrite_Request) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Request>);
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalibConfigWrite_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_config: rosidl_runtime_rs::String,

}



impl Default for CalibConfigWrite_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__CalibConfigWrite_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__CalibConfigWrite_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalibConfigWrite_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalibConfigWrite_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalibConfigWrite_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/CalibConfigWrite_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Response__init(msg: *mut CalibConfigWrite_Response) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Response>);
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalibConfigWrite_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,

}



impl Default for CalibConfigWrite_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__CalibConfigWrite_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__CalibConfigWrite_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalibConfigWrite_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalibConfigWrite_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalibConfigWrite_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/CalibConfigWrite_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite_Response() }
  }
}






#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__DeviceInfo
#[allow(missing_docs, non_camel_case_types)]
pub struct DeviceInfo;

impl rosidl_runtime_rs::Service for DeviceInfo {
    type Request = DeviceInfo_Request;
    type Response = DeviceInfo_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead
#[allow(missing_docs, non_camel_case_types)]
pub struct CalibConfigRead;

impl rosidl_runtime_rs::Service for CalibConfigRead {
    type Request = CalibConfigRead_Request;
    type Response = CalibConfigRead_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite
#[allow(missing_docs, non_camel_case_types)]
pub struct CalibConfigWrite;

impl rosidl_runtime_rs::Service for CalibConfigWrite {
    type Request = CalibConfigWrite_Request;
    type Response = CalibConfigWrite_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite() }
    }
}


