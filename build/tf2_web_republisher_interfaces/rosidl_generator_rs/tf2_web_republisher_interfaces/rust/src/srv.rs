#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to tf2_web_republisher_interfaces__srv__RepublishTFs_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RepublishTFs_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub source_frames: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_frame: std::string::String,


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
    pub timeout: builtin_interfaces::msg::Duration,

}



impl Default for RepublishTFs_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RepublishTFs_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RepublishTFs_Request {
  type RmwMsg = super::srv::rmw::RepublishTFs_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        source_frames: msg.source_frames
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        target_frame: msg.target_frame.as_str().into(),
        angular_thres: msg.angular_thres,
        trans_thres: msg.trans_thres,
        rate: msg.rate,
        timeout: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.timeout)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        source_frames: msg.source_frames
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        target_frame: msg.target_frame.as_str().into(),
      angular_thres: msg.angular_thres,
      trans_thres: msg.trans_thres,
      rate: msg.rate,
        timeout: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.timeout)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      source_frames: msg.source_frames
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      target_frame: msg.target_frame.to_string(),
      angular_thres: msg.angular_thres,
      trans_thres: msg.trans_thres,
      rate: msg.rate,
      timeout: builtin_interfaces::msg::Duration::from_rmw_message(msg.timeout),
    }
  }
}


// Corresponds to tf2_web_republisher_interfaces__srv__RepublishTFs_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RepublishTFs_Response {
    /// a topic of type geometry_msgs/TransformStamped[] that publishes the requested transforms
    pub topic_name: std::string::String,

}



impl Default for RepublishTFs_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RepublishTFs_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RepublishTFs_Response {
  type RmwMsg = super::srv::rmw::RepublishTFs_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic_name: msg.topic_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic_name: msg.topic_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      topic_name: msg.topic_name.to_string(),
    }
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


