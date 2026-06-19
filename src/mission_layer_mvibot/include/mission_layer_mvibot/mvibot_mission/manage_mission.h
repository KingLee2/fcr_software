#include "../common/library_basic.h"
#include "../common/library_ros.h"
#include "../common/string_Iv2.h"
#include "../common/stof.h"
#include "../common/stoi.h"
#include "mission_define.h"
#include <mutex>
using namespace std;
using json = nlohmann::json;
class mission{
    public:
        string mission_id;
        string mission_name;
        std::map<string,json>contents_map;
        std::map<string,json>triggers_map;
        void reset(){
            mission_id="";
            mission_name="";
            contents_map.clear();
            triggers_map.clear();
        }
};
class manage_mission : public rclcpp::Node{
    private:
        //declare tranform var
        std::unique_ptr<tf2_ros::Buffer> tf_Buffer_;
        std::shared_ptr<tf2_ros::TransformListener> tf_Listener_{nullptr};
        std::unique_ptr<tf2_ros::TransformBroadcaster> tf_Broadcaster_;
        //var
        string define_path="/home/mvibot/floorCleaningRobot_ws/";
        string mvibot_seri_,mvibot_seri_f_;
        string ip_robot="";
        mission mission_;
        string action_mode_mission;
        string mission_execution_time, learning_path_time;
        //
        int motor_left_ready = 0;
        int motor_right_ready = 0;
        int brush_status = 0, brush_status_f = 0;
        int status = Finish_;
        string active_mission_id = "";
        int state;
        int step_handle_content = 0;
        int step_try_catch = 0;
        double ts_execute_callback = 0.05;
        json his_content;
        string learning_path_name = "";
        //curl
        CURL* curl;
        CURLcode result;
        std::string readBuffer;
        //pub
        //brush state
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr brush_state_pub_;
        //information mission active
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr active_mission_info_pub_;
        //robot
        rclcpp::Publisher<geometry_msgs::msg::Twist>::SharedPtr stop_robot_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr robot_position_pub_;
        //gpio
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr gpio_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr gpio_function_state_pub_;
        //footprint
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr footprint_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr footprint_function_state_pub_;
        //config runtime
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr config_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr config_function_state_pub_;
        //navigation
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr navigation_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr navigation_function_state_pub_;
        //marker
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr marker_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr marker_function_state_pub_;
        //sleep
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr sleep_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr sleep_function_state_pub_;
        //variable
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr var_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr var_function_state_pub_;
        //lift
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr lift_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr lift_function_state_pub_;
        //brush
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr brush_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr brush_function_state_pub_;
        //suction
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr suction_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr suction_function_state_pub_;
        //loadmap
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr loadmap_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr loadmap_function_state_pub_;
        //initialpose
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr initialpose_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr initialpose_function_state_pub_;
        //history
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr history_pub_;
        //led
        rclcpp::Publisher<std_msgs::msg::Float32MultiArray>::SharedPtr led_pub_;
        //sound
        rclcpp::Publisher<std_msgs::msg::Float32>::SharedPtr sound_pub_;
        //covered pose
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr covered_pose_pub_;
        //learning path
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr learning_path_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr status_learning_path_pub_;
        //sub
        //mission
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr reset_mission_sub_;
        //get brush status
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr brush_status_sub_;
        //get request robot (stop,continues)
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr get_request_robot_sub_;
        //get request learning path
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr learning_path_sub_;
        //get mission executed
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr get_mission_executed_sub_;
        //get status motor
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr motor_left_status_sub_;
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr motor_right_status_sub_;
        //get status battery
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr battery_status_sub_;
        //get status function
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr function_state_sub_;
        //timer
        rclcpp::TimerBase::SharedPtr execute_mission_timer_;
        rclcpp::TimerBase::SharedPtr controll_timer_;
    public:
        manage_mission(const string &node_name, const string &sub_namespace) : Node(node_name, sub_namespace){
            mvibot_seri_ = this->get_namespace();
            mvibot_seri_f_ = mvibot_seri_;
            mvibot_seri_f_.erase(0,1);
            rclcpp::QoS qos_profile(rclcpp::KeepLast(10));
            qos_profile.best_effort();
            //transform
            tf_Buffer_ = std::make_unique<tf2_ros::Buffer>(this->get_clock());
            tf_Listener_ = std::make_shared<tf2_ros::TransformListener>(*tf_Buffer_);
            tf_Broadcaster_ = std::make_unique<tf2_ros::TransformBroadcaster>(*this);
            //
	        action_mode_mission = "N_A";
            active_mission_id = "";
            status = Finish_;
            state = N_A_;
            motor_left_ready = 0;
            motor_right_ready = 0;
            //get ip_robot
            ip_robot = load_file("ip_robot");
            //init curl
            curl_global_init(CURL_GLOBAL_DEFAULT);
            curl = curl_easy_init();
            // init pub//
            //brush state
            brush_state_pub_ = this->create_publisher<std_msgs::msg::String>("brush_state",1);
            // active mission pub
            active_mission_info_pub_ = this->create_publisher<std_msgs::msg::String>("active_mission_info",1);
            //stop robot
            stop_robot_pub_ = this->create_publisher<geometry_msgs::msg::Twist>("cmd_vel",1);
            //robot position
            robot_position_pub_ = this->create_publisher<std_msgs::msg::String>("robot_position",1);
            //send history
            history_pub_ = this->create_publisher<std_msgs::msg::String>("history",1);
            //set led
            led_pub_ = this->create_publisher<std_msgs::msg::Float32MultiArray>("set_led",1);
            //set sound
            sound_pub_ = this->create_publisher<std_msgs::msg::Float32>("music_start",1);
            //gpio
            gpio_info_pub_ = this->create_publisher<std_msgs::msg::String>("gpio_info",1);
            gpio_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("gpio_function_status",1);
            //footprint
            footprint_info_pub_ = this->create_publisher<std_msgs::msg::String>("footprint_info",1);
            footprint_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("footprint_function_status",1);
            //config
            config_info_pub_ = this->create_publisher<std_msgs::msg::String>("config_info",1);
            config_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("config_function_status",1);
            //sleep
            sleep_info_pub_ = this->create_publisher<std_msgs::msg::String>("sleep_info",1);
            sleep_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("sleep_function_status",1);
            //var
            var_info_pub_ = this->create_publisher<std_msgs::msg::String>("variable_info",1);
            var_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("variable_function_status",1);
            //navigation
            navigation_info_pub_ = this->create_publisher<std_msgs::msg::String>("navigation_info",1);
            navigation_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("navigation_function_status",1);
            //marker
            marker_info_pub_ = this->create_publisher<std_msgs::msg::String>("marker_info",1);
            marker_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("marker_function_status",1);
            //lift
            lift_info_pub_ = this->create_publisher<std_msgs::msg::String>("lift_info",1);
            lift_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("lift_function_status",1);
            //brush
            brush_info_pub_ = this->create_publisher<std_msgs::msg::String>("brush_info",1);
            brush_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("brush_function_status",1);
            //suction
            suction_info_pub_ = this->create_publisher<std_msgs::msg::String>("suction_info",1);
            suction_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("suction_function_status",1);
            //loadmap
            loadmap_info_pub_ = this->create_publisher<std_msgs::msg::String>("loadmap_info",1);
            loadmap_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("loadmap_function_status",1);
            //initialpose
            initialpose_info_pub_ = this->create_publisher<std_msgs::msg::String>("initialpose_info",1);
            initialpose_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("initialpose_function_status",1);
            //covered pose
            covered_pose_pub_ = this->create_publisher<std_msgs::msg::String>("covered_pose",1);
            //covered pose
            learning_path_pub_ = this->create_publisher<std_msgs::msg::String>("learning_path_pose",1);
            status_learning_path_pub_ = this->create_publisher<std_msgs::msg::String>("learning_path_status",1);
            ///init subscriber///
            //brush status
            auto brush_callback = [this](std_msgs::msg::String msg)->void{
                char ch_last = msg.data.back();
                if( ch_last=='0') brush_status = 0;
                else if(ch_last == '1') brush_status = 1;
            };
            brush_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/brush_status", qos_profile, brush_callback);
            auto reset_mission_callback = [this](std_msgs::msg::String msg)->void{
                if(status != Active_){
                    mission_.reset();
                    his_content["type"] = action_mode_mission;
                    his_content["state"] = "reset";
                    his_content["description"] = mission_.mission_name;
                    send_history("warning", his_content.dump());
                    reset_function();
                    action_mode_mission = "N_A";
                    step_handle_content = 0;
                    step_try_catch = 0;
                }
            };
            reset_mission_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/reset_mission", qos_profile, reset_mission_callback);
            //status motor
            auto motor_left_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                static int is_ready;
                static int live=-1,live_f=-1,brake=-1,brake_f=-1,enable=-1,enable_f=-1,error=-1,error_f=-1;
                is_ready=1;
                data.detect(msg.data,"","|","");
                for(int i=0;i<data.data1.size();i++){
                    static string_Iv2 data2;
                    data2.detect(data.data1[i],"",":","");
                    if(data2.data1.size()==2){
                        if(data2.data1[0]=="live")      live=stoi_f(data2.data1[1]);
                        if(data2.data1[0]=="enable")    enable=stoi_f(data2.data1[1]);
                        if(data2.data1[0]=="brake")     brake=stoi_f(data2.data1[1]);
                        if(data2.data1[0]=="error")     error=stoi_f(data2.data1[1]);
                    }
                }
                //
                if(live==0) is_ready=0;
                else{
                    if(enable==0) is_ready=0;
                    if(brake==0)  is_ready=0;
                }
                ///test///
                // motor_left_ready=1;
                ///test///
                motor_left_ready=is_ready;// thuc te
                // unlock();
            };
            motor_left_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/motor_left_status", qos_profile, motor_left_status_callback);
            auto motor_right_status_callback = [this](std_msgs::msg::String msg)->void{
                static string_Iv2 data;
                static int live=-1,live_f=-1,brake=-1,brake_f=-1,enable=-1,enable_f=-1,error=-1,error_f=-1;
                static int is_ready;
                is_ready=1;
                data.detect(msg.data,"","|","");
                for(int i=0;i<data.data1.size();i++){
                        static string_Iv2 data2;
                        data2.detect(data.data1[i],"",":","");
                        if(data2.data1.size()==2){
                            if(data2.data1[0]=="live")      live=stoi_f(data2.data1[1]);
                            if(data2.data1[0]=="enable")    enable=stoi_f(data2.data1[1]);
                            if(data2.data1[0]=="brake")     brake=stoi_f(data2.data1[1]);
                            if(data2.data1[0]=="error")     error=stoi_f(data2.data1[1]);
                        }
                    }
                    //
                    if(live==0) is_ready=0;
                    else{
                        if(enable==0) is_ready=0;
                        if(brake==0)  is_ready=0;
                    }
                    ///test///
                    // motor_right_ready=1;
                    ///test///
                    motor_right_ready=is_ready;//thuc te
                    // unlock();
            };
            motor_right_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/motor_right_status", qos_profile, motor_right_status_callback);
            //get request robot (stop,continues)
            auto request_robot_callback = [this](std_msgs::msg::String msg)->void{
                if(status != Finish_){
                    if(msg.data == "stop") status = Stop_;
                    else if(msg.data == "continues") {
                        if(status == Stop_) status = Active_;
                    }
                }
            };
            get_request_robot_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/robot_state", qos_profile,request_robot_callback);
            //get request execute mission
            auto mission_executed_callback = [this](std_msgs::msg::String msg)->void{
                json msg_json;
                msg_json = json::parse(msg.data);
                if(status == Finish_) {
                    active_mission_id = msg_json["active_mission_id"].get<string>();
                    action_mode_mission = msg_json["action_mode_mission"].get<string>();
                }
            };
            get_mission_executed_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/executed_mission", qos_profile, mission_executed_callback);
            //get request "learning path"
            auto learning_path_callback = [this](std_msgs::msg::String msg)->void{
                json data;
                data = json::parse(msg.data);
                if(action_mode_mission == "N_A"){
                    if(data["status"].get<string>() == "start") {
                        action_mode_mission = "learning_path";
                        learning_path_name = data["path_name"].get<string>();
                        learning_path_time = get_time_string();
                        pub_learning_path_status(learning_path_name,"start");
                    }
                }
                else if(action_mode_mission == "learning_path"){
                    if(data["status"].get<string>() == "finish"){
                        action_mode_mission = "N_A";
                        pub_learning_path_status(learning_path_name,"finish");
                        learning_path_name = "";
                    }
                }
            };
            learning_path_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/learning_path", qos_profile, learning_path_callback);
            //get state of function
            auto function_state_callback = [this](std_msgs::msg::String msg)->void{
                if(msg.data == "stop") state = Stop_;
                else if(msg.data == "active") state = Active_;
                else if(msg.data == "finish") state = Finish_;
                else if(msg.data == "cancel") state = Cancel_;
                else if(msg.data == "error") state = Error_;
                else if(msg.data == "true") state = True_;
                else if(msg.data == "false") state = False_;
            };
            function_state_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/function_state", qos_profile, function_state_callback);
            //init timer//
            auto execute_mission_timer_callback = [this]()->void{
                execute_mission();
            };
            execute_mission_timer_ = this->create_wall_timer(50ms, execute_mission_timer_callback);
            auto controll_timer_callback = [this]()->void{
                //set led
                set_led(action_mode_mission);
                //set sound
                set_sound(action_mode_mission);
                //pub covered pose and learning path
                std_msgs::msg::String msg;
                double * robot_pose;
                string data;
                json covered_pose_json, learning_pose_json;
                static rclcpp::Time last_movement_time = this->now();
                static double x_f = 0, y_f = 0, z_f, w_f, x_l, y_l;
                static double dis = 0, angle1 = 0, angle2 = 0, denta_angle = 0, no_move_duration;
                static bool is_stuck = false, brush_pause_stuck = false;
                robot_pose = get_position_tf("map",mvibot_seri_f_+"/base_footprint");
                //pub robot position
                data = mvibot_seri_f_+"|x:"+to_string(robot_pose[0])+"|y:"+to_string(robot_pose[1])+"|thz:"+to_string(robot_pose[2])+"|thw:"+to_string(robot_pose[3]);
                pub_robot_position(data);
                //check and handle error//
                //check stuck
                auto now_time = this->now();
                dis = std::hypot(robot_pose[0]-x_l, robot_pose[1]-y_l);
                RCLCPP_INFO(this->get_logger(),"dis: %f", dis);
                if(dis > 1.0){
                    x_l = robot_pose[0];
                    y_l = robot_pose[1];
                    last_movement_time = now_time;
		            is_stuck = false;
                }
                if(status != Finish_) no_move_duration = (now_time - last_movement_time).seconds();
                else {
                    no_move_duration = 0.0;
                    is_stuck = false;
                }
                RCLCPP_INFO(this->get_logger(),"duration: %f", no_move_duration);
                if(no_move_duration > 15.0) is_stuck = true;
                if(status != Finish_ && brush_status == 1 && is_stuck){
                    //pub brush off
                    pub_state_brush(0);
                    brush_pause_stuck = true;
                    RCLCPP_INFO(this->get_logger(),"turn off brush");
                }
                if(status != Finish_ && brush_pause_stuck && !is_stuck){
                    //pub brush on
                    pub_state_brush(1);
                    brush_pause_stuck = false;
                    RCLCPP_INFO(this->get_logger(),"return on brush");
                }
                //
                //save coverage pose and learning path
                if(action_mode_mission == "mopping_mission"){
                    dis = std::hypot(robot_pose[0]-x_f, robot_pose[1]-y_f);
                    if(dis >= 0.05){
                        x_f = robot_pose[0];
                        y_f = robot_pose[1];
                        covered_pose_json["mission_id"] = mission_.mission_id;
                        covered_pose_json["mission_name"] = mission_.mission_name;
                        covered_pose_json["x"] = to_string(robot_pose[0]);
                        covered_pose_json["y"] = to_string(robot_pose[1]);
                        covered_pose_json["created_at"] = mission_execution_time;
                        msg.data = covered_pose_json.dump();
                        covered_pose_pub_->publish(msg);
                    }
                }
                else if(action_mode_mission == "learning_path"){
                    dis = std::hypot(robot_pose[0]-x_f, robot_pose[1]-y_f);
                    angle1 = getyaw(z_f,w_f);
                    angle2 = getyaw(robot_pose[2],robot_pose[3]);
                    denta_angle = fabs(angle2-angle1);
                    if(dis >= 1.0 || denta_angle >= 0.35){ //1.0m and 0.35rad
                        x_f = robot_pose[00];
                        y_f = robot_pose[1];
                        z_f = robot_pose[2];
                        w_f = robot_pose[3];
                        //
                        learning_pose_json["path_name"] = learning_path_name;
                        learning_pose_json["x"] = to_string(robot_pose[0]);
                        learning_pose_json["y"] = to_string(robot_pose[1]);
                        learning_pose_json["z"] = to_string(robot_pose[2]);
                        learning_pose_json["w"] = to_string(robot_pose[3]);
                        learning_pose_json["created_at"] = learning_path_time;
                        msg.data = learning_pose_json.dump();
                        learning_path_pub_->publish(msg);
                    }
                    pub_learning_path_status(learning_path_name,"active");
                }
            };
            controll_timer_ = this->create_wall_timer(1000ms, controll_timer_callback);
        }
        static size_t WriteCallback(void* contents, size_t size, size_t nmemb, void* userp){
            ((std::string*)userp)->append((char*)contents, size * nmemb);
            return size * nmemb;
        }
        string load_file(string name_file);
        string get_time_string();
        float getyaw(double data1, double data2);
        double *get_position_tf(string name1, string name2);
        void send_history(string status, string info);
        void pub_robot_position(string data);
        void pub_state_brush(int st);
        void pub_led(float red, float green, float blue, float ll, float lr, float lb, float lf);
        void set_led(string mode_action);
        void set_sound(string mode_action);
        void pub_stop_robot();
        void pub_active_mission_info(string type, string mission_id, string content_id, string content_sum);
        void pub_learning_path_status(string path_name, string status);
        void reset_function();
        int handle_content(const json& content, const double& time_out, double & timer, int& status, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr info_pub, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr state_pub);
        void execute_mission();
        int execute_content(mission& mission, vector<string>& queue_content, string& active_content, string& next_to, int& status);                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     
};
string manage_mission::load_file(string name_file){
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
string manage_mission::get_time_string(){
    time_t now_time;
    tm* now_tm;
    std::ostringstream oss;
    auto now = chrono::system_clock::now();
    // Chuyển đổi thành std::time_t
    now_time = chrono::system_clock::to_time_t(now);
    now_tm = gmtime(&now_time);
    oss << put_time(now_tm, "%Y-%m-%d %H:%M:%S");
    return oss.str();
}
float manage_mission::getyaw(double data1, double data2){
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
double *manage_mission::get_position_tf(string source_frame, string target_frame){
    static double data[2];
    //get position
    static double x,y,z,thz,thw;
    static geometry_msgs::msg::TransformStamped transformStamped;
    try{
        transformStamped = tf_Buffer_->lookupTransform(source_frame,target_frame,tf2::TimePointZero);
        x=transformStamped.transform.translation.x;
        y=transformStamped.transform.translation.y;
        z=transformStamped.transform.translation.z;
        thz=transformStamped.transform.rotation.z;
        thw=transformStamped.transform.rotation.w;
    }
    catch (tf2::TransformException &e) {
        x=-1; y=-1; thz=-1; thw=-1;
        RCLCPP_ERROR(this->get_logger(),"Error occured: %s", e.what());
    }
    data[0]=x; data[1]=y; data[2]=thz; data[3]=thw;
    return data;
}
void manage_mission::send_history(string status, string info){
    std_msgs::msg::String history_msg;
    json history_json;
    history_json["name_seri"] = mvibot_seri_f_;
    history_json["status"] = status;
    history_json["content"] = info;
    history_msg.data = history_json.dump();
    history_pub_->publish(history_msg);
}
void manage_mission::pub_robot_position(string data){
    std_msgs::msg::String msg;
    msg.data = data;
    robot_position_pub_->publish(msg);
}
void manage_mission::pub_state_brush(int st){
    std_msgs::msg::String msg;
    if(st == 1) msg.data = "1";
    else if (st == 0) msg.data = "0";
    brush_state_pub_->publish(msg);
}
void manage_mission::pub_led(float red, float green, float blue, float ll, float lr, float lb, float lf){
    static float creat_fun=0;
    static std_msgs::msg::Float32MultiArray msg;
    if(creat_fun==1)
    {
        msg.data.resize(7);
        msg.data[0]=red/100*255;
        msg.data[1]=green/100*255;
        msg.data[2]=blue/100*255;
        msg.data[3]=ll;
        msg.data[4]=lr;
        msg.data[5]=lb;
        msg.data[6]=lf;
        led_pub_->publish(msg);
    } else creat_fun=1;
}
void manage_mission::set_led(string mode_action){
    if(mode_action == "learning_path") pub_led(100,0,100,1,1,1,1);
    else{
        if(status == Active_){
            if(mode_action == "mopping_mission") pub_led(0,100,0,1,1,1,1);
            else if(mode_action == "sub_mission") pub_led(0,100,0,2,2,2,2);
        }
        else if(status == Error_) pub_led(100,0,0,1,1,1,1);
        else if(status == Stop_) pub_led(100,100,0,2,2,2,2);
        else if(status == Finish_) pub_led(100,100,0,1,1,1,1);
        else pub_led(100,100,0,1,1,1,1);
    }
}
void manage_mission::set_sound(string mode_action){
    std_msgs::msg::Float32 msg;
    if(mode_action == "learning_path") {
        msg.data = 4;
        sound_pub_->publish(msg);
    }
    else{
        if(status == Active_){
            msg.data = 3;
            sound_pub_->publish(msg);
        }
        else if(status == Error_) {
            msg.data = 2;
            sound_pub_->publish(msg);
        }
        else {
            msg.data = 0;
            sound_pub_->publish(msg);
        }
    }
}
void manage_mission::pub_stop_robot(){
    static geometry_msgs::msg::Twist stop_robot_;
    stop_robot_.linear.x = 0.0;
    stop_robot_.angular.z = 0.0;
    stop_robot_pub_->publish(stop_robot_);
}
void manage_mission::pub_active_mission_info(string type, string mission_id, string content_id, string content_sum){
    json mission_info;
    std_msgs::msg::String mission_info_str;
    mission_info["type"] = type;
    mission_info["mission_id"] = mission_id;
    mission_info["content_id"] = content_id;
    mission_info["content_sum"] = content_sum; 
    mission_info["status"] = status;
    mission_info_str.data = mission_info.dump();
    active_mission_info_pub_->publish(mission_info_str);
}
void manage_mission::pub_learning_path_status(string path_name, string status){
    json learning_path_json;
    std_msgs::msg::String learning_path_str;
    learning_path_json["path_name"] = path_name; 
    learning_path_json["status"] = status; 
    learning_path_str.data = learning_path_json.dump();
    status_learning_path_pub_->publish(learning_path_str);
}
void manage_mission::reset_function(){
    std_msgs::msg::String state_msg;
    state_msg.data = "finish";
    gpio_function_state_pub_->publish(state_msg);
    sleep_function_state_pub_->publish(state_msg);
    config_function_state_pub_->publish(state_msg);
    var_function_state_pub_->publish(state_msg);
    navigation_function_state_pub_->publish(state_msg);
    marker_function_state_pub_->publish(state_msg);
    brush_function_state_pub_->publish(state_msg);
    suction_function_state_pub_->publish(state_msg);
    lift_function_state_pub_->publish(state_msg);
    status = Finish_;
}
int manage_mission::handle_content(const json& content, const double& time_out, double& timer, int& status, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr info_pub, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr state_pub){
    static std_msgs::msg::String info_msg;
    static std_msgs::msg::String state_msg;
    cout<<"handle content|step:"<<step_handle_content<<"|status:"<<status<<endl;
    if(step_handle_content == 0){
        if(status == Active_){
            // cout<<"INFOMATION CONTENT: "<< content["parameters"]<<endl;
            info_msg.data = content["parameters"].dump();
            info_pub->publish(info_msg);
            state_msg.data = "active";
            state_pub->publish(state_msg);
            timer += ts_execute_callback;
            step_handle_content = 1;
            return Active_;
        }
        else return status;
    }
    else if(step_handle_content == 1){
        RCLCPP_INFO(this->get_logger(),"send state function");
        if(timer>time_out && time_out != -1){
            RCLCPP_INFO(this->get_logger(),"timer lon hon timeout");
            state_msg.data = "error";
            state_pub->publish(state_msg);
            timer = 0.0;
            step_handle_content = 0;
            his_content["type"] = action_mode_mission;
            his_content["state"] = "error";
            his_content["description"] = content["type"].get<string>() + " ," + content["name"].get<string>();
            send_history("error", his_content.dump());
            return Error_;
        }
        else{
            if(state == Finish_){
                RCLCPP_INFO(this->get_logger(),"state finish");
                timer = 0.0;
                step_handle_content = 0;
                state = N_A_;
                return Finish_;
            }
            else if(state == Active_){
                if(status == Stop_){
                    RCLCPP_INFO(this->get_logger(),"state active, status stop");
                    state_msg.data = "stop";
                    state_pub->publish(state_msg);
                    return Stop_;
                }
                else{
                    RCLCPP_INFO(this->get_logger(),"state active, status active");
                    timer += ts_execute_callback;
                    return Active_;
                }
            }
            else if(state == Stop_){
                if(status == Active_){
                    RCLCPP_INFO(this->get_logger(),"state stop, status active");
                    state_msg.data = "active";
                    state_pub->publish(state_msg);
                    return Active_;
                }
                return Stop_;
            }
            else if(state == True_){
                RCLCPP_INFO(this->get_logger(),"state true");
                timer = 0.0;
                step_handle_content = 0;
                state = N_A_;
                return True_;
            }
            else if(state == False_){
                RCLCPP_INFO(this->get_logger(),"state false");
                timer = 0.0;
                step_handle_content = 0;
                state = N_A_;
                return False_;
            }
            else if(state == Error_){
                RCLCPP_INFO(this->get_logger(),"state error");
                timer = 0.0;
                step_handle_content = 0;
                state = N_A_;
                his_content["type"] = action_mode_mission;
                his_content["state"] = "error";
                his_content["description"] = content["type"].get<string>() + " ," + content["name"].get<string>();
                send_history("error", his_content.dump());
                return Error_;
            }
            else {
                RCLCPP_INFO(this->get_logger(),"state N/A");
                timer += ts_execute_callback;
                return Active_;
            }
        }
    }
}
int manage_mission::execute_content(mission& mission, vector<string>& queue_content, string& active_content, string& next_to, int& status){
    json content;
    static double timer = 0.0;
    static string type = "";
    content = mission.contents_map[active_content];
    type = content["type"].get<string>();
    cout<<"Content ID: "<<active_content<<endl;
    cout<<"Type: "<<type<<endl;
    if(type == "start"){
        next_to = content["content"]["next"].get<string>();
        return Finish_;
    }
    else if(type == "group"){
        static std::vector<string> do_content;
        if(next_to == "") {
            next_to = content["content"]["next"].get<string>();
            cout<<"Next to: "<<next_to<<endl;
        }
        if(!queue_content.empty()){
            queue_content.erase(queue_content.begin());
        }
        do_content = content["content"]["body"].get<std::vector<string>>();
        queue_content.insert(queue_content.begin(),do_content.begin(),do_content.end());
        active_content = queue_content[0];
        return Active_;
    }
    else if(type == "if_else"){
        static string cond_id;
        static std::vector<string> then_content, else_content;
        int res;
        cond_id = content["content"]["condition"].get<string>();
        if(cond_id != ""){
            res = execute_content(mission, queue_content, cond_id, next_to, status);
            if(res!=True_ && res!=False_) return Active_;
            else{
                if(next_to == "") {
                    next_to = content["content"]["next"].get<string>();
                    cout<<"Next to: "<<next_to<<endl;
                }
                if(!queue_content.empty()){
                    queue_content.erase(queue_content.begin());
                }
                if(res == True_){
                    then_content = content["content"]["then"].get<std::vector<string>>();
                    queue_content.insert(queue_content.begin(),then_content.begin(),then_content.end());
                }
                else if(res == False_){
                    else_content = content["content"]["else"].get<std::vector<string>>();
                    queue_content.insert(queue_content.begin(),else_content.begin(),else_content.end());
                }
                active_content = queue_content[0];
                return Active_;
            }
        }
        else return Finish_;
    }
    else if(type == "while_do"){
        static string cond_id;
        static std::vector<string> do_content;
        int res;
        cond_id = content["content"]["condition"].get<string>();
        if(cond_id != ""){
            res = execute_content(mission, queue_content, cond_id, next_to, status);
            if(res!=True_ && res!=False_) return Active_;
            else{
                if(next_to == "") {
                    next_to = content["content"]["next"].get<string>();
                    cout<<"Next to: "<<next_to<<endl;
                }
                if(res == True_){
                    do_content = content["content"]["do"].get<std::vector<string>>();
                    queue_content.insert(queue_content.begin(),do_content.begin(),do_content.end());
                    active_content = queue_content[0];
                    return Active_;
                }
                else return Finish_;
            }
        }
        else return Finish_;
    }
    else if(type == "try_catch"){
       static vector<string> try_catch_content;
       static vector<string> try_catch_queue;
       static string active_try_catch;
       if(step_try_catch == 0){
            if(next_to == "") {
                next_to = content["content"]["next"].get<string>();
                cout<<"Next to: "<<next_to<<endl;
            }
            // Tao hang doi thuc thi try-catch
            try_catch_queue.resize(0);
            try_catch_content = content["content"]["try"].get<std::vector<string>>();
            if(try_catch_content.empty()) return Finish_;
            try_catch_queue.resize(try_catch_content.size());
            try_catch_queue = try_catch_content;
            step_try_catch = 1;
            return Active_;
       }
       else if(step_try_catch == 1){
            //thuc thi try
            int res;
            active_try_catch = try_catch_queue[0];
            res = execute_content(mission, queue_content, active_try_catch, next_to, status);
            if(res == Active_) return Active_;
            else if(res == Finish_){
                try_catch_queue.erase(try_catch_queue.begin());
                if(try_catch_queue.empty()){
                    step_try_catch = 0;
                    return Finish_;
                }
                else return Active_;
            }
            else if(res == Error_){
                try_catch_content = content["content"]["catch"].get<std::vector<string>>();
                if(try_catch_content.empty()) return Finish_;
                try_catch_queue.resize(try_catch_content.size());
                try_catch_queue = try_catch_content;
                step_try_catch = 2;
                return Active_;
            }
       }
       else if(step_try_catch == 2){
        //thuc thi catch
            int res;
            active_try_catch = try_catch_queue[0];
            res = execute_content(mission, queue_content, active_try_catch, next_to, status);
            if(res == Active_) return Active_;
            else if(res == Finish_){
                try_catch_queue.erase(try_catch_queue.begin());
                if(try_catch_queue.empty()){
                    step_try_catch = 0;
                    return Finish_;
                }
                else return Active_;
            }
            else if(res == Error_) return Error_;
       }
    }
    else if(type == "and"){
        static vector<string>and_contents_list;
        static int i=0;
        int res;
        and_contents_list = content["content"]["body"].get<std::vector<string>>();
        res = execute_content(mission, queue_content, and_contents_list[i], next_to, status);
        if(res != True_ && res != False_){
            if(res == Error_) return Error_;
            return Active_;
        }
        else{
            if(res == False_){
                i=0;
                return False_;
            }
            else i++;
        }
        if(i == and_contents_list.size()){
            i=0;
            return True_;
        }
        else return Active_;
    }
    else if(type == "or"){
        static vector<string>or_contents_list;
        static int i=0;
        int res;
        or_contents_list = content["content"]["body"].get<std::vector<string>>();
        res = execute_content(mission, queue_content, or_contents_list[i], next_to, status);
        if(res != True_ && res != False_){
            if(res == Error_) return Error_;
            return Active_;
        }
        else{
            if(res == True_){
                i=0;
                return True_;
            }
            else i++;
        }
        if(i == or_contents_list.size()){
            i=0;
            return False_;
        }
        else return Active_;
    }
    else if(type == "gpio"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, gpio_info_pub_, gpio_function_state_pub_);
    }
    else if(type == "footprint"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, footprint_info_pub_, footprint_function_state_pub_);
    }
    else if(type == "config"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>()); 
        return handle_content(content, time_out, timer, status, config_info_pub_, config_function_state_pub_);
    }
    else if(type == "navigation"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, navigation_info_pub_, navigation_function_state_pub_);
    }
    else if(type == "marker"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, marker_info_pub_, marker_function_state_pub_);
    }
    else if(type == "var"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, var_info_pub_, var_function_state_pub_);
    }
    else if(type == "sleep"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, sleep_info_pub_, sleep_function_state_pub_);
    }
    else if(type == "lift"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, lift_info_pub_, lift_function_state_pub_);
    }
    else if(type == "brush"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, brush_info_pub_, brush_function_state_pub_);
    }
    else if(type == "suction"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, suction_info_pub_, suction_function_state_pub_);
    }
    else if(type == "loadmap"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, loadmap_info_pub_, loadmap_function_state_pub_);
    }
    else if(type == "initialpose"){
        static double time_out;
        time_out = stof(content["time_out"].get<string>());
        return handle_content(content, time_out, timer, status, initialpose_info_pub_, initialpose_function_state_pub_);
    }
}
void manage_mission::execute_mission(){
    //var
    RCLCPP_INFO(this->get_logger(),"start execute mission");
    static vector<string> queue_content;
    static string next_to = "";
    static string active_content = "";
    static int active_content_sum;
    static int res;
    json mission_json;
    string api_str="";
    if(motor_left_ready==0 || motor_right_ready==0) {
        if(status == Active_) status = Stop_;
        RCLCPP_INFO(this->get_logger(),"motor is not ready");
    }
    if(status==Finish_){
        RCLCPP_INFO(this->get_logger(),"continue execute mission, status finish");
        active_content = "";
        queue_content.resize(0);
        next_to = "";
        
        if(active_mission_id != ""){
            //reset buffer and curl
            readBuffer.clear();
            curl_easy_reset(curl);
            //get data for api
            RCLCPP_INFO(this->get_logger(),"get data for api");
            if (curl) {
                //get api
                api_str = "http://"+ip_robot+":6430/api/v1/wf/"+active_mission_id;
                RCLCPP_INFO(this->get_logger(),"api: %s", api_str.c_str());
                //get data
                curl_easy_setopt(curl, CURLOPT_URL, api_str.c_str());
                curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
                curl_easy_setopt(curl, CURLOPT_WRITEDATA, &readBuffer);

                result = curl_easy_perform(curl);

                if (result != CURLE_OK) {
                    std::cerr << "curl_easy_perform() failed: " << curl_easy_strerror(result) << std::endl;
                } else {
                    try {
                        RCLCPP_INFO(this->get_logger(),"load data to mission_");
                        mission_json = json::parse(readBuffer);
                        mission_.mission_id = mission_json["mission_id"].get<string>();
                        mission_.mission_name = mission_json["mission_name"].get<string>();
                        if (mission_json.contains("contents") && mission_json["contents"].is_object()) {
                            for (auto& [key, value] : mission_json["contents"].items()) {
                                mission_.contents_map[key] = value;
                            }
                        }
                        RCLCPP_INFO(this->get_logger(),"mission_id: %s",mission_.mission_id.c_str());
                        RCLCPP_INFO(this->get_logger(),"mission_name: %s",mission_.mission_name.c_str());
                    }
                    catch (std::exception& e) {
                        std::cerr << "JSON parse error: " << e.what() << std::endl;
                    }
                }

                // curl_easy_cleanup(curl);
            }
            // curl_global_cleanup();

            //run mission_
            RCLCPP_INFO(this->get_logger(),"check run mission_");
            if(!mission_.contents_map.empty()){
                for (const auto& [key, value] : mission_.contents_map) {
                    if (value.contains("type") && value["type"] == "start") {
                        active_content = key;
                        if(action_mode_mission == "mopping_mission") {
                            mission_execution_time = get_time_string();
                            his_content["type"] = "mopping_mission";
                        }
                        else his_content["type"] = "sub_mission";
                        his_content["state"] = "run";
                        his_content["description"] = mission_.mission_name;
                        send_history("normal", his_content.dump());
                        RCLCPP_INFO(this->get_logger(),"start mission_");
                        break;
                    }
                }
                status = Active_;
                active_content_sum = mission_.contents_map.size();
            }
            active_mission_id = "";
        }
    }
    else {
        if (status == Error_){
            RCLCPP_INFO(this->get_logger(),"continue execute mission, status error");
            //pub stop robot
            pub_stop_robot();
        }
        else {
            //execute mission main
            RCLCPP_INFO(this->get_logger(),"before execute content|status: %d", status);
            res = execute_content(mission_, queue_content, active_content, next_to, status);
            RCLCPP_INFO(this->get_logger(),"result execute content: %d", res);

            if(res == Active_) {
                status = Active_;
            }
            else if(res == Stop_) {
                status = Stop_;
            }
            else if(res == Error_){
                pub_stop_robot();
                status = Error_;
            }
            else if(res == Finish_){
                if(queue_content.empty()){
                    active_content = next_to;
                    next_to = "";
                }
                else{
                    queue_content.erase(queue_content.begin());
                    if(queue_content.empty()){
                        active_content = next_to;
                        next_to = "";
                    }
                    else active_content = queue_content[0];
                }
                if(mission_.contents_map[active_content]["type"].get<string>() == "end"){
                    status = Finish_;
                    mission_execution_time = "";
                    active_content = "";
                    his_content["type"] = action_mode_mission;
                    his_content["state"] = "finish";
                    his_content["description"] = mission_.mission_name;
                    send_history("normal", his_content.dump());
                    action_mode_mission = "N_A";
                    mission_.reset();
                }
                else status = Active_;
            }
            pub_active_mission_info(action_mode_mission, mission_.mission_id, active_content, to_string(active_content_sum));
        }
    }
}
