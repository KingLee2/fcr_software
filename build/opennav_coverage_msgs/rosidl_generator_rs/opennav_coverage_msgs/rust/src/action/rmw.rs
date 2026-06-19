
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_Goal() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Goal__init(msg: *mut ComputeCoveragePath_Goal) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Goal>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Goal>);
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCoveragePath_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Goal>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_Goal {
    /// Whether to perform all 4 stages: Headlands, Swath (Required), Route, Path
    pub generate_headland: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub generate_route: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub generate_path: bool,

    /// The field specification to use.
    /// If using polygons, bounding polygon must be first, followed by inner cutouts
    /// Both must specify if the data is cartesian or GPS coordinates
    /// If using Row Coverage Server, must use gml field.
    pub use_gml_file: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gml_field: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polygons: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Coordinates>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: rosidl_runtime_rs::String,

    /// Modes of operation of each stage, if used
    pub headland_mode: super::super::msg::rmw::HeadlandMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swath_mode: super::super::msg::rmw::SwathMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub row_swath_mode: super::super::msg::rmw::RowSwathMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub route_mode: super::super::msg::rmw::RouteMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub path_mode: super::super::msg::rmw::PathMode,

}



impl Default for ComputeCoveragePath_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__ComputeCoveragePath_Goal__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__ComputeCoveragePath_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCoveragePath_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCoveragePath_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/ComputeCoveragePath_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_Goal() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_Result() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Result__init(msg: *mut ComputeCoveragePath_Result) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Result>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Result>);
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCoveragePath_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Result>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub nav_path: nav_msgs::msg::rmw::Path,


    // This member is not documented.
    #[allow(missing_docs)]
    pub coverage_path: super::super::msg::rmw::PathComponents,


    // This member is not documented.
    #[allow(missing_docs)]
    pub planning_time: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: u16,

}

impl ComputeCoveragePath_Result {
    /// Error codes
    /// Note: The expected priority order of the errors should match the message order
    pub const NONE: u16 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INTERNAL_F2C_ERROR: u16 = 801;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_MODE_SET: u16 = 802;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_REQUEST: u16 = 803;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_COORDS: u16 = 803;

}


impl Default for ComputeCoveragePath_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__ComputeCoveragePath_Result__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__ComputeCoveragePath_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCoveragePath_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCoveragePath_Result where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/ComputeCoveragePath_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_Result() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__init(msg: *mut ComputeCoveragePath_Feedback) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Feedback>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Feedback>);
    fn opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCoveragePath_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_Feedback>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ComputeCoveragePath_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCoveragePath_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCoveragePath_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/ComputeCoveragePath_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_Feedback() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__init(msg: *mut ComputeCoveragePath_FeedbackMessage) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_FeedbackMessage>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_FeedbackMessage>);
    fn opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCoveragePath_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_FeedbackMessage>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::ComputeCoveragePath_Feedback,

}



impl Default for ComputeCoveragePath_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCoveragePath_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCoveragePath_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/ComputeCoveragePath_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__init(msg: *mut NavigateCompleteCoverage_Goal) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Goal>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Goal>);
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Goal>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_Goal {
    /// Define the field as either a filepath to a GML file or as a vector of polygons
    /// Whereas the first polygon is the outer field and subsequent polygons are internal voids.
    /// When both are specified, the file is used in ComputeCoveragePath BT Node.
    pub field_filepath: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polygons: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Polygon>,

    /// Specify the frame of reference of the polygon field
    pub frame_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub behavior_tree: rosidl_runtime_rs::String,

}



impl Default for NavigateCompleteCoverage_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateCompleteCoverage_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateCompleteCoverage_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/NavigateCompleteCoverage_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_Result() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__init(msg: *mut NavigateCompleteCoverage_Result) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Result>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Result>);
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Result>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: u16,

}

impl NavigateCompleteCoverage_Result {
    /// Error codes
    /// Note: The expected priority order of the errors should match the message order
    pub const NONE: u16 = 0;

}


impl Default for NavigateCompleteCoverage_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateCompleteCoverage_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateCompleteCoverage_Result where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/NavigateCompleteCoverage_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_Result() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__init(msg: *mut NavigateCompleteCoverage_Feedback) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Feedback>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Feedback>);
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_Feedback>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::rmw::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub navigation_time: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub estimated_time_remaining: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub number_of_recoveries: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_remaining: f32,

}



