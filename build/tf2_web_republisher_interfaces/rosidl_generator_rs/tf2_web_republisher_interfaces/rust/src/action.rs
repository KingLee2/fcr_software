
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_Goal {

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

}



impl Default for TFSubscription_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TFSubscription_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_Goal {
  type RmwMsg = super::action::rmw::TFSubscription_Goal;

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
    }
  }
}


// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for TFSubscription_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TFSubscription_Result::default())
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_Result {
  type RmwMsg = super::action::rmw::TFSubscription_Result;

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


// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub transforms: Vec<geometry_msgs::msg::TransformStamped>,

}



impl Default for TFSubscription_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TFSubscription_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_Feedback {
  type RmwMsg = super::action::rmw::TFSubscription_Feedback;

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


// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::TFSubscription_Feedback,

}



impl Default for TFSubscription_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TFSubscription_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_FeedbackMessage {
  type RmwMsg = super::action::rmw::TFSubscription_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::TFSubscription_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::TFSubscription_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::TFSubscription_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::TFSubscription_Goal,

}



impl Default for TFSubscription_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TFSubscription_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_SendGoal_Request {
  type RmwMsg = super::action::rmw::TFSubscription_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::TFSubscription_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::TFSubscription_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::TFSubscription_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for TFSubscription_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TFSubscription_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_SendGoal_Response {
  type RmwMsg = super::action::rmw::TFSubscription_SendGoal_Response;

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


// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for TFSubscription_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TFSubscription_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_GetResult_Request {
  type RmwMsg = super::action::rmw::TFSubscription_GetResult_Request;

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


// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TFSubscription_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::TFSubscription_Result,

}



impl Default for TFSubscription_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TFSubscription_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TFSubscription_GetResult_Response {
  type RmwMsg = super::action::rmw::TFSubscription_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::TFSubscription_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::TFSubscription_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::TFSubscription_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "tf2_web_republisher_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription() -> *const std::ffi::c_void;
}

// Corresponds to tf2_web_republisher_interfaces__action__TFSubscription
#[allow(missing_docs, non_camel_case_types)]
pub struct TFSubscription;

impl rosidl_runtime_rs::Action for TFSubscription {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = TFSubscription_Goal;

  /// The result message defined in the action definition.
  type Result = TFSubscription_Result;

  /// The feedback message defined in the action definition.
  type Feedback = TFSubscription_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::TFSubscription_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::TFSubscription_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::TFSubscription_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tf2_web_republisher_interfaces__action__TFSubscription() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::TFSubscription_Goal,
  ) -> super::action::rmw::TFSubscription_SendGoal_Request {
   super::action::rmw::TFSubscription_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::TFSubscription_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::TFSubscription_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::TFSubscription_SendGoal_Response {
   super::action::rmw::TFSubscription_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::TFSubscription_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::TFSubscription_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::TFSubscription_Feedback,
  ) -> super::action::rmw::TFSubscription_FeedbackMessage {
    let mut message = super::action::rmw::TFSubscription_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::TFSubscription_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::TFSubscription_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::TFSubscription_GetResult_Request {
   super::action::rmw::TFSubscription_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::TFSubscription_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::TFSubscription_Result,
  ) -> super::action::rmw::TFSubscription_GetResult_Response {
   super::action::rmw::TFSubscription_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::TFSubscription_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::TFSubscription_Result,
  ) {
    (response.status, response.result)
  }
}


