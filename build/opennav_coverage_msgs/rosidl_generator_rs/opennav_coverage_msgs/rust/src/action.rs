
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub gml_field: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polygons: Vec<super::msg::Coordinates>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: std::string::String,

    /// Modes of operation of each stage, if used
    pub headland_mode: super::msg::HeadlandMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swath_mode: super::msg::SwathMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub row_swath_mode: super::msg::RowSwathMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub route_mode: super::msg::RouteMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub path_mode: super::msg::PathMode,

}



impl Default for ComputeCoveragePath_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ComputeCoveragePath_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_Goal {
  type RmwMsg = super::action::rmw::ComputeCoveragePath_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        generate_headland: msg.generate_headland,
        generate_route: msg.generate_route,
        generate_path: msg.generate_path,
        use_gml_file: msg.use_gml_file,
        gml_field: msg.gml_field.as_str().into(),
        polygons: msg.polygons
          .into_iter()
          .map(|elem| super::msg::Coordinates::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        frame_id: msg.frame_id.as_str().into(),
        headland_mode: super::msg::HeadlandMode::into_rmw_message(std::borrow::Cow::Owned(msg.headland_mode)).into_owned(),
        swath_mode: super::msg::SwathMode::into_rmw_message(std::borrow::Cow::Owned(msg.swath_mode)).into_owned(),
        row_swath_mode: super::msg::RowSwathMode::into_rmw_message(std::borrow::Cow::Owned(msg.row_swath_mode)).into_owned(),
        route_mode: super::msg::RouteMode::into_rmw_message(std::borrow::Cow::Owned(msg.route_mode)).into_owned(),
        path_mode: super::msg::PathMode::into_rmw_message(std::borrow::Cow::Owned(msg.path_mode)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      generate_headland: msg.generate_headland,
      generate_route: msg.generate_route,
      generate_path: msg.generate_path,
      use_gml_file: msg.use_gml_file,
        gml_field: msg.gml_field.as_str().into(),
        polygons: msg.polygons
          .iter()
          .map(|elem| super::msg::Coordinates::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        frame_id: msg.frame_id.as_str().into(),
        headland_mode: super::msg::HeadlandMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.headland_mode)).into_owned(),
        swath_mode: super::msg::SwathMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.swath_mode)).into_owned(),
        row_swath_mode: super::msg::RowSwathMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.row_swath_mode)).into_owned(),
        route_mode: super::msg::RouteMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.route_mode)).into_owned(),
        path_mode: super::msg::PathMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.path_mode)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      generate_headland: msg.generate_headland,
      generate_route: msg.generate_route,
      generate_path: msg.generate_path,
      use_gml_file: msg.use_gml_file,
      gml_field: msg.gml_field.to_string(),
      polygons: msg.polygons
          .into_iter()
          .map(super::msg::Coordinates::from_rmw_message)
          .collect(),
      frame_id: msg.frame_id.to_string(),
      headland_mode: super::msg::HeadlandMode::from_rmw_message(msg.headland_mode),
      swath_mode: super::msg::SwathMode::from_rmw_message(msg.swath_mode),
      row_swath_mode: super::msg::RowSwathMode::from_rmw_message(msg.row_swath_mode),
      route_mode: super::msg::RouteMode::from_rmw_message(msg.route_mode),
      path_mode: super::msg::PathMode::from_rmw_message(msg.path_mode),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub nav_path: nav_msgs::msg::Path,


    // This member is not documented.
    #[allow(missing_docs)]
    pub coverage_path: super::msg::PathComponents,


    // This member is not documented.
    #[allow(missing_docs)]
    pub planning_time: builtin_interfaces::msg::Duration,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ComputeCoveragePath_Result::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_Result {
  type RmwMsg = super::action::rmw::ComputeCoveragePath_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        nav_path: nav_msgs::msg::Path::into_rmw_message(std::borrow::Cow::Owned(msg.nav_path)).into_owned(),
        coverage_path: super::msg::PathComponents::into_rmw_message(std::borrow::Cow::Owned(msg.coverage_path)).into_owned(),
        planning_time: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.planning_time)).into_owned(),
        error_code: msg.error_code,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        nav_path: nav_msgs::msg::Path::into_rmw_message(std::borrow::Cow::Borrowed(&msg.nav_path)).into_owned(),
        coverage_path: super::msg::PathComponents::into_rmw_message(std::borrow::Cow::Borrowed(&msg.coverage_path)).into_owned(),
        planning_time: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.planning_time)).into_owned(),
      error_code: msg.error_code,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      nav_path: nav_msgs::msg::Path::from_rmw_message(msg.nav_path),
      coverage_path: super::msg::PathComponents::from_rmw_message(msg.coverage_path),
      planning_time: builtin_interfaces::msg::Duration::from_rmw_message(msg.planning_time),
      error_code: msg.error_code,
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ComputeCoveragePath_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ComputeCoveragePath_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_Feedback {
  type RmwMsg = super::action::rmw::ComputeCoveragePath_Feedback;

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


// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::ComputeCoveragePath_Feedback,

}



