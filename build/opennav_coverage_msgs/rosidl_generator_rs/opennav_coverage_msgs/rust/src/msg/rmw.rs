#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__Coordinate() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__Coordinate__init(msg: *mut Coordinate) -> bool;
    fn opennav_coverage_msgs__msg__Coordinate__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coordinate>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__Coordinate__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coordinate>);
    fn opennav_coverage_msgs__msg__Coordinate__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coordinate>, out_seq: *mut rosidl_runtime_rs::Sequence<Coordinate>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__Coordinate
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Can be GPS, Cartesian, or any other kind of coordinate system

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coordinate {

    // This member is not documented.
    #[allow(missing_docs)]
    pub axis1: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub axis2: f32,

}



impl Default for Coordinate {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__Coordinate__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__Coordinate__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coordinate {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Coordinate__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Coordinate__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Coordinate__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coordinate {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coordinate where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/Coordinate";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__Coordinate() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__Coordinates() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__Coordinates__init(msg: *mut Coordinates) -> bool;
    fn opennav_coverage_msgs__msg__Coordinates__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coordinates>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__Coordinates__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coordinates>);
    fn opennav_coverage_msgs__msg__Coordinates__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coordinates>, out_seq: *mut rosidl_runtime_rs::Sequence<Coordinates>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__Coordinates
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coordinates {

    // This member is not documented.
    #[allow(missing_docs)]
    pub coordinates: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Coordinate>,

}



impl Default for Coordinates {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__Coordinates__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__Coordinates__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coordinates {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Coordinates__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Coordinates__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Coordinates__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coordinates {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coordinates where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/Coordinates";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__Coordinates() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__HeadlandMode() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__HeadlandMode__init(msg: *mut HeadlandMode) -> bool;
    fn opennav_coverage_msgs__msg__HeadlandMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HeadlandMode>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__HeadlandMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HeadlandMode>);
    fn opennav_coverage_msgs__msg__HeadlandMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HeadlandMode>, out_seq: *mut rosidl_runtime_rs::Sequence<HeadlandMode>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__HeadlandMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HeadlandMode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: rosidl_runtime_rs::String,

    /// Specific mode setting
    /// width of headland around field
    pub width: f32,

}



impl Default for HeadlandMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__HeadlandMode__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__HeadlandMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HeadlandMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__HeadlandMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__HeadlandMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__HeadlandMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HeadlandMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HeadlandMode where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/HeadlandMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__HeadlandMode() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__SwathMode() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__SwathMode__init(msg: *mut SwathMode) -> bool;
    fn opennav_coverage_msgs__msg__SwathMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwathMode>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__SwathMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwathMode>);
    fn opennav_coverage_msgs__msg__SwathMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwathMode>, out_seq: *mut rosidl_runtime_rs::Sequence<SwathMode>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__SwathMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwathMode {
    /// LENGTH, NUMBER, or COVERAGE
    pub objective: rosidl_runtime_rs::String,

    /// BRUTE_FORCE, SET_ANGLE
    pub mode: rosidl_runtime_rs::String,

    /// Specific mode settings
    /// If mode=SET_ANGLE for angle to generate swaths based on
    pub best_angle: f32,

    /// If mode=BRUTE_FORCE, angular step to search
    pub step_angle: f32,

}



impl Default for SwathMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__SwathMode__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__SwathMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwathMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__SwathMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__SwathMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__SwathMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwathMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwathMode where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/SwathMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__SwathMode() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__RowSwathMode() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__RowSwathMode__init(msg: *mut RowSwathMode) -> bool;
    fn opennav_coverage_msgs__msg__RowSwathMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RowSwathMode>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__RowSwathMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RowSwathMode>);
    fn opennav_coverage_msgs__msg__RowSwathMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RowSwathMode>, out_seq: *mut rosidl_runtime_rs::Sequence<RowSwathMode>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__RowSwathMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RowSwathMode {
    /// CENTER, OFFSET, ROWSARESWATHS
    pub mode: rosidl_runtime_rs::String,

    /// The ids to skip
    pub skip_ids: rosidl_runtime_rs::Sequence<i32>,

    /// Specific mode settings
    /// If mode=OFFSET for offset for swath relative to rows
    pub offset: f32,

}



impl Default for RowSwathMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__RowSwathMode__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__RowSwathMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RowSwathMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__RowSwathMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__RowSwathMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__RowSwathMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RowSwathMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RowSwathMode where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/RowSwathMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__RowSwathMode() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__RouteMode() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__RouteMode__init(msg: *mut RouteMode) -> bool;
    fn opennav_coverage_msgs__msg__RouteMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RouteMode>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__RouteMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RouteMode>);
    fn opennav_coverage_msgs__msg__RouteMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RouteMode>, out_seq: *mut rosidl_runtime_rs::Sequence<RouteMode>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__RouteMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RouteMode {
    /// BOUSTROPHEDON, SNAKE, SPIRAL, CUSTOM
    pub mode: rosidl_runtime_rs::String,

    /// Specific mode setting
    /// If mode=SPIRAL, this is the number of swaths to spiral
    pub spiral_n: u16,

    /// If mode=CUSTOM, this is the order of swaths to set. Must be specified.
    pub custom_order: rosidl_runtime_rs::Sequence<u16>,

}



