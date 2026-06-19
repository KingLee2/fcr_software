#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to opennav_coverage_msgs__msg__Coordinate
/// Can be GPS, Cartesian, or any other kind of coordinate system

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Coordinate::default())
  }
}

impl rosidl_runtime_rs::Message for Coordinate {
  type RmwMsg = super::msg::rmw::Coordinate;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        axis1: msg.axis1,
        axis2: msg.axis2,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      axis1: msg.axis1,
      axis2: msg.axis2,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      axis1: msg.axis1,
      axis2: msg.axis2,
    }
  }
}


// Corresponds to opennav_coverage_msgs__msg__Coordinates

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coordinates {

    // This member is not documented.
    #[allow(missing_docs)]
    pub coordinates: Vec<super::msg::Coordinate>,

}



impl Default for Coordinates {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Coordinates::default())
  }
}

impl rosidl_runtime_rs::Message for Coordinates {
  type RmwMsg = super::msg::rmw::Coordinates;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        coordinates: msg.coordinates
          .into_iter()
          .map(|elem| super::msg::Coordinate::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        coordinates: msg.coordinates
          .iter()
          .map(|elem| super::msg::Coordinate::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      coordinates: msg.coordinates
          .into_iter()
          .map(super::msg::Coordinate::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to opennav_coverage_msgs__msg__HeadlandMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HeadlandMode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: std::string::String,

    /// Specific mode setting
    /// width of headland around field
    pub width: f32,

}



impl Default for HeadlandMode {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HeadlandMode::default())
  }
}

impl rosidl_runtime_rs::Message for HeadlandMode {
  type RmwMsg = super::msg::rmw::HeadlandMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
        width: msg.width,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
      width: msg.width,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode.to_string(),
      width: msg.width,
    }
  }
}


// Corresponds to opennav_coverage_msgs__msg__SwathMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwathMode {
    /// LENGTH, NUMBER, or COVERAGE
    pub objective: std::string::String,

    /// BRUTE_FORCE, SET_ANGLE
    pub mode: std::string::String,

    /// Specific mode settings
    /// If mode=SET_ANGLE for angle to generate swaths based on
    pub best_angle: f32,

    /// If mode=BRUTE_FORCE, angular step to search
    pub step_angle: f32,

}



impl Default for SwathMode {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SwathMode::default())
  }
}

impl rosidl_runtime_rs::Message for SwathMode {
  type RmwMsg = super::msg::rmw::SwathMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        objective: msg.objective.as_str().into(),
        mode: msg.mode.as_str().into(),
        best_angle: msg.best_angle,
        step_angle: msg.step_angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        objective: msg.objective.as_str().into(),
        mode: msg.mode.as_str().into(),
      best_angle: msg.best_angle,
      step_angle: msg.step_angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      objective: msg.objective.to_string(),
      mode: msg.mode.to_string(),
      best_angle: msg.best_angle,
      step_angle: msg.step_angle,
    }
  }
}


// Corresponds to opennav_coverage_msgs__msg__RowSwathMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RowSwathMode {
    /// CENTER, OFFSET, ROWSARESWATHS
    pub mode: std::string::String,

    /// The ids to skip
    pub skip_ids: Vec<i32>,

    /// Specific mode settings
    /// If mode=OFFSET for offset for swath relative to rows
    pub offset: f32,

}



impl Default for RowSwathMode {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RowSwathMode::default())
  }
}

impl rosidl_runtime_rs::Message for RowSwathMode {
  type RmwMsg = super::msg::rmw::RowSwathMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
        skip_ids: msg.skip_ids.into(),
        offset: msg.offset,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
        skip_ids: msg.skip_ids.as_slice().into(),
      offset: msg.offset,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode.to_string(),
      skip_ids: msg.skip_ids
          .into_iter()
          .collect(),
      offset: msg.offset,
    }
  }
}


