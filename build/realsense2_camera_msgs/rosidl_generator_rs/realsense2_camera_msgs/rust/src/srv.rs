#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to realsense2_camera_msgs__srv__DeviceInfo_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceInfo_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DeviceInfo_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeviceInfo_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeviceInfo_Request {
  type RmwMsg = super::srv::rmw::DeviceInfo_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__DeviceInfo_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceInfo_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub device_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_number: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_version: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub usb_type_descriptor: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_update_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sensors: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub physical_port: std::string::String,

}



impl Default for DeviceInfo_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeviceInfo_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeviceInfo_Response {
  type RmwMsg = super::srv::rmw::DeviceInfo_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        device_name: msg.device_name.as_str().into(),
        serial_number: msg.serial_number.as_str().into(),
        firmware_version: msg.firmware_version.as_str().into(),
        usb_type_descriptor: msg.usb_type_descriptor.as_str().into(),
        firmware_update_id: msg.firmware_update_id.as_str().into(),
        sensors: msg.sensors.as_str().into(),
        physical_port: msg.physical_port.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        device_name: msg.device_name.as_str().into(),
        serial_number: msg.serial_number.as_str().into(),
        firmware_version: msg.firmware_version.as_str().into(),
        usb_type_descriptor: msg.usb_type_descriptor.as_str().into(),
        firmware_update_id: msg.firmware_update_id.as_str().into(),
        sensors: msg.sensors.as_str().into(),
        physical_port: msg.physical_port.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      device_name: msg.device_name.to_string(),
      serial_number: msg.serial_number.to_string(),
      firmware_version: msg.firmware_version.to_string(),
      usb_type_descriptor: msg.usb_type_descriptor.to_string(),
      firmware_update_id: msg.firmware_update_id.to_string(),
      sensors: msg.sensors.to_string(),
      physical_port: msg.physical_port.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for CalibConfigRead_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalibConfigRead_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CalibConfigRead_Request {
  type RmwMsg = super::srv::rmw::CalibConfigRead_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_config: std::string::String,

}



impl Default for CalibConfigRead_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalibConfigRead_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CalibConfigRead_Response {
  type RmwMsg = super::srv::rmw::CalibConfigRead_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
        calib_config: msg.calib_config.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
        calib_config: msg.calib_config.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
      calib_config: msg.calib_config.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_config: std::string::String,

}



impl Default for CalibConfigWrite_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalibConfigWrite_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CalibConfigWrite_Request {
  type RmwMsg = super::srv::rmw::CalibConfigWrite_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        calib_config: msg.calib_config.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        calib_config: msg.calib_config.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      calib_config: msg.calib_config.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,

}



impl Default for CalibConfigWrite_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalibConfigWrite_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CalibConfigWrite_Response {
  type RmwMsg = super::srv::rmw::CalibConfigWrite_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
    }
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


