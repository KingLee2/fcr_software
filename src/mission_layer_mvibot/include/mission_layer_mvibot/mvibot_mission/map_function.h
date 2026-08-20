#include "../common/library_basic.h"
#include "../common/library_ros.h"
#include "../common/string_Iv2.h"
#include "../common/stof.h"
#include "../common/stoi.h"
#include "mission_define.h"
#include <nlohmann/json.hpp>

using namespace std;
using json = nlohmann::json;

class loadmap_function : public rclcpp::Node{
    private:
        //declare var
        string mvibot_seri_, mvibot_seri_f_;
        string map_name ="";
        json parameters;
        int status = Finish_;
        int request = 0; //request = 1: yeu cau thuc thi, request = 0: khong co yeu cau thuc thi
        int result_srv = 0;
        //declare pub
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr function_state_pub_;
        //declare sub
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr loadmap_info_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr loadmap_function_status_sub_;
        //declare service
        rclcpp::Client<nav2_msgs::srv::LoadMap>::SharedPtr load_map_srv_;
        //declare timer
        rclcpp::TimerBase::SharedPtr action_timer_;
        
    public:
        loadmap_function(const string &node_name, const string &sub_namespace) : Node(node_name, sub_namespace){
            rclcpp::QoS qos_profile(rclcpp::KeepLast(10));
            qos_profile.best_effort();
            mvibot_seri_ = this->get_namespace();
            mvibot_seri_f_ = mvibot_seri_;
            mvibot_seri_f_.erase(0,1);
            //init publisher
            function_state_pub_ = this->create_publisher<std_msgs::msg::String>("function_state",1);
            //init subscriber
            //
            auto loadmap_info_callback = [this](std_msgs::msg::String msg)->void{
                parameters = json::parse(msg.data);
                // cout<<parameters<<endl;
                process_data();
                // request = 1;
            };
            loadmap_info_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/loadmap_info", qos_profile, loadmap_info_callback);
            //
            auto loadmap_function_status_callback = [this](std_msgs::msg::String msg)->void{
                cout<<"loadmap|received request,status"<<endl;
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
            loadmap_function_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/loadmap_function_status", qos_profile, loadmap_function_status_callback);
            //init service
            load_map_srv_=this->create_client<nav2_msgs::srv::LoadMap>("map_server/load_map");
            //init timer
            //
            auto action_timer_callback = [this]()->void{
                cout<<"loadmap|request:"<<request<<"|state:"<<status<<endl;
                if(request == 1){
                    int res;
                    res = action();
                    pub_function_state(res);
                }
            };
            action_timer_ = this->create_wall_timer(1000ms, action_timer_callback);
        }
        int load_map(string map_url);
        void pub_function_state(int st);
        void process_data();
        int action();
};
int loadmap_function::load_map(string map_url){
    RCLCPP_INFO(rclcpp::get_logger("Map"), "Load Map");
    if(!load_map_srv_->wait_for_service(std::chrono::duration<float>(0.5))){
        RCLCPP_INFO(rclcpp::get_logger("Map"),"Load Map service not available");
        return 0;
    }
    //send request
    auto req = std::make_shared<nav2_msgs::srv::LoadMap_Request>();
    req->map_url = map_url;
    auto load_map_service_callback = [this](rclcpp::Client<nav2_msgs::srv::LoadMap>::SharedFuture result){
        if(result.get()->result == nav2_msgs::srv::LoadMap::Response::RESULT_SUCCESS){
            RCLCPP_INFO(rclcpp::get_logger("Map"),"Load Map successed");
            result_srv = 3;
        }
        else {
            RCLCPP_INFO(rclcpp::get_logger("Map"),"Load Map not successed");
            result_srv = 2;
        }
    };
    auto future = load_map_srv_->async_send_request(req,load_map_service_callback);
    result_srv = 1;
    return 1;
}
void loadmap_function::pub_function_state(int st){
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
void loadmap_function::process_data(){
    map_name = parameters["map_name"].get<string>();
    cout<<"Map Name: "<<map_name<<endl;
}
int loadmap_function::action(){
    if(status == Active_){
        if(result_srv == 0){
            int loadmap_srv_available = 0;
            loadmap_srv_available = load_map("/assets/maps/" + map_name + ".yaml");
            if(loadmap_srv_available == 1) return Active_;
            else{
                result_srv = 0;
                status = Error_;
                request = 0;
                return Error_;
            }
        }
        else if(result_srv == 1) return Active_;
        else if(result_srv == 2) {
            result_srv = 0;
            status = Active_;
            request = 1;
            return Active_;
        }
        else if(result_srv == 3){
            result_srv = 0;
            status = Finish_;
            request = 0;
            return Finish_;
        }
    }else{
        return status;
    }
}
