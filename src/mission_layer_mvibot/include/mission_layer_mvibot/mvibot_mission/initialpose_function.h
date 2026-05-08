#include "../common/library_basic.h"
#include "../common/library_ros.h"
#include "../common/string_Iv2.h"
#include "../common/stof.h"
#include "../common/stoi.h"
#include "mission_define.h"
#include <nlohmann/json.hpp>

using namespace std;
using json = nlohmann::json;

class initialpose_function : public rclcpp::Node{
    private:
        //declare var
        string mvibot_seri_, mvibot_seri_f_;
        json parameters;
        int status = Finish_;
        int request = 0; //request = 1: yeu cau thuc thi, request = 0: khong co yeu cau thuc thi
        geometry_msgs::msg::PoseWithCovarianceStamped initialPose;
        geometry_msgs::msg::PoseWithCovarianceStamped robotPose;
        //declare pub
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr function_state_pub_;
        rclcpp::Publisher<geometry_msgs::msg::PoseWithCovarianceStamped>::SharedPtr initialize_pose_pub_;
        //declare sub
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr initialpose_info_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr initialpose_function_status_sub_;
        rclcpp::Subscription<geometry_msgs::msg::PoseWithCovarianceStamped>::SharedPtr robot_pose_sub_;
        //declare timer
        rclcpp::TimerBase::SharedPtr action_timer_;
        
    public:
        initialpose_function(const string &node_name, const string &sub_namespace) : Node(node_name, sub_namespace){
            rclcpp::QoS qos_profile(rclcpp::KeepLast(10));
            qos_profile.best_effort();
            rclcpp::QoS qos_amcl(rclcpp::KeepLast(1));
            qos_amcl.best_effort();
            qos_amcl.durability_volatile();
            mvibot_seri_ = this->get_namespace();
            mvibot_seri_f_ = mvibot_seri_;
            mvibot_seri_f_.erase(0,1);
            //init publisher
            function_state_pub_ = this->create_publisher<std_msgs::msg::String>("function_state",1);
            initialize_pose_pub_ = this->create_publisher<geometry_msgs::msg::PoseWithCovarianceStamped>("initialpose",qos_amcl);
            //init subscriber
            auto robot_pose_callback = [this](geometry_msgs::msg::PoseWithCovarianceStamped msg)->void{
                robotPose = msg;
            };
            robot_pose_sub_ = this->create_subscription<geometry_msgs::msg::PoseWithCovarianceStamped>(mvibot_seri_ + "/amcl_pose",qos_profile,robot_pose_callback);
            //
            auto initialpose_info_callback = [this](std_msgs::msg::String msg)->void{
                parameters = json::parse(msg.data);
                // cout<<parameters<<endl;
                process_data();
                request = 1;
            };
            initialpose_info_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/initialpose_info", qos_profile, initialpose_info_callback);
            //
            auto initialpose_function_status_callback = [this](std_msgs::msg::String msg)->void{
                cout<<"initialpose|received request,status"<<endl;
                if(msg.data == "active"){
                    request = 1;
                    status = Active_;
                }
                else if(msg.data == "stop") {
                    request = 1;
                    status = Stop_;
                }
                else if(msg.data == "error") {
                    status = Error_;
                    request = 0;
                }
                else if(msg.data == "cancel") {
                    request = 0;
                    status = Cancel_;
                }
                else if(msg.data == "finish") {
                    request = 0;
                    status = Finish_;
                }
            };
            initialpose_function_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/initialpose_function_status", qos_profile, initialpose_function_status_callback);
            //init timer
            //
            auto action_timer_callback = [this]()->void{
                cout<<"initialpose|request:"<<request<<"|state:"<<status<<endl;
                if(request == 1){
                    int res;
                    res = action();
                    pub_function_state(res);
                }
            };
            action_timer_ = this->create_wall_timer(1000ms, action_timer_callback);
        }
        float getyaw(double data1, double data2);
        int check_robot_position();
        void pub_function_state(int st);
        void process_data();
        int action();
};
float initialpose_function::getyaw(double data1, double data2){
    geometry_msgs::msg::Quaternion quat_msg;
    double roll, pitch, yaw;
    tf2::Quaternion quat_tf;
    quat_msg.x=0;
    quat_msg.y=0;
    quat_msg.z=data1;
    quat_msg.w=data2;
    tf2::fromMsg(quat_msg, quat_tf);
    tf2::Matrix3x3(quat_tf).getRPY(roll, pitch, yaw);
    return yaw;
}
int initialpose_function::check_robot_position(){
    float dis;
    float angle1, angle2;
    float x_robot,y_robot,z_robot,w_robot,x_init,y_init,z_init,w_init;
    //
    x_init = initialPose.pose.pose.position.x;
    y_init = initialPose.pose.pose.position.y;
    z_init = initialPose.pose.pose.orientation.z;
    w_init = initialPose.pose.pose.orientation.w;
    //
    x_robot = robotPose.pose.pose.position.x;
    y_robot = robotPose.pose.pose.position.y;
    z_robot = robotPose.pose.pose.orientation.z;
    w_robot = robotPose.pose.pose.orientation.w;
    //
    dis = sqrt((x_robot-x_init)*(x_robot-x_init)+(y_robot-y_init)*(y_robot-y_init));
    angle1 = getyaw(z_init,w_init);
    angle2 = getyaw(z_robot,w_robot);

    //
    if(dis <= 0.05 && fabs(sin(angle2)-sin(angle1))<=0.05 && fabs(cos(angle2)-cos(angle1))<=0.05) return 1;
    return 0;
}
void initialpose_function::pub_function_state(int st){
    std_msgs::msg::String msg;
    if(st == Active_) msg.data = "active";
    else if(st == Finish_) msg.data = "finish";
    else if(st == Error_) msg.data = "error";
    else if(st == Cancel_) msg.data = "cancel";
    else if(st == Stop_) msg.data = "stop";
    else if(st == True_) msg.data = "true";
    else if(st == False_) msg.data = "false";
    function_state_pub_->publish(msg);
}
void initialpose_function::process_data(){
    initialPose.header.frame_id = "map";
    initialPose.pose.pose.position.x = std::stod(parameters["x"].get<string>());
    initialPose.pose.pose.position.y = std::stod(parameters["y"].get<string>());
    initialPose.pose.pose.position.z = 0.0;
    initialPose.pose.pose.orientation.x = 0.0;
    initialPose.pose.pose.orientation.y = 0.0;
    initialPose.pose.pose.orientation.z = std::stod(parameters["z"].get<string>());
    initialPose.pose.pose.orientation.w = std::stod(parameters["w"].get<string>());
    initialPose.pose.covariance[0]=0.25;
    initialPose.pose.covariance[7]=0.25;
    initialPose.pose.covariance[35]=0.06853892326654787;
}
int initialpose_function::action(){
    if(status == Active_){
        int check_res = 0;
        check_res = check_robot_position();
        if(check_res == 1){
            status = Finish_;
            request = 0;
            return Finish_;
        }
        else {
            process_data();
            initialize_pose_pub_->publish(initialPose);
            return Active_;
        }
    }else{
        return status;
    }
}