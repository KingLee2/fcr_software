#include "../common/library_basic.h"
#include "../common/library_ros.h"
#include "../common/set_get_param.h"
#include "../common/string_Iv2.h"
#include "../common/stof.h"
#include "mission_define.h"
#include <nlohmann/json.hpp>
#include <cstdint>
#include <mutex>

using namespace std;
using json = nlohmann::json;
class config_function : public rclcpp::Node{
    public:
        config_function(const string &node_name , const string &sub_namespace) : rclcpp::Node(node_name, sub_namespace){
            rclcpp::QoS qos_profile(rclcpp::KeepLast(10));
            qos_profile.best_effort();
            mvibot_seri_ = this->get_namespace();
            mvibot_seri_f_ = mvibot_seri_;
            mvibot_seri_f_.erase(0,1);
            //init service
            //footprint padding
            set_footprint_padding_local_client_ = this->create_client<rcl_interfaces::srv::SetParameters>("local_costmap/local_costmap/set_parameters");
            get_footprint_padding_local_client_ = this->create_client<rcl_interfaces::srv::GetParameters>("local_costmap/local_costmap/get_parameters");
            set_footprint_padding_global_client_ = this->create_client<rcl_interfaces::srv::SetParameters>("global_costmap/global_costmap/set_parameters");
            get_footprint_padding_global_client_ = this->create_client<rcl_interfaces::srv::GetParameters>("global_costmap/global_costmap/get_parameters");
            //velocity
            set_desired_linear_vel_client_ = this->create_client<rcl_interfaces::srv::SetParameters>("controller_server/set_parameters");
            get_desired_linear_vel_client_ = this->create_client<rcl_interfaces::srv::GetParameters>("controller_server/get_parameters");
            //avoid_obstacle
            set_avoid_obstacle_client_ = this->create_client<rcl_interfaces::srv::SetParameters>("global_costmap/global_costmap/set_parameters");
            get_avoid_obstacle_client_ = this->create_client<rcl_interfaces::srv::GetParameters>("global_costmap/global_costmap/get_parameters");
            //init publisher
            function_state_pub_ = this->create_publisher<std_msgs::msg::String>("function_state",1);
            //init subscriber
            //
            auto config_info_callback = [this](std_msgs::msg::String msg)->void{
                parameters = json::parse(msg.data);
                // cout<<parameters<<endl;
                process_data();
                request = 1;
                status = Active_;
            };
            config_info_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/config_info", qos_profile, config_info_callback);
            //
            auto config_function_status_callback = [this](std_msgs::msg::String msg)->void{
                cout<<"config|received request,status"<<endl;
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
            config_function_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/config_function_status", qos_profile, config_function_status_callback);
            //init timer
            //
            auto action_timer_callback = [this]()->void{
                cout<<"config|request:"<<request<<"|state:"<<status<<endl;
                if(request == 1){
                    int res;
                    res = action();
                    pub_function_state(res);
                }
            };
            action_timer_ = this->create_wall_timer(50ms, action_timer_callback);
        }
        void pub_function_state(int st);
        void process_data();
        int action();
    private:
        //declare var
        string mvibot_seri_, mvibot_seri_f_;

        json parameters;
        int status = Finish_;
        int request = 0; //request = 1: yeu cau thuc thi, request = 0: khong co yeu cau thuc thi
        //footprint_padding
        string footprint_padding = "";
        //inflation_radius
        string inflation_radius = "";
        //velocity
        string desired_linear_vel = "";
        //avoid obstacle
        string avoid_obstacle = "";
        bool set_result_vel = false;
        bool set_done_vel = false;
        bool set_result_avoid_ob = false;
        bool set_done_avoid_ob = false;
        bool set_result_local = false;
        bool set_done_local = false;
        bool set_result_global = false;
        bool set_done_global = false;
        std::uint64_t config_generation_ = 0;
        std::mutex config_mutex_;
        //declare service
        //footprint padding
        rclcpp::Client<rcl_interfaces::srv::SetParameters>::SharedPtr set_footprint_padding_local_client_;
        rclcpp::Client<rcl_interfaces::srv::GetParameters>::SharedPtr get_footprint_padding_local_client_;
        rclcpp::Client<rcl_interfaces::srv::SetParameters>::SharedPtr set_footprint_padding_global_client_;
        rclcpp::Client<rcl_interfaces::srv::GetParameters>::SharedPtr get_footprint_padding_global_client_;
        //desired_linear_vel
        rclcpp::Client<rcl_interfaces::srv::SetParameters>::SharedPtr set_desired_linear_vel_client_;
        rclcpp::Client<rcl_interfaces::srv::GetParameters>::SharedPtr get_desired_linear_vel_client_;
        //avoid_obstacle
        rclcpp::Client<rcl_interfaces::srv::SetParameters>::SharedPtr set_avoid_obstacle_client_;
        rclcpp::Client<rcl_interfaces::srv::GetParameters>::SharedPtr get_avoid_obstacle_client_;
        //declare pub
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr function_state_pub_;
        //declare sub
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr config_info_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr config_function_status_sub_;
        //declare timer
        rclcpp::TimerBase::SharedPtr action_timer_;
};
void config_function::pub_function_state(int st){
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
void config_function::process_data(){
    cout<<parameters<<endl;
    const string desired_linear_vel_value = parameters["desired_linear_vel"].get<string>();
    const string footprint_padding_value = parameters["footprint_padding"].get<string>();
    const string avoid_obstacle_value = parameters["avoid_obstacle"].get<string>();
    std::lock_guard<std::mutex> lock(config_mutex_);
    desired_linear_vel = desired_linear_vel_value;
    footprint_padding = footprint_padding_value;
    avoid_obstacle = avoid_obstacle_value;
    set_result_vel = false;
    set_done_vel = false;
    set_result_local = false;
    set_done_local = false;
    set_result_global = false;
    set_done_global = false;
    set_result_avoid_ob = false;
    set_done_avoid_ob = false;
    ++config_generation_;
}

int config_function::action(){
    cout<<"config|status: "<<status<<endl;
    if(status != Active_) return status;

    string desired_linear_vel_value;
    string footprint_padding_value;
    string avoid_obstacle_value;
    std::uint64_t generation;
    {
        std::lock_guard<std::mutex> lock(config_mutex_);
        desired_linear_vel_value = desired_linear_vel;
        footprint_padding_value = footprint_padding;
        avoid_obstacle_value = avoid_obstacle;
        generation = config_generation_;
    }

    auto process_parameter = [this, generation](
        bool& result_flag,
        bool& done_flag,
        const auto& set_client,
        const auto& get_client,
        const string& name,
        const auto& value) {
        bool start_request = false;
        bool already_complete = false;
        {
            std::lock_guard<std::mutex> lock(config_mutex_);
            if(generation != config_generation_) return true;
            already_complete = result_flag;
            if(!already_complete && !done_flag){
                done_flag = true;
                start_request = true;
            }
        }
        if(already_complete) return false;
        if(start_request){
            bool* result_flag_ptr = &result_flag;
            bool* done_flag_ptr = &done_flag;
            check_param(set_client, get_client, name, value,
                [this, generation, result_flag_ptr, done_flag_ptr](bool res){
                    std::lock_guard<std::mutex> lock(config_mutex_);
                    if(generation != config_generation_) return;
                    *result_flag_ptr = res;
                    if(!res) *done_flag_ptr = false;
                }
            );
        }
        return true;
    };

    if(desired_linear_vel_value != ""){
        double desired_linear_vel_set = stod_f(desired_linear_vel_value);
        if(desired_linear_vel_set > 0.5) desired_linear_vel_set = 0.5;
        if(process_parameter(set_result_vel, set_done_vel,
            set_desired_linear_vel_client_, get_desired_linear_vel_client_,
            "FollowPath.desired_linear_vel", desired_linear_vel_set)){
            return Active_;
        }
    }

    if(footprint_padding_value != ""){
        double footprint_padding_set = stod_f(footprint_padding_value);
        if(footprint_padding_set < 0.0) footprint_padding_set = 0.0;
        if(process_parameter(set_result_local, set_done_local,
            set_footprint_padding_local_client_, get_footprint_padding_local_client_,
            "footprint_padding", footprint_padding_set)){
            return Active_;
        }
        if(process_parameter(set_result_global, set_done_global,
            set_footprint_padding_global_client_, get_footprint_padding_global_client_,
            "footprint_padding", footprint_padding_set)){
            return Active_;
        }
    }

    if(avoid_obstacle_value != ""){
        const bool avoid_obstacle_set = avoid_obstacle_value == "true";
        if(process_parameter(set_result_avoid_ob, set_done_avoid_ob,
            set_avoid_obstacle_client_, get_avoid_obstacle_client_,
            "obstacle_layer.enabled", avoid_obstacle_set)){
            return Active_;
        }
    }

    {
        std::lock_guard<std::mutex> lock(config_mutex_);
        if(generation != config_generation_) return Active_;
    }
    status = Finish_;
    request = 0;
    return Finish_;
}