impl Default for NavigateCompleteCoverage_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateCompleteCoverage_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateCompleteCoverage_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/NavigateCompleteCoverage_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__init(msg: *mut NavigateCompleteCoverage_FeedbackMessage) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_FeedbackMessage>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_FeedbackMessage>);
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_FeedbackMessage>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::NavigateCompleteCoverage_Feedback,

}



impl Default for NavigateCompleteCoverage_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateCompleteCoverage_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateCompleteCoverage_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/NavigateCompleteCoverage_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage() }
  }
}




#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__init(msg: *mut ComputeCoveragePath_SendGoal_Request) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_SendGoal_Request>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_SendGoal_Request>);
    fn opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCoveragePath_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_SendGoal_Request>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::ComputeCoveragePath_Goal,

}



impl Default for ComputeCoveragePath_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCoveragePath_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCoveragePath_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/ComputeCoveragePath_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__init(msg: *mut ComputeCoveragePath_SendGoal_Response) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_SendGoal_Response>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_SendGoal_Response>);
    fn opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCoveragePath_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_SendGoal_Response>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for ComputeCoveragePath_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCoveragePath_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCoveragePath_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/ComputeCoveragePath_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__init(msg: *mut ComputeCoveragePath_GetResult_Request) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_GetResult_Request>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_GetResult_Request>);
    fn opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCoveragePath_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_GetResult_Request>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for ComputeCoveragePath_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCoveragePath_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCoveragePath_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/ComputeCoveragePath_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__init(msg: *mut ComputeCoveragePath_GetResult_Response) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_GetResult_Response>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_GetResult_Response>);
    fn opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCoveragePath_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCoveragePath_GetResult_Response>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::ComputeCoveragePath_Result,

}



impl Default for ComputeCoveragePath_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCoveragePath_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCoveragePath_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/ComputeCoveragePath_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__init(msg: *mut NavigateCompleteCoverage_SendGoal_Request) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_SendGoal_Request>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_SendGoal_Request>);
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_SendGoal_Request>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::NavigateCompleteCoverage_Goal,

}



impl Default for NavigateCompleteCoverage_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateCompleteCoverage_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateCompleteCoverage_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/NavigateCompleteCoverage_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__init(msg: *mut NavigateCompleteCoverage_SendGoal_Response) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_SendGoal_Response>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_SendGoal_Response>);
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_SendGoal_Response>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for NavigateCompleteCoverage_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateCompleteCoverage_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateCompleteCoverage_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/NavigateCompleteCoverage_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__init(msg: *mut NavigateCompleteCoverage_GetResult_Request) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_GetResult_Request>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_GetResult_Request>);
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_GetResult_Request>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for NavigateCompleteCoverage_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateCompleteCoverage_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateCompleteCoverage_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/NavigateCompleteCoverage_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__init(msg: *mut NavigateCompleteCoverage_GetResult_Response) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_GetResult_Response>, size: usize) -> bool;
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_GetResult_Response>);
    fn opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateCompleteCoverage_GetResult_Response>) -> bool;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::NavigateCompleteCoverage_Result,

}



impl Default for NavigateCompleteCoverage_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateCompleteCoverage_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateCompleteCoverage_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/action/NavigateCompleteCoverage_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response() }
  }
}






#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct ComputeCoveragePath_SendGoal;

impl rosidl_runtime_rs::Service for ComputeCoveragePath_SendGoal {
    type Request = ComputeCoveragePath_SendGoal_Request;
    type Response = ComputeCoveragePath_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal() }
    }
}




#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct ComputeCoveragePath_GetResult;

impl rosidl_runtime_rs::Service for ComputeCoveragePath_GetResult {
    type Request = ComputeCoveragePath_GetResult_Request;
    type Response = ComputeCoveragePath_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath_GetResult() }
    }
}




#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateCompleteCoverage_SendGoal;

impl rosidl_runtime_rs::Service for NavigateCompleteCoverage_SendGoal {
    type Request = NavigateCompleteCoverage_SendGoal_Request;
    type Response = NavigateCompleteCoverage_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal() }
    }
}




#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateCompleteCoverage_GetResult;

impl rosidl_runtime_rs::Service for NavigateCompleteCoverage_GetResult {
    type Request = NavigateCompleteCoverage_GetResult_Request;
    type Response = NavigateCompleteCoverage_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult() }
    }
}


