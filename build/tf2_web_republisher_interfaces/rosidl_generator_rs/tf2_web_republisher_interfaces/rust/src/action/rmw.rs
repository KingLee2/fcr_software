
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__action__TFSubscription_Goal__init(msg: *mut TFSubscription_Goal) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Goal>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Goal>);
    fn tf2_web_republisher_interfaces__action__TFSubscription_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFSubscription_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Goal>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_Goal {

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

}



impl Default for TFSubscription_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__action__TFSubscription_Goal__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__action__TFSubscription_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFSubscription_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFSubscription_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/action/TFSubscription_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_Goal() }
  }
}


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_Result() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__action__TFSubscription_Result__init(msg: *mut TFSubscription_Result) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Result>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Result>);
    fn tf2_web_republisher_interfaces__action__TFSubscription_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFSubscription_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Result>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for TFSubscription_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__action__TFSubscription_Result__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__action__TFSubscription_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFSubscription_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFSubscription_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/action/TFSubscription_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_Result() }
  }
}


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__action__TFSubscription_Feedback__init(msg: *mut TFSubscription_Feedback) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Feedback>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Feedback>);
    fn tf2_web_republisher_interfaces__action__TFSubscription_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFSubscription_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_Feedback>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub transforms: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::TransformStamped>,

}



impl Default for TFSubscription_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__action__TFSubscription_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__action__TFSubscription_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFSubscription_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFSubscription_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/action/TFSubscription_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_Feedback() }
  }
}


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__init(msg: *mut TFSubscription_FeedbackMessage) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_FeedbackMessage>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_FeedbackMessage>);
    fn tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFSubscription_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_FeedbackMessage>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::TFSubscription_Feedback,

}



impl Default for TFSubscription_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFSubscription_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFSubscription_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/action/TFSubscription_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage() }
  }
}




#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__init(msg: *mut TFSubscription_SendGoal_Request) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_SendGoal_Request>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_SendGoal_Request>);
    fn tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFSubscription_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_SendGoal_Request>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::TFSubscription_Goal,

}



impl Default for TFSubscription_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFSubscription_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFSubscription_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/action/TFSubscription_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request() }
  }
}


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__init(msg: *mut TFSubscription_SendGoal_Response) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_SendGoal_Response>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_SendGoal_Response>);
    fn tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFSubscription_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_SendGoal_Response>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for TFSubscription_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFSubscription_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFSubscription_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/action/TFSubscription_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response() }
  }
}


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__init(msg: *mut TFSubscription_GetResult_Request) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_GetResult_Request>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_GetResult_Request>);
    fn tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFSubscription_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_GetResult_Request>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for TFSubscription_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFSubscription_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFSubscription_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/action/TFSubscription_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request() }
  }
}


#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tf2_web_republisher_interfaces__rosidl_generator_c")]
extern "C" {
    fn tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__init(msg: *mut TFSubscription_GetResult_Response) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_GetResult_Response>, size: usize) -> bool;
    fn tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_GetResult_Response>);
    fn tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TFSubscription_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TFSubscription_GetResult_Response>) -> bool;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::TFSubscription_Result,

}



impl Default for TFSubscription_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TFSubscription_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TFSubscription_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tf2_web_republisher_interfaces/action/TFSubscription_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response() }
  }
}






#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct TFSubscription_SendGoal;

impl rosidl_runtime_rs::Service for TFSubscription_SendGoal {
    type Request = TFSubscription_SendGoal_Request;
    type Response = TFSubscription_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_SendGoal() }
    }
}




#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct TFSubscription_GetResult;

impl rosidl_runtime_rs::Service for TFSubscription_GetResult {
    type Request = TFSubscription_GetResult_Request;
    type Response = TFSubscription_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription_GetResult() }
    }
}


