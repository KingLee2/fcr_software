#include "mvibot_server_init.h"
using namespace std;
using json = nlohmann::json;
class fcr_server : public rclcpp::Node{
    private:
        //robot var
        string mvibot_seri_, mvibot_seri_f_;
        string mode="";
        //map var
        nav_msgs::msg::OccupancyGrid my_map;
        int save_map_ = -1;
        int load_map_ = -1;
        int map_server_state = -1, slam_toolbox_state = -1;
	    bool map_transition_trigger=false, slam_transition_trigger=false;
        int load_map_first = 0;
        //curl
        CURL* curl;
        CURLcode result;
        std::string readBuffer;
        //declare pub//
        rclcpp::Publisher<nav_msgs::msg::OccupancyGrid>::SharedPtr map_pub_;
        //declare sub//
        //sub map
        rclcpp::Subscription<nav_msgs::msg::OccupancyGrid>::SharedPtr map_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr map_request_sub_;
        //sub robot
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr robot_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr infor_robot_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr sensor_status_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr battery_status_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr battery_cell_status_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr motor_right_status_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr motor_left_status_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr input_status_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr output_status_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr robot_config_sub_;
        //declare service //
        rclcpp::Client<nav2_msgs::srv::SaveMap>::SharedPtr save_map_srv_;
        rclcpp::Client<nav2_msgs::srv::LoadMap>::SharedPtr load_map_srv_;
	    rclcpp::Client<slam_toolbox::srv::SerializePoseGraph>::SharedPtr save_serialize_map_srv_;
        rclcpp::Client<lifecycle_msgs::srv::GetState>::SharedPtr get_state_map_server_srv_;
        rclcpp::Client<lifecycle_msgs::srv::GetState>::SharedPtr get_state_slam_srv_;
	    rclcpp::Client<lifecycle_msgs::srv::ChangeState>::SharedPtr change_state_map_server_srv_;
        rclcpp::Client<lifecycle_msgs::srv::ChangeState>::SharedPtr change_state_slam_toolbox_srv_;
        //declare timer
        rclcpp::TimerBase::SharedPtr map_timer_;
        rclcpp::TimerBase::SharedPtr check_map_server_timer_;