impl Default for ComputeCoveragePath_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ComputeCoveragePath_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_FeedbackMessage {
  type RmwMsg = super::action::rmw::ComputeCoveragePath_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::ComputeCoveragePath_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::ComputeCoveragePath_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::ComputeCoveragePath_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_Goal {
    /// Define the field as either a filepath to a GML file or as a vector of polygons
    /// Whereas the first polygon is the outer field and subsequent polygons are internal voids.
    /// When both are specified, the file is used in ComputeCoveragePath BT Node.
    pub field_filepath: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polygons: Vec<geometry_msgs::msg::Polygon>,

    /// Specify the frame of reference of the polygon field
    pub frame_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub behavior_tree: std::string::String,

}



impl Default for NavigateCompleteCoverage_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavigateCompleteCoverage_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_Goal {
  type RmwMsg = super::action::rmw::NavigateCompleteCoverage_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        field_filepath: msg.field_filepath.as_str().into(),
        polygons: msg.polygons
          .into_iter()
          .map(|elem| geometry_msgs::msg::Polygon::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        frame_id: msg.frame_id.as_str().into(),
        behavior_tree: msg.behavior_tree.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        field_filepath: msg.field_filepath.as_str().into(),
        polygons: msg.polygons
          .iter()
          .map(|elem| geometry_msgs::msg::Polygon::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        frame_id: msg.frame_id.as_str().into(),
        behavior_tree: msg.behavior_tree.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      field_filepath: msg.field_filepath.to_string(),
      polygons: msg.polygons
          .into_iter()
          .map(geometry_msgs::msg::Polygon::from_rmw_message)
          .collect(),
      frame_id: msg.frame_id.to_string(),
      behavior_tree: msg.behavior_tree.to_string(),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavigateCompleteCoverage_Result::default())
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_Result {
  type RmwMsg = super::action::rmw::NavigateCompleteCoverage_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: msg.error_code,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_code: msg.error_code,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: msg.error_code,
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub navigation_time: builtin_interfaces::msg::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub estimated_time_remaining: builtin_interfaces::msg::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub number_of_recoveries: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_remaining: f32,

}



impl Default for NavigateCompleteCoverage_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavigateCompleteCoverage_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_Feedback {
  type RmwMsg = super::action::rmw::NavigateCompleteCoverage_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.current_pose)).into_owned(),
        navigation_time: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.navigation_time)).into_owned(),
        estimated_time_remaining: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.estimated_time_remaining)).into_owned(),
        number_of_recoveries: msg.number_of_recoveries,
        distance_remaining: msg.distance_remaining,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_pose)).into_owned(),
        navigation_time: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.navigation_time)).into_owned(),
        estimated_time_remaining: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.estimated_time_remaining)).into_owned(),
      number_of_recoveries: msg.number_of_recoveries,
      distance_remaining: msg.distance_remaining,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.current_pose),
      navigation_time: builtin_interfaces::msg::Duration::from_rmw_message(msg.navigation_time),
      estimated_time_remaining: builtin_interfaces::msg::Duration::from_rmw_message(msg.estimated_time_remaining),
      number_of_recoveries: msg.number_of_recoveries,
      distance_remaining: msg.distance_remaining,
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::NavigateCompleteCoverage_Feedback,

}



impl Default for NavigateCompleteCoverage_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavigateCompleteCoverage_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_FeedbackMessage {
  type RmwMsg = super::action::rmw::NavigateCompleteCoverage_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::NavigateCompleteCoverage_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::NavigateCompleteCoverage_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::NavigateCompleteCoverage_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::ComputeCoveragePath_Goal,

}



