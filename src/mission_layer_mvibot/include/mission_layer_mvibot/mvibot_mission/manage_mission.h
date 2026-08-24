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
        int valve_status = 0, valve_status_f = 0;
        int battery_soc = 50, battery_soc_f = 50, battery_filter = 50;
        const float ANPHA_BATTERY_FILTER = 0.1;
        int battery_low_soc = 20;
        int battery_mission_trigger = 0;
        int MAX_BATTERY_SOC = 100;
        int charge_state = 0;
        int status = Finish_;
        string active_mission_id = "";
	    string type = "";
        int state;
        int step_handle_content = 0;
        double ts_execute_callback = 0.05;
        json his_content;
        string learning_path_name = "";
        //curl
        CURL* curl;
        CURLcode result;
        std::string readBuffer;
        //telegram
        string bot_token, chat_id, text;
        //pub
        //valve state
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr valve_state_pub_;
        //brush
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr brush_state_pub_;
        //suction
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr suction_state_pub_;
        //lift
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr lift_brush_power_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr lift_brush_control_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr lift_suction_power_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr lift_suction_control_pub_;
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
	    //charge
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr charge_info_pub_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr charge_function_state_pub_;
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
        //get valve status
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr valve_status_sub_;
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
        //get status charge
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr charge_status_sub_;
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
            //
            bot_token = "8834617424:AAFoiL2dgH8YNcUS5CTWnWPEaeUFoVA9K4M";
            chat_id = "7220296293";
            //get ip_robot
            ip_robot = load_file("ip_robot");
            battery_low_soc = stoi(load_file("robot_low_battery"));
            //init curl
            curl_global_init(CURL_GLOBAL_DEFAULT);
            curl = curl_easy_init();
            // init pub//
            //valve
            valve_state_pub_ = this->create_publisher<std_msgs::msg::String>("valve_state",1);
            //brush state
            brush_state_pub_ = this->create_publisher<std_msgs::msg::String>("brush_state",1);
            // suction state
            suction_state_pub_ = this->create_publisher<std_msgs::msg::String>("suction_state",1);
            // lift brush
            lift_brush_power_pub_ = this->create_publisher<std_msgs::msg::String>("lift_brush_power",1);
            lift_brush_control_pub_ = this->create_publisher<std_msgs::msg::String>("lift_brush_control",1);
            // lift suction
            lift_suction_power_pub_ = this->create_publisher<std_msgs::msg::String>("lift_suction_power",1);
            lift_suction_control_pub_ = this->create_publisher<std_msgs::msg::String>("lift_suction_control",1);
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
	        //charge
            charge_info_pub_ = this->create_publisher<std_msgs::msg::String>("charge_info",1);
            charge_function_state_pub_ = this->create_publisher<std_msgs::msg::String>("charge_function_status",1);
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
            //battery status
            auto battery_status_callback = [this](std_msgs::msg::String msg)->void{
                if(msg.data != "N/A"){
                    static string_Iv2 data;
                    data.detect(msg.data,"","|","");
                    for(int i=1;i<data.data1.size();i++){
                        static string_Iv2 data1;
                        data1.detect(data.data1[i],"",":","");
                        if(data1.data1[0]=="soc"){
                            battery_soc_f = battery_soc;
                            battery_soc = stoi_f(data1.data1[1]);
                            if(abs(battery_soc - battery_soc_f) < 3) battery_filter = round(ANPHA_BATTERY_FILTER*battery_soc + (1- ANPHA_BATTERY_FILTER)*battery_soc_f);
                        }
                    }
                }
                RCLCPP_INFO(this->get_logger(),"battery_soc: %d",battery_filter);
            };
            battery_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/battery_status", qos_profile, battery_status_callback);
            //charge status
            //
            auto charge_callback = [this](std_msgs::msg::String msg)->void{
                char ch_last = msg.data.back();
                if( ch_last=='0') charge_state = 0;
                else if(ch_last == '1') charge_state = 1;
                else if(ch_last == '2') charge_state = 2;
                cout << "charge_state: "<<charge_state<<endl;
            };
            charge_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/charge_status", qos_profile, charge_callback);
            //valve status
            auto valve_callback = [this](std_msgs::msg::String msg)->void{
                char ch_last = msg.data.back();
                if( ch_last=='0') valve_status = 0;
                else if(ch_last == '1') valve_status = 1;
            };
            valve_status_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/valve_status", qos_profile, valve_callback);
            auto reset_mission_callback = [this](std_msgs::msg::String msg)->void{
                if(status != Active_){
                    his_content["type"] = action_mode_mission;
                    his_content["state"] = "reset";
                    his_content["description"] = mission_.mission_name;
                    send_history("warning", his_content.dump());
		            mission_.reset();
                    reset_function();
                    action_mode_mission = "N_A";
                    step_handle_content = 0;
                    battery_mission_trigger = 0;
                    RCLCPP_INFO(this->get_logger(),"battery_mission_trigger: %d", battery_mission_trigger);
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
                    if(charge_state == 0){
                        active_mission_id = msg_json["active_mission_id"].get<string>();
                        action_mode_mission = msg_json["action_mode_mission"].get<string>();
                        battery_mission_trigger = 0;
                        RCLCPP_INFO(this->get_logger(),"battery_mission_trigger: %d", battery_mission_trigger);
                    }
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
                static bool is_stuck = false, valve_pause_stuck = false;
                robot_pose = get_position_tf("map",mvibot_seri_f_+"/base_footprint");
                //pub robot position
                data = mvibot_seri_f_+"|x:"+to_string(robot_pose[0])+"|y:"+to_string(robot_pose[1])+"|thz:"+to_string(robot_pose[2])+"|thw:"+to_string(robot_pose[3]);
                pub_robot_position(data);
                //check and handle error//
                //check stuck
                auto now_time = this->now();
                dis = std::hypot(robot_pose[0]-x_l, robot_pose[1]-y_l);
                RCLCPP_INFO(this->get_logger(),"dis: %f", dis);
                if(dis > 0.5){ //1.0
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
                if(no_move_duration > 5.0) is_stuck = true;
                if(status != Finish_ && valve_status == 1 && is_stuck && type == "navigation"){
                    //pub valve off
                    pub_state_valve(0);
                    valve_pause_stuck = true;
                    RCLCPP_INFO(this->get_logger(),"turn off valve");
                }
                if(status != Finish_ && valve_pause_stuck && !is_stuck){
                    //pub valve on
                    pub_state_valve(1);
                    valve_pause_stuck = false;
                    RCLCPP_INFO(this->get_logger(),"turn on valve");
                }
                if(status != Finish_ && type == "brush"){
                    is_stuck = false;
                    valve_pause_stuck = false;
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
                    if(dis >= 1.0 || denta_angle >= 0.1){ //1.0m and 0.35rad
                        x_f = robot_pose[0];
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
        bool sendTelegramMessage(const string& bot_token, const string& chat_id, const string& text);
        void sendNotification(string robot, string status, int battery, string task, string error);
        float getyaw(double data1, double data2);
        double *get_position_tf(string name1, string name2);
        void send_history(string status, string info);
        void pub_robot_position(string data);
        void pub_state_valve(int st);
        void pub_state_brush(int st);
        void pub_state_suction(int st);
        void pub_state_lift_brush(int st);
        void pub_state_lift_suction(int st);
        void pub_led(float red, float green, float blue, float ll, float lr, float lb, float lf);
        void set_led(string mode_action);
        void set_sound(string mode_action);
        void pub_stop_robot();
        void pub_active_mission_info(string type, string mission_id, string content_id, string content_sum);
        void pub_learning_path_status(string path_name, string status);
        void reset_function();
        int handle_content(const json& content, const double& time_out, double & timer, int& status, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr info_pub, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr state_pub);
        void execute_mission();
        int execute_content(mission& mission, vector<string>& queue_content, string& active_content, int& status);                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     
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
bool manage_mission::sendTelegramMessage(const string& bot_token, const string& chat_id, const string& text){
    //reset buffer and curl
    readBuffer.clear();
    curl_easy_reset(curl);
    //get data for api
    RCLCPP_INFO(this->get_logger(),"send message to telegram");
    if (curl) {
        //get api
        string url;
        url = "https://api.telegram.org/bot" + bot_token + "/sendMessage";
        // Escape nội dung text
        char* escaped_text = curl_easy_escape(curl, text.c_str(), text.length());
        std::string post_fields ="chat_id=" + chat_id + "&text=" + escaped_text;
        curl_free(escaped_text);
        //get data
        curl_easy_setopt(curl, CURLOPT_URL, url.c_str());
        curl_easy_setopt(curl, CURLOPT_POST, 1L);
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, post_fields.c_str());

        result = curl_easy_perform(curl);
        if(result == CURLE_OK) return true;
    }
    return false;
}
void manage_mission::sendNotification(string robot_name, string robot_state, int battery, string task, string error_code){
    stringstream ss;
    ss << "🤖 Robot: " << robot_name << "\n";
    ss << "📍 State: " << robot_state << "\n";
    ss << "🔋 Battery: " << battery << "%\n";
    ss << "📦 Mission: " << task << "\n";
    ss << "⚠️ Error: " << error_code;
    text = ss.str();
    sendTelegramMessage(bot_token, chat_id, text);
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
void manage_mission::pub_state_valve(int st){
    std_msgs::msg::String msg;
    if(st == 1) msg.data = "1";
    else if (st == 0) msg.data = "0";
    valve_state_pub_->publish(msg);
}
void manage_mission::pub_state_brush(int st){
    std_msgs::msg::String msg;
    if(st == 1) msg.data = "1";
    else if (st == 0) msg.data = "0";
    brush_state_pub_->publish(msg);
}
void manage_mission::pub_state_suction(int st){
    std_msgs::msg::String msg;
    if(st == 1) msg.data = "1";
    else if (st == 0) msg.data = "0";
    suction_state_pub_->publish(msg);
}
void manage_mission::pub_state_lift_brush(int st){
    std_msgs::msg::String msg;
    if(st == 1) msg.data = "1";
    else if (st == 0) msg.data = "0";
    lift_brush_power_pub_->publish(msg);
    lift_brush_control_pub_->publish(msg);
}
void manage_mission::pub_state_lift_suction(int st){
    std_msgs::msg::String msg;
    if(st == 1) msg.data = "1";
    else if (st == 0) msg.data = "0";
    lift_suction_power_pub_->publish(msg);
    lift_suction_control_pub_->publish(msg);
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
    charge_function_state_pub_ ->publish(state_msg);
    initialpose_function_state_pub_ ->publish(state_msg);
    loadmap_function_state_pub_ ->publish(state_msg);
    status = Finish_;
}
int manage_mission::handle_content(const json& content, const double& time_out, double& timer, int& status, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr info_pub, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr state_pub){
    static std_msgs::msg::String info_msg;
    static std_msgs::msg::String state_msg;
    cout<<"handle content|step:"<<step_handle_content<<"|status:"<<status<<endl;
    if(step_handle_content == 0){
        if(status == Active_){
            // cout<<"INFOMATION CONTENT: "<< content["parameters"]<<endl;
            state = N_A_;
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
int manage_mission::execute_content(mission& mission, vector<string>& queue_content, string& active_content, int& status){
    json content;
    string next_to = "";
    static double timer = 0.0;
    content = mission.contents_map[active_content];
    type = content["type"].get<string>();
    cout<<"Content ID: "<<active_content<<endl;
    cout<<"Type: "<<type<<endl;
    if(type == "start"){
        next_to = content["content"].contains("next") ? content["content"]["next"].get<string>() : "";
        if (!next_to.empty()) {
            queue_content.insert(queue_content.begin(),next_to);
        }
        return Finish_;
    }
    else if(type == "end"){
        queue_content.clear();
        return Finish_;
    }
    else if(type == "group"){
        std::vector<string> do_content;
        do_content = content["content"]["body"].get<std::vector<string>>();
        next_to = content["content"].contains("next") ? content["content"]["next"].get<string>() : "";
        queue_content.erase(queue_content.begin());
        queue_content.insert(queue_content.begin(),do_content.begin(),do_content.end());
        if(!next_to.empty()) {
            queue_content.insert(queue_content.begin() + do_content.size(), next_to);
        }
        return Finish_;
    }
    else if(type == "if_else"){
        static string cond_id;
        static std::vector<string> then_content, else_content;
        int res;
        cond_id = content["content"]["condition"].get<string>();
        next_to = content["content"].contains("next") ? content["content"]["next"].get<string>() : "";
        if(cond_id != ""){
            res = execute_content(mission, queue_content, cond_id, status);
            if(res!=True_ && res!=False_) return Active_;
            else{
                queue_content.erase(queue_content.begin());
                if(!next_to.empty()) {
                    queue_content.insert(queue_content.begin(), next_to);
                }
                if(res == True_){
                    then_content = content["content"]["then"].get<std::vector<string>>();
                    queue_content.insert(queue_content.begin(),then_content.begin(),then_content.end());
                }
                else if(res == False_){
                    else_content = content["content"]["else"].get<std::vector<string>>();
                    queue_content.insert(queue_content.begin(),else_content.begin(),else_content.end());
                }
                return Finish_;
            }
        }
        else{
            queue_content.erase(queue_content.begin());
            if(!next_to.empty()) {
                queue_content.insert(queue_content.begin(), next_to);
            } 
            return Finish_;
        }
    }
    else if(type == "while_do"){
        static string cond_id;
        static std::vector<string> do_content;
        int res;
        cond_id = content["content"]["condition"].get<string>();
        next_to = content["content"].contains("next") ? content["content"]["next"].get<string>() : "";
        if(cond_id != ""){
            res = execute_content(mission, queue_content, cond_id, status);
            if(res!=True_ && res!=False_) return Active_;
            else{
                if(res == True_){
                    do_content = content["content"]["do"].get<std::vector<string>>();
                    queue_content.insert(queue_content.begin(),do_content.begin(),do_content.end());
                    active_content = queue_content[0];
                    return Active_;
                }
                else{
                    queue_content.erase(queue_content.begin());
                    // next_to = content["content"].contains("next") ? content["content"]["next"].get<string>() : "";
                    if(!next_to.empty()) {
                        queue_content.insert(queue_content.begin(), next_to);
                    }
                    return Finish_;
                }
            }
        }
        else{
            queue_content.erase(queue_content.begin());
            if(!next_to.empty()) {
                queue_content.insert(queue_content.begin(), next_to);
            }
            return Finish_;
        }
    }
    else if(type == "try_catch"){
        vector<string> try_content = content["content"]["try"].get<std::vector<string>>();
        vector<string> catch_content = content["content"]["catch"].get<std::vector<string>>();
        next_to = content["content"].contains("next") ? content["content"]["next"].get<string>() : "";
        // Tạo một Catch Marker Node ảo để lưu giữ khối catch trong map
        string catch_marker_id = "MARKER_CATCH_" + active_content;
        json catch_marker_node;
        catch_marker_node["type"] = "catch_marker";
        catch_marker_node["catch_content"] = catch_content;
        mission.contents_map[catch_marker_id] = catch_marker_node;
        //
        queue_content.erase(queue_content.begin());
        if (!next_to.empty()) {
            queue_content.insert(queue_content.begin(), next_to);
        }
        queue_content.insert(queue_content.begin(), catch_marker_id);
        if (!try_content.empty()) queue_content.insert(queue_content.begin(), try_content.begin(), try_content.end());
        return Finish_;
    }
    else if (type == "catch_marker") {
        queue_content.erase(queue_content.begin());
        return Finish_;
    }
    else if(type == "and"){
        static vector<string>and_contents_list;
        static int i=0;
        int res;
        and_contents_list = content["content"]["body"].get<std::vector<string>>();
        res = execute_content(mission, queue_content, and_contents_list[i], status);
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
        res = execute_content(mission, queue_content, or_contents_list[i], status);
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
    else{
        double time_out = stof(content["time_out"].get<string>());
        int res;
        if(type == "gpio") res = handle_content(content, time_out, timer, status, gpio_info_pub_, gpio_function_state_pub_);
        else if(type == "footprint") res = handle_content(content, time_out, timer, status, footprint_info_pub_, footprint_function_state_pub_);
        else if(type == "config") res = handle_content(content, time_out, timer, status, config_info_pub_, config_function_state_pub_);
        else if(type == "navigation") res = handle_content(content, time_out, timer, status, navigation_info_pub_, navigation_function_state_pub_);
        else if(type == "marker") res = handle_content(content, time_out, timer, status, marker_info_pub_, marker_function_state_pub_);
        else if(type == "var") res = handle_content(content, time_out, timer, status, var_info_pub_, var_function_state_pub_);
        else if(type == "sleep") res = handle_content(content, time_out, timer, status, sleep_info_pub_, sleep_function_state_pub_);
        else if(type == "lift") res = handle_content(content, time_out, timer, status, lift_info_pub_, lift_function_state_pub_);
        else if(type == "brush") res = handle_content(content, time_out, timer, status, brush_info_pub_, brush_function_state_pub_);
        else if(type == "suction") res = handle_content(content, time_out, timer, status, suction_info_pub_, suction_function_state_pub_);
        else if(type == "charge") res = handle_content(content, time_out, timer, status, charge_info_pub_, charge_function_state_pub_);
        else if(type == "loadmap") res = handle_content(content, time_out, timer, status, loadmap_info_pub_, loadmap_function_state_pub_);
        else if(type == "initialpose") res = handle_content(content, time_out, timer, status, initialpose_info_pub_, initialpose_function_state_pub_);
        if(res == Finish_) queue_content.erase(queue_content.begin());
        return res; 
    }
}
void manage_mission::execute_mission(){
    //var
    RCLCPP_INFO(this->get_logger(),"start execute mission");
    static vector<string> queue_content;
    static string active_content = "";
    static int active_content_sum;
    static int res;
    json mission_json;
    string api_str="";
    if(motor_left_ready==0 || motor_right_ready==0) {
        if(status == Active_) status = Stop_;
        RCLCPP_INFO(this->get_logger(),"motor is not ready");
    }
    //check battery
    if(battery_filter<=battery_low_soc){
        if((action_mode_mission == "mopping_mission" || action_mode_mission == "N_A") && battery_mission_trigger == 0 && charge_state == 0){                                                               
            //
            status = Stop_;
            mission_.reset();
            reset_function();
            pub_state_brush(0);
            pub_state_suction(0);
            pub_state_lift_brush(0);
            pub_state_lift_suction(0);
            //send notification
            sendNotification("Moshi", "low battery", battery_filter, "none", "Low battery, I need to go to a charging station.");
            //set trigger goto charging
            action_mode_mission = "battery_charge_mission";
            battery_mission_trigger = 1;
            RCLCPP_INFO(this->get_logger(),"battery_mission_trigger: %d", battery_mission_trigger);
        }
    }
    else {
        if(action_mode_mission == "N_A" && charge_state!=0 && battery_filter >= MAX_BATTERY_SOC){
            //send notification
            sendNotification("Moshi", "full battery", battery_filter, "none", "Full battery, I need to go to the docking");
            //set trigger goto the docking
            action_mode_mission = "battery_charge_mission";
            battery_mission_trigger = 3;
            RCLCPP_INFO(this->get_logger(),"battery_mission_trigger: %d", battery_mission_trigger);
            //
        }
    }
    //run mission
    if(status==Finish_){
        RCLCPP_INFO(this->get_logger(),"continue execute mission, status finish");
        active_content = "";
        queue_content.resize(0);
        if(active_mission_id != "" || battery_mission_trigger == 1 || battery_mission_trigger == 3){
            //reset buffer and curl
            readBuffer.clear();
            curl_easy_reset(curl);
            //get data for api
            RCLCPP_INFO(this->get_logger(),"get data for api");
            if (curl) {
                //get api
                //api_str = "http://"+ip_robot+":6430/api/v1/wf/"+active_mission_id;
		        if(battery_mission_trigger == 1) api_str = "http://127.0.0.1:6430/api/v1/wf/setting/charging";
                else if (battery_mission_trigger == 3) api_str = "http://127.0.0.1:6430/api/v1/wf/setting/parking";
                else api_str = "http://127.0.0.1:6430/api/v1/wf/"+active_mission_id;
                RCLCPP_INFO(this->get_logger(),"api: %s", api_str.c_str());
                //get data
                curl_easy_setopt(curl, CURLOPT_URL, api_str.c_str());
                curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
                curl_easy_setopt(curl, CURLOPT_WRITEDATA, &readBuffer);

                result = curl_easy_perform(curl);

                if (result != CURLE_OK) {
                    std::cerr << "curl_easy_perform() failed: " << curl_easy_strerror(result) << std::endl;
                    battery_mission_trigger = 0;
                    RCLCPP_INFO(this->get_logger(),"GET API FAIL");
                } else {
                    try {
                        RCLCPP_INFO(this->get_logger(),"load data to mission_");
                        mission_json = json::parse(readBuffer);
                        if(mission_json.is_null()){
                            battery_mission_trigger = 0;
                            RCLCPP_INFO(this->get_logger(),"mission json has null");
                        }
                        else if(mission_json.empty()){
                            battery_mission_trigger = 0;
                            RCLCPP_INFO(this->get_logger(),"mission json empty");
                        }
                        else{
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
                    }
                    catch (std::exception& e) {
                        std::cerr << "JSON parse error: " << e.what() << std::endl;
                        battery_mission_trigger = 0;
                    }
                }
            }
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
                        else his_content["type"] = action_mode_mission;
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
            //
            auto marker_it = std::find_if(queue_content.begin(), queue_content.end(), [](const string& id) {
                return id.rfind("MARKER_CATCH_", 0) == 0; // Check prefix MARKER_CATCH_
            });
            if (marker_it != queue_content.end()) {
                string marker_id = *marker_it;
                json catch_node = mission_.contents_map[marker_id];
                vector<string> catch_body = catch_node["catch_content"].get<vector<string>>();
                queue_content.erase(queue_content.begin(), marker_it + 1);
                //
                if (!catch_body.empty()) queue_content.insert(queue_content.begin(), catch_body.begin(), catch_body.end());
                active_content = queue_content[0];
                state = N_A_;
                status = Active_;
            } else {
                RCLCPP_INFO(this->get_logger(),"continue execute mission, status error");
                //pub stop robot
                pub_stop_robot();
                //send notification
                sendNotification("Moshi", "Error", battery_filter, mission_.mission_name, "Robot is error, help me!!!");
                status = Error_;
            }
        }
        else {
            //execute mission main
            RCLCPP_INFO(this->get_logger(),"before execute content|status: %d", status);
            res = execute_content(mission_, queue_content, active_content, status);
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
                    if(action_mode_mission == "battery_charge_mission" && battery_mission_trigger == 1){
                        battery_mission_trigger = 2;
                        RCLCPP_INFO(this->get_logger(),"battery_mission_trigger: %d", battery_mission_trigger);
                    }
                    else if(action_mode_mission == "battery_charge_mission" && battery_mission_trigger == 3) {
                        battery_mission_trigger = 0;
                        RCLCPP_INFO(this->get_logger(),"battery_mission_trigger: %d", battery_mission_trigger);
                    }
                    status = Finish_;
                    mission_execution_time = "";
                    active_content = "";
		            type = "";
                    his_content["type"] = action_mode_mission;
                    his_content["state"] = "finish";
                    his_content["description"] = mission_.mission_name;
                    send_history("normal", his_content.dump());
                    //send notification
                    sendNotification("Moshi", "Finish mission", battery_filter, mission_.mission_name, "None");
                    action_mode_mission = "N_A";
                    mission_.reset();
                }
                else {
                    active_content = queue_content[0];
                    status = Active_;
                }
            }
            else status = Active_;
        }
        pub_active_mission_info(action_mode_mission, mission_.mission_id, active_content, to_string(active_content_sum));
    }
}