    public:
        fcr_server(const string& node_name, const string& sub_namespace) : Node(node_name, sub_namespace){
            mvibot_seri_ = this->get_namespace();
            mvibot_seri_f_ = mvibot_seri_;
            mvibot_seri_f_.erase(0,1);
            mode = load_file("mode");
            load_map_first = 1;
            RCLCPP_INFO(rclcpp::get_logger("robot"),"mode: %s",mode.c_str());
            rclcpp::QoS qos_profile(rclcpp::KeepLast(5));
            qos_profile.best_effort();
            rclcpp::QoS qos_profile_map(rclcpp::KeepLast(1));
            qos_profile_map.reliability(RMW_QOS_POLICY_RELIABILITY_RELIABLE);
            qos_profile_map.durability(RMW_QOS_POLICY_DURABILITY_TRANSIENT_LOCAL);
            //init curl
            curl_global_init(CURL_GLOBAL_DEFAULT);
            curl = curl_easy_init();
            //init pub//
            map_pub_ = this->create_publisher<nav_msgs::msg::OccupancyGrid>("map2",qos_profile_map);
            //init sub//
            //get robot
            auto robot_callback = [this](std_msgs::msg::String msg)->void{
                static int is_have = 0;
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                if(data.data1.size()==2){
                    for(int i=0;i<my_robots.size();i++){
                        if(my_robots[i].name_seri==data.data1[0]) {
                            is_have=1;
                            my_robots[i].time_out=0;
                        }
                    }
                    if(is_have==0){
                        my_robots.resize(my_robots.size()+1);
                        my_robots[my_robots.size()-1].name_seri=data.data1[0];
                        my_robots[my_robots.size()-1].type=data.data1[1];
                        my_robots[my_robots.size()-1].id=my_robots.size();
                        cout << "robot_id: "<<my_robots[my_robots.size()-1].id<<endl;
                        my_robots[my_robots.size()-1].update_database=0;
                    }
                }  

            };
            robot_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/robot",qos_profile, robot_callback);
            //get robot status
            auto robot_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].robot_status=msg.data;
                        break;
                    }
                }
            };
            infor_robot_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/robot_status", qos_profile, robot_status_callback);
            //get sensor status
            auto sensor_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].sensor_status=msg.data;
                        break;
                    }
                }
            };
            sensor_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/sensor_status", qos_profile, sensor_status_callback);
            //get battery status
            auto battery_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].battery_status=msg.data;
                        // my_robots[i].battery_status_chart=data.data1[0] + "|" + data.data1[1];
                        break;
                    }
                }
            };
            battery_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/battery_status", qos_profile, battery_status_callback);
            //get cell battery status
            auto battery_cell_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].battery_cell_status=msg.data;
                        break;
                    }
                }
            };
            battery_cell_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/battery_cell_status", qos_profile, battery_cell_status_callback);
            //get motor right status
            auto motor_right_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].motor_right_status=msg.data;
                        break;
                    }
                }
            };
            motor_right_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/motor_right_status", qos_profile, motor_right_status_callback);
            //get motor left status
            auto motor_left_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].motor_left_status=msg.data;
                        break;
                    }
                }
            };
            motor_left_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/motor_left_status", qos_profile, motor_left_status_callback);
            //get input status
            auto input_user_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].input_user_status=msg.data;
                        break;
                    }
                }
            };
            input_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/input_user_status_string", qos_profile, input_user_status_callback);
            //get output status
            auto output_user_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].output_user_status=msg.data;
                        break;
                    }
                }
            };
            output_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/output_user_status_string", qos_profile, output_user_status_callback);
            //get robot config
            auto robot_config_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                std::lock_guard<std::recursive_mutex> lock(mutex_robot);
                data.detect(msg.data,"","|","");
                for(int i=0;i<my_robots.size();i++){
                    if(my_robots[i].name_seri==data.data1[0]){
                        my_robots[i].robot_config_status=msg.data;
                        break;
                    }
                }
            };
            robot_config_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/config_robot", qos_profile, robot_config_callback);
            //get map
            auto map_callback = [this](nav_msgs::msg::OccupancyGrid msg)->void{
                my_map = msg;
            };
            map_sub_ = this->create_subscription<nav_msgs::msg::OccupancyGrid>(mvibot_seri_+"/map", qos_profile_map, map_callback);
            //get request map
            auto request_map_callback = [this](std_msgs::msg::String msg)->void{ //msg="request:0|action:active_map|name_map:...", request(save_map, delete_map, active_map)
                static string data;
                static string_Iv2 data1;
                std::lock_guard<std::recursive_mutex> lock(mutex_common);
                data = msg.data;
                data1.detect(data,"","|","");
                for(int i=0; i<data1.data1.size();i++){
                    static string_Iv2 data2;
                    data2.detect(data1.data1[i],"",":","");
                    if(data2.data1[0]=="request") request_map = stoi(data2.data1[1]);
                    else if(data2.data1[0]=="action") action_map = data2.data1[1];
                    else if(data2.data1[0]=="name_map") name_map = data2.data1[1];
                }
                if(action_map == "active_map" || action_map == "update_map") name_map_active =name_map;
            };
            map_request_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/request_map",qos_profile, request_map_callback);
            //init service
            load_map_srv_=this->create_client<nav2_msgs::srv::LoadMap>("map_server/load_map");
            save_map_srv_=this->create_client<nav2_msgs::srv::SaveMap>("map_saver/save_map");
	        save_serialize_map_srv_ = this->create_client<slam_toolbox::srv::SerializePoseGraph>("/slam_toolbox/serialize_map");
            get_state_map_server_srv_ = this->create_client<lifecycle_msgs::srv::GetState>("map_server/get_state");
            get_state_slam_srv_ = this->create_client<lifecycle_msgs::srv::GetState>("/slam_toolbox/get_state");
	        change_state_map_server_srv_ = this->create_client<lifecycle_msgs::srv::ChangeState>("map_server/change_state");
            change_state_slam_toolbox_srv_ = this->create_client<lifecycle_msgs::srv::ChangeState>("/slam_toolbox/change_state");
            //init timer//
            auto map_timer_callback = [this]()->void{
                std::lock_guard<std::recursive_mutex> lock(mutex_common);
                if(map_server_state == 3 || slam_toolbox_state == 3){
                    if(load_map_first == 1 && map_server_state == 3){
                        if(name_map_active!=""){
                            string map_url;
			                map_url = "/assets/maps/" + name_map_active + ".yaml";
                            load_map(map_url);
                            process_map_active = 1;
                            update_map_database = 1;
                            action_map = "active_map";
                        }
                        load_map_first = 0;
                    }
                    //execute request(save_map, delete_map, active_map)
                    process_request_map();
                    if(process_map_active == 1){
                        //pub map with layer map
                        // cout<<"start pub map"<<endl;
                        pub_map();
                        process_map_active = 0;
                    }
                }
		        else if(map_server_state != 3 && map_server_state != -1){
                    change_state_map_server(map_server_state);
                }
                else if(slam_toolbox_state != 3 && slam_toolbox_state != -1){
                    change_state_slam_toolbox(slam_toolbox_state);
                }
                //execute request(save_map, delete_map, active_map)
                process_request_map();
                if(process_map_active == 1){
                    //pub map with layer map
                    // cout<<"start pub map"<<endl;
                    pub_map();
                    process_map_active = 0;
                }
            };
            map_timer_ = this->create_wall_timer(100ms, map_timer_callback);
            auto check_map_server_timer_callback = [this]()->void{
                if(mode == "navigation") get_state_map_server();  
                else get_state_slam_toolbox();
            };
            check_map_server_timer_ = this->create_wall_timer(5000ms, check_map_server_timer_callback);
        }
        static size_t WriteCallback(void* contents, size_t size, size_t nmemb, void* userp){
            ((std::string*)userp)->append((char*)contents, size * nmemb);
            return size * nmemb;
        }
        double getyaw(geometry_msgs::msg::Quaternion quat_msg);
        string load_file(string name_file);
        nav_msgs::msg::OccupancyGrid get_map_select(string path);
        void pub_map();
        int load_map(string map_url);
        int save_map(string map_topic, string map_url);
	    int save_serialize_map(string map_url);
        void get_state_map_server();
        void get_state_slam_toolbox();
	    void change_state_map_server(int map_server_state);
        void change_state_slam_toolbox(int slam_toolbox_state);
        void process_request_map(); 
};
string fcr_server::load_file(string name_file){
    //
    static string value_return;
    try
    {
	    std::ifstream file(define_path+"config/"+name_file);
	    std::string str; 
	    std::string data;
        std::getline(file, str);
        value_return=str;
    }
    catch(const std::exception& e)
    {
        std::cerr << e.what() << '\n';
        value_return="-1";
    }
    return value_return;
}
int fcr_server::load_map(string map_url){
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
            load_map_ = 1;
        }
        else {
            RCLCPP_INFO(rclcpp::get_logger("Map"),"Load Map not successed");
            load_map_ = 0;
        }
    };
    auto future = load_map_srv_->async_send_request(req,load_map_service_callback);
    return 1;
}
int fcr_server::save_map(string map_topic, string map_url){
    RCLCPP_INFO(rclcpp::get_logger("Map"), "Save Map");
    while(!save_map_srv_->wait_for_service(std::chrono::duration<float>(0.5))){
        RCLCPP_INFO(rclcpp::get_logger("Map"),"Save Map service not available");
        return 0;
    }
    //send request
    auto req = std::make_shared<nav2_msgs::srv::SaveMap_Request>();
    req->map_topic = map_topic;
    req->map_url = map_url;
    req->image_format = "pgm";
    req->map_mode = "trinary";
    req->free_thresh = 0.25;
    req->occupied_thresh = 0.65;
    auto save_map_service_callback = [this](rclcpp::Client<nav2_msgs::srv::SaveMap>::SharedFuture result){
        if(result.get()->result == 1){
            RCLCPP_INFO(rclcpp::get_logger("Map"),"Save Map successed");
            save_map_ = 1;
        }
        else {
            RCLCPP_INFO(rclcpp::get_logger("Map"),"Save Map not successed");
            save_map_ = 0;
        }
    };
    auto future = save_map_srv_->async_send_request(req,save_map_service_callback);
    return 1;
}
int fcr_server::save_serialize_map(string map_url){
    RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"), "Save Serialize Map");
    while(!save_serialize_map_srv_->wait_for_service(std::chrono::duration<float>(0.5))){
        RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"Save Serialize Map service not available");
        return 0;
    }
    //send request
    auto req = std::make_shared<slam_toolbox::srv::SerializePoseGraph::Request>();
    req->filename = map_url;
    auto save_serialize_map_service_callback = [this](rclcpp::Client<slam_toolbox::srv::SerializePoseGraph>::SharedFuture result){
        if(result.get()->result == 0){
            RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"Save Serialize Map successed");
        }
        else {
            RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"Save Serialize Map not successed");
        }
    };
    auto future = save_serialize_map_srv_->async_send_request(req,save_serialize_map_service_callback);
    return 1;
}
void fcr_server::get_state_map_server(){
    RCLCPP_INFO(rclcpp::get_logger("Map"), "Get state map server");
    if(!get_state_map_server_srv_->wait_for_service(std::chrono::duration<float>(0.5))){
        RCLCPP_INFO(rclcpp::get_logger("Map"),"Get state map server service not available");
        return ;
    }
    //send request
    auto req = std::make_shared<lifecycle_msgs::srv::GetState::Request>();
    auto get_state_map_server_service_callback = [this](rclcpp::Client<lifecycle_msgs::srv::GetState>::SharedFuture result){
        auto res = result.get();
        RCLCPP_INFO(rclcpp::get_logger("Map"),"%s",res->current_state.label.c_str());
        if(res->current_state.label == "active"){
            RCLCPP_INFO(rclcpp::get_logger("Map"),"Map server is ACTIVE");
            map_server_state = res->current_state.id;
        }
    };
    auto future = get_state_map_server_srv_->async_send_request(req,get_state_map_server_service_callback);
}
void fcr_server::get_state_slam_toolbox(){
    RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"), "Get state slam toolbox");
    if(!get_state_slam_srv_->wait_for_service(std::chrono::duration<float>(0.5))){
        RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"Get state slam toolbox service not available");
        return ;
    }
    //send request
    auto req = std::make_shared<lifecycle_msgs::srv::GetState::Request>();
    auto get_state_slam_toolbox_service_callback = [this](rclcpp::Client<lifecycle_msgs::srv::GetState>::SharedFuture result){
        auto res = result.get();
        RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"%s",res->current_state.label.c_str());
        if(res->current_state.label == "active"){
            RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"Slam Toolbox is ACTIVE");
            slam_toolbox_state = res->current_state.id;
        }
    };
    auto future = get_state_slam_srv_->async_send_request(req,get_state_slam_toolbox_service_callback);
}
void fcr_server::change_state_map_server(int map_server_state){
    RCLCPP_INFO(rclcpp::get_logger("Map"), "Change state map server");
    if(!change_state_map_server_srv_->wait_for_service(std::chrono::duration<float>(0.5))){
        RCLCPP_INFO(rclcpp::get_logger("Map"),"Change state map server service not available");
        return ;
    }
    //send request
    auto req = std::make_shared<lifecycle_msgs::srv::ChangeState::Request>();
    if(map_server_state == 1) req->transition.id = lifecycle_msgs::msg::Transition::TRANSITION_CONFIGURE;
    else if(map_server_state == 2) req->transition.id = lifecycle_msgs::msg::Transition::TRANSITION_ACTIVATE;
    else return;
    auto change_state_map_server_service_callback = [this](rclcpp::Client<lifecycle_msgs::srv::ChangeState>::SharedFuture result){
        std::shared_ptr<lifecycle_msgs::srv::ChangeState_Response> res;
        try{
            res = result.get();
        }
        catch(const std::exception &e){
            RCLCPP_ERROR(rclcpp::get_logger("Map"),"ChangeState exception: %s",e.what());
        }
        RCLCPP_INFO(rclcpp::get_logger("Map"),"%d",res->success);
        if(res->success == true){
            RCLCPP_INFO(rclcpp::get_logger("Map"),"Map server is ACTIVE");
        }
        else{
            RCLCPP_INFO(rclcpp::get_logger("Map"),"Change state MAP SERVER fail");
        }
	map_transition_trigger = false;
    };
    if(map_transition_trigger) return;
    map_transition_trigger = true;
    auto future = change_state_map_server_srv_->async_send_request(req,change_state_map_server_service_callback);
}
void fcr_server::change_state_slam_toolbox(int slam_toolbox_state){
    RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"), "Change state slam toolbox");
    if(!change_state_slam_toolbox_srv_->wait_for_service(std::chrono::duration<float>(0.5))){
        RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"Change state slam toolbox service not available");
        return ;
    }
    //send request
    auto req = std::make_shared<lifecycle_msgs::srv::ChangeState::Request>();
    if(slam_toolbox_state == 1) req->transition.id = lifecycle_msgs::msg::Transition::TRANSITION_CONFIGURE;
    else if(slam_toolbox_state == 2) req->transition.id = lifecycle_msgs::msg::Transition::TRANSITION_ACTIVATE;
    else return;
    auto change_state_slam_toolbox_service_callback = [this](rclcpp::Client<lifecycle_msgs::srv::ChangeState>::SharedFuture result){
        std::shared_ptr<lifecycle_msgs::srv::ChangeState_Response> res;
        try{
            res = result.get();
        }
        catch(const std::exception &e){
            RCLCPP_ERROR(rclcpp::get_logger("Slam Toolbox"),"ChangeState exception: %s",e.what());
        }
        RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"%d",res->success);
        if(res->success == true){
            RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"Slam Toolbox is ACTIVE");
        }
        else{
            RCLCPP_INFO(rclcpp::get_logger("Slam Toolbox"),"Change state Slam Toolbox fail");
        }
	slam_transition_trigger = false;
    };
    if(slam_transition_trigger) return;
    slam_transition_trigger = true;
    auto future = change_state_slam_toolbox_srv_->async_send_request(req,change_state_slam_toolbox_service_callback);
}
double fcr_server::getyaw(geometry_msgs::msg::Quaternion quat_msg){
    //get angle around Z
    double roll, pitch, yaw;
    tf2::Quaternion quat_tf;
    tf2::fromMsg(quat_msg, quat_tf);
    tf2::Matrix3x3(quat_tf).getRPY(roll, pitch, yaw);
    return yaw;
}
void fcr_server::pub_map(){
    string api_str="";
    nav_msgs::msg::OccupancyGrid output_map;
    json layer_map_json;
    //reset buffer and curl
    readBuffer.clear();
    curl_easy_reset(curl);
    if (curl) {
        //get api
        api_str = "http://127.0.0.1:6430/api/v1/layers/"+name_map_active;
        RCLCPP_INFO(this->get_logger(),"layer api: %s", api_str.c_str());
        //get data
        curl_easy_setopt(curl, CURLOPT_URL, api_str.c_str());
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &readBuffer);

        result = curl_easy_perform(curl);

        if (result != CURLE_OK) {
            std::cerr << "curl_easy_perform() failed: " << curl_easy_strerror(result) << std::endl;
            RCLCPP_INFO(this->get_logger(),"GET API FAIL");
            return;
        }
        try {
            RCLCPP_INFO(this->get_logger(),"load layer map");
            layer_map_json = json::parse(readBuffer);
            if(layer_map_json.is_null()){
                RCLCPP_ERROR(this->get_logger(),"layer map json has null");
                return;
            }
        }
        catch (std::exception& e) {
            std::cerr << "JSON parse error: " << e.what() << std::endl;
        }
        if (my_map.info.width == 0 || my_map.info.height == 0 || my_map.info.resolution <= 0.0 || my_map.data.size() != static_cast<size_t>(my_map.info.width)*my_map.info.height){
            RCLCPP_ERROR(this->get_logger(),"Invalid source OccupancyGrid");
            return;
        }
        output_map = my_map;
        const auto &info = my_map.info;
        const double resolution = info.resolution;
        const double origin_x = info.origin.position.x;
        const double origin_y = info.origin.position.y;
        const double yaw_origin = getyaw(info.origin.orientation);
        const double cos_map = std::cos(yaw_origin);
        const double sin_map = std::sin(yaw_origin);
        size_t modified_cells = 0;
        for (const auto &layer : layer_map_json){
            try {
                if (!layer.is_object() || !layer.contains("type") || !layer.contains("x") || !layer.contains("y") || !layer.contains("width") || !layer.contains("height") || !layer.contains("yaw")){
                    RCLCPP_WARN(this->get_logger(),"Skip layer: missing required fields");
                    continue;
                }
                if (!layer["type"].is_string() || !layer["x"].is_number() || !layer["y"].is_number() || !layer["width"].is_number() || !layer["height"].is_number() || !layer["yaw"].is_number()){
                    RCLCPP_WARN(this->get_logger(),"Skip layer: invalid field types");
                    continue;
                }
                const std::string type = layer["type"].get<std::string>();
                if (type != "occupied" && type != "free") {
                    RCLCPP_WARN(this->get_logger(),"Skip layer: unsupported type '%s'",type.c_str());
                    continue;
                }
                const double cx = layer["x"].get<double>();
                const double cy = layer["y"].get<double>();
                const double width = layer["width"].get<double>();
                const double height = layer["height"].get<double>();
                const double yaw = layer["yaw"].get<double>();
                if (!std::isfinite(cx) || !std::isfinite(cy) || !std::isfinite(width) ||!std::isfinite(height) || !std::isfinite(yaw) || width <= 0.0 || height <= 0.0){
                    RCLCPP_WARN(this->get_logger(), "Skip layer: invalid geometry");
                    continue;
                }
                const double cos_yaw = std::cos(yaw*M_PI/180);
                const double sin_yaw = std::sin(yaw*M_PI/180);
                // OccupancyGrid Value: occupied = 100, free = 0.
                const int8_t cell_value = (type == "occupied") ? 100 : 0;
                // Iterate through the OccupancyGrid in row-major order.
                for (uint32_t gy = 0; gy < info.height; ++gy) {
                    for (uint32_t gx = 0; gx < info.width; ++gx) {
                        // Grid cell center in the map's local coordinate system.
                        const double local_x =(static_cast<double>(gx) + 0.5) * resolution;
                        const double local_y =(static_cast<double>(gy) + 0.5) * resolution;
                        // Transform the grid cell center to the world coordinate system.
                        const double world_x = origin_x + cos_map * local_x - sin_map * local_y;
                        const double world_y = origin_y + sin_map * local_x + cos_map * local_y;
                        // Convert world coordinates to the local coordinate system of the rectangular layer.
                        const double dx = world_x - cx;
                        const double dy = world_y - cy;
                        const double rect_x = cos_yaw * dx + sin_yaw * dy;
                        const double rect_y = -sin_yaw * dx + cos_yaw * dy;

                        // Check if the grid cell lies within the rectangle.
                        if (std::abs(rect_x) <= width * 0.5 && std::abs(rect_y) <= height * 0.5){
                            const size_t index =static_cast<size_t>(gy) * info.width + gx;
                            output_map.data[index] = cell_value;
                            ++modified_cells;
                        }
                    }
                }
            }
            catch (const json::exception &e) {
                RCLCPP_WARN(this->get_logger(),"Skip invalid layer: %s", e.what());
                continue;
            }
        }
        RCLCPP_INFO(this->get_logger(), "Applied %zu virtual layer cell modifications", modified_cells);
        map_pub_->publish(output_map);
        RCLCPP_INFO(this->get_logger(), "Published virtual-layer map: %s, layers=%zu", name_map_active.c_str(), layer_map_json.size());
    }
}
void fcr_server::process_request_map(){
    // RCLCPP_INFO(rclcpp::get_logger("Map"),"check process request map");
    if(request_map == 1){
        if(action_map == "save_map"){
            string map_topic, map_url;
            map_topic = "/map";
	        map_url = "/assets/maps/" + name_map;
            save_map(map_topic, map_url);
	        save_serialize_map(map_url);
        }
        else if(action_map == "delete_map"){
            static string cmd;
            cmd="";
	        cmd=cmd+"sudo rm "+"/assets/maps/"+name_map+".*";
            system(cmd.c_str());
        }
        else if(action_map == "active_map"){
            string map_url;
	        map_url = "/assets/maps/" + name_map_active + ".yaml";
            load_map(map_url);
            process_map_active = 1;
        }
        else if(action_map == "update_map"){
            process_map_active = 1;
        }
        request_map = 0;
        update_map_database = 1;
    }
}