impl Default for ComputeCoveragePath_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ComputeCoveragePath_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_SendGoal_Request {
  type RmwMsg = super::action::rmw::ComputeCoveragePath_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::ComputeCoveragePath_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::ComputeCoveragePath_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::ComputeCoveragePath_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for ComputeCoveragePath_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ComputeCoveragePath_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_SendGoal_Response {
  type RmwMsg = super::action::rmw::ComputeCoveragePath_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for ComputeCoveragePath_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ComputeCoveragePath_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_GetResult_Request {
  type RmwMsg = super::action::rmw::ComputeCoveragePath_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCoveragePath_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::ComputeCoveragePath_Result,

}



impl Default for ComputeCoveragePath_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ComputeCoveragePath_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCoveragePath_GetResult_Response {
  type RmwMsg = super::action::rmw::ComputeCoveragePath_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::ComputeCoveragePath_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::ComputeCoveragePath_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::ComputeCoveragePath_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::NavigateCompleteCoverage_Goal,

}



impl Default for NavigateCompleteCoverage_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavigateCompleteCoverage_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_SendGoal_Request {
  type RmwMsg = super::action::rmw::NavigateCompleteCoverage_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::NavigateCompleteCoverage_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::NavigateCompleteCoverage_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::NavigateCompleteCoverage_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for NavigateCompleteCoverage_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavigateCompleteCoverage_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_SendGoal_Response {
  type RmwMsg = super::action::rmw::NavigateCompleteCoverage_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for NavigateCompleteCoverage_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavigateCompleteCoverage_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_GetResult_Request {
  type RmwMsg = super::action::rmw::NavigateCompleteCoverage_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateCompleteCoverage_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::NavigateCompleteCoverage_Result,

}



impl Default for NavigateCompleteCoverage_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavigateCompleteCoverage_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for NavigateCompleteCoverage_GetResult_Response {
  type RmwMsg = super::action::rmw::NavigateCompleteCoverage_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::NavigateCompleteCoverage_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::NavigateCompleteCoverage_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::NavigateCompleteCoverage_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath() -> *const std::ffi::c_void;
}

// Corresponds to opennav_coverage_msgs__action__ComputeCoveragePath
#[allow(missing_docs, non_camel_case_types)]
pub struct ComputeCoveragePath;

impl rosidl_runtime_rs::Action for ComputeCoveragePath {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = ComputeCoveragePath_Goal;

  /// The result message defined in the action definition.
  type Result = ComputeCoveragePath_Result;

  /// The feedback message defined in the action definition.
  type Feedback = ComputeCoveragePath_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::ComputeCoveragePath_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::ComputeCoveragePath_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::ComputeCoveragePath_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__opennav_coverage_msgs__action__ComputeCoveragePath() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::ComputeCoveragePath_Goal,
  ) -> super::action::rmw::ComputeCoveragePath_SendGoal_Request {
   super::action::rmw::ComputeCoveragePath_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::ComputeCoveragePath_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::ComputeCoveragePath_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::ComputeCoveragePath_SendGoal_Response {
   super::action::rmw::ComputeCoveragePath_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::ComputeCoveragePath_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::ComputeCoveragePath_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::ComputeCoveragePath_Feedback,
  ) -> super::action::rmw::ComputeCoveragePath_FeedbackMessage {
    let mut message = super::action::rmw::ComputeCoveragePath_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::ComputeCoveragePath_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::ComputeCoveragePath_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::ComputeCoveragePath_GetResult_Request {
   super::action::rmw::ComputeCoveragePath_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::ComputeCoveragePath_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::ComputeCoveragePath_Result,
  ) -> super::action::rmw::ComputeCoveragePath_GetResult_Response {
   super::action::rmw::ComputeCoveragePath_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::ComputeCoveragePath_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::ComputeCoveragePath_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "opennav_coverage_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage() -> *const std::ffi::c_void;
}

// Corresponds to opennav_coverage_msgs__action__NavigateCompleteCoverage
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateCompleteCoverage;

impl rosidl_runtime_rs::Action for NavigateCompleteCoverage {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = NavigateCompleteCoverage_Goal;

  /// The result message defined in the action definition.
  type Result = NavigateCompleteCoverage_Result;

  /// The feedback message defined in the action definition.
  type Feedback = NavigateCompleteCoverage_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::NavigateCompleteCoverage_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::NavigateCompleteCoverage_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::NavigateCompleteCoverage_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__opennav_coverage_msgs__action__NavigateCompleteCoverage() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::NavigateCompleteCoverage_Goal,
  ) -> super::action::rmw::NavigateCompleteCoverage_SendGoal_Request {
   super::action::rmw::NavigateCompleteCoverage_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::NavigateCompleteCoverage_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::NavigateCompleteCoverage_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::NavigateCompleteCoverage_SendGoal_Response {
   super::action::rmw::NavigateCompleteCoverage_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::NavigateCompleteCoverage_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::NavigateCompleteCoverage_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::NavigateCompleteCoverage_Feedback,
  ) -> super::action::rmw::NavigateCompleteCoverage_FeedbackMessage {
    let mut message = super::action::rmw::NavigateCompleteCoverage_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::NavigateCompleteCoverage_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::NavigateCompleteCoverage_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::NavigateCompleteCoverage_GetResult_Request {
   super::action::rmw::NavigateCompleteCoverage_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::NavigateCompleteCoverage_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::NavigateCompleteCoverage_Result,
  ) -> super::action::rmw::NavigateCompleteCoverage_GetResult_Response {
   super::action::rmw::NavigateCompleteCoverage_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::NavigateCompleteCoverage_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::NavigateCompleteCoverage_Result,
  ) {
    (response.status, response.result)
  }
}