// Corresponds to opennav_coverage_msgs__msg__RouteMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RouteMode {
    /// BOUSTROPHEDON, SNAKE, SPIRAL, CUSTOM
    pub mode: std::string::String,

    /// Specific mode setting
    /// If mode=SPIRAL, this is the number of swaths to spiral
    pub spiral_n: u16,

    /// If mode=CUSTOM, this is the order of swaths to set. Must be specified.
    pub custom_order: Vec<u16>,

}



impl Default for RouteMode {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RouteMode::default())
  }
}

impl rosidl_runtime_rs::Message for RouteMode {
  type RmwMsg = super::msg::rmw::RouteMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
        spiral_n: msg.spiral_n,
        custom_order: msg.custom_order.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
      spiral_n: msg.spiral_n,
        custom_order: msg.custom_order.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode.to_string(),
      spiral_n: msg.spiral_n,
      custom_order: msg.custom_order
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to opennav_coverage_msgs__msg__PathMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathMode {
    /// DUBIN, REEDS_SHEPP
    pub mode: std::string::String,

    /// CONTINUOUS, DISCONTINUOUS
    pub continuity_mode: std::string::String,

    /// Distance between path points in non-swath turns
    pub turn_point_distance: f32,

}



impl Default for PathMode {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PathMode::default())
  }
}

impl rosidl_runtime_rs::Message for PathMode {
  type RmwMsg = super::msg::rmw::PathMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
        continuity_mode: msg.continuity_mode.as_str().into(),
        turn_point_distance: msg.turn_point_distance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
        continuity_mode: msg.continuity_mode.as_str().into(),
      turn_point_distance: msg.turn_point_distance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode.to_string(),
      continuity_mode: msg.continuity_mode.to_string(),
      turn_point_distance: msg.turn_point_distance,
    }
  }
}


// Corresponds to opennav_coverage_msgs__msg__Swath

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Swath {

    // This member is not documented.
    #[allow(missing_docs)]
    pub start: geometry_msgs::msg::Point32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub end: geometry_msgs::msg::Point32,

}



impl Default for Swath {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Swath::default())
  }
}

impl rosidl_runtime_rs::Message for Swath {
  type RmwMsg = super::msg::rmw::Swath;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start: geometry_msgs::msg::Point32::into_rmw_message(std::borrow::Cow::Owned(msg.start)).into_owned(),
        end: geometry_msgs::msg::Point32::into_rmw_message(std::borrow::Cow::Owned(msg.end)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start: geometry_msgs::msg::Point32::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start)).into_owned(),
        end: geometry_msgs::msg::Point32::into_rmw_message(std::borrow::Cow::Borrowed(&msg.end)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      start: geometry_msgs::msg::Point32::from_rmw_message(msg.start),
      end: geometry_msgs::msg::Point32::from_rmw_message(msg.end),
    }
  }
}


// Corresponds to opennav_coverage_msgs__msg__PathComponents
/// An ordered set of swaths (if route generated) and turns (if path generated)

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathComponents {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swaths: Vec<super::msg::Swath>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub turns: Vec<nav_msgs::msg::Path>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub contains_turns: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swaths_ordered: bool,

}



impl Default for PathComponents {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PathComponents::default())
  }
}

impl rosidl_runtime_rs::Message for PathComponents {
  type RmwMsg = super::msg::rmw::PathComponents;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        swaths: msg.swaths
          .into_iter()
          .map(|elem| super::msg::Swath::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        turns: msg.turns
          .into_iter()
          .map(|elem| nav_msgs::msg::Path::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        contains_turns: msg.contains_turns,
        swaths_ordered: msg.swaths_ordered,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        swaths: msg.swaths
          .iter()
          .map(|elem| super::msg::Swath::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        turns: msg.turns
          .iter()
          .map(|elem| nav_msgs::msg::Path::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      contains_turns: msg.contains_turns,
      swaths_ordered: msg.swaths_ordered,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      swaths: msg.swaths
          .into_iter()
          .map(super::msg::Swath::from_rmw_message)
          .collect(),
      turns: msg.turns
          .into_iter()
          .map(nav_msgs::msg::Path::from_rmw_message)
          .collect(),
      contains_turns: msg.contains_turns,
      swaths_ordered: msg.swaths_ordered,
    }
  }
}


