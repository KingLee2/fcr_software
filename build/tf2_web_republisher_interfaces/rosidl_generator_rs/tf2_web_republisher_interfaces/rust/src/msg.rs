#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to tf2_web_republisher_interfaces__msg__TFArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub transforms: Vec<geometry_msgs::msg::TransformStamped>,

}



impl Default for TFArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TFArray::default())
  }
}

impl rosidl_runtime_rs::Message for TFArray {
  type RmwMsg = super::msg::rmw::TFArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        transforms: msg.transforms
          .into_iter()
          .map(|elem| geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        transforms: msg.transforms
          .iter()
          .map(|elem| geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      transforms: msg.transforms
          .into_iter()
          .map(geometry_msgs::msg::TransformStamped::from_rmw_message)
          .collect(),
    }
  }
}