impl Default for RouteMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__RouteMode__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__RouteMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RouteMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__RouteMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__RouteMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__RouteMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RouteMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RouteMode where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/RouteMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__RouteMode() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__PathMode() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__PathMode__init(msg: *mut PathMode) -> bool;
    fn opennav_coverage_msgs__msg__PathMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathMode>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__PathMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathMode>);
    fn opennav_coverage_msgs__msg__PathMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathMode>, out_seq: *mut rosidl_runtime_rs::Sequence<PathMode>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__PathMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathMode {
    /// DUBIN, REEDS_SHEPP
    pub mode: rosidl_runtime_rs::String,

    /// CONTINUOUS, DISCONTINUOUS
    pub continuity_mode: rosidl_runtime_rs::String,

    /// Distance between path points in non-swath turns
    pub turn_point_distance: f32,

}



impl Default for PathMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__PathMode__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__PathMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__PathMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__PathMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__PathMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathMode where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/PathMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__PathMode() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__Swath() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__Swath__init(msg: *mut Swath) -> bool;
    fn opennav_coverage_msgs__msg__Swath__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Swath>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__Swath__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Swath>);
    fn opennav_coverage_msgs__msg__Swath__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Swath>, out_seq: *mut rosidl_runtime_rs::Sequence<Swath>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__Swath
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Swath {

    // This member is not documented.
    #[allow(missing_docs)]
    pub start: geometry_msgs::msg::rmw::Point32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub end: geometry_msgs::msg::rmw::Point32,

}



impl Default for Swath {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__Swath__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__Swath__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Swath {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Swath__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Swath__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__Swath__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Swath {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Swath where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/Swath";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__Swath() }
  }
}


#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__PathComponents() -> *const std::ffi::c_void;
}

#[link(name = "opennav_coverage_msgs__rosidl_generator_c")]
extern "C" {
    fn opennav_coverage_msgs__msg__PathComponents__init(msg: *mut PathComponents) -> bool;
    fn opennav_coverage_msgs__msg__PathComponents__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathComponents>, size: usize) -> bool;
    fn opennav_coverage_msgs__msg__PathComponents__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathComponents>);
    fn opennav_coverage_msgs__msg__PathComponents__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathComponents>, out_seq: *mut rosidl_runtime_rs::Sequence<PathComponents>) -> bool;
}

// Corresponds to opennav_coverage_msgs__msg__PathComponents
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// An ordered set of swaths (if route generated) and turns (if path generated)

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathComponents {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swaths: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Swath>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub turns: rosidl_runtime_rs::Sequence<nav_msgs::msg::rmw::Path>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub contains_turns: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swaths_ordered: bool,

}



impl Default for PathComponents {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !opennav_coverage_msgs__msg__PathComponents__init(&mut msg as *mut _) {
        panic!("Call to opennav_coverage_msgs__msg__PathComponents__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathComponents {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__PathComponents__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__PathComponents__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { opennav_coverage_msgs__msg__PathComponents__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathComponents {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathComponents where Self: Sized {
  const TYPE_NAME: &'static str = "opennav_coverage_msgs/msg/PathComponents";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__opennav_coverage_msgs__msg__PathComponents() }
  }
}


