#include "../common/library_basic.h"
#include "../common/library_ros.h"
using namespace std;
#if !defined(mvibot_core_init)
    //socket
    #define PORT 9080
    struct sockaddr_in address, serv_addr;
    int sock=0;
    char add[225]="127.0.0.1";
    int data_socket_error = 0;
    int data_socket_ready = 0;
    float time_out_socket = 0;
    // mode operating for robot
    string mode = "navigation";
    string mvibot_seri="mvibot";
    string define_path = "/home/mvibot/floorCleaningRobot_ws/";
    // param robot 
    float R=0.085,L=0.52;
    float gear=20;
    float v_max=0.5,w_max=0.314;
    float ax=1.0,aw=3.0;
    float robot_shutdown=0;
    float volume=100;
    float low_battery=20;
    // lift brush param
    int lift_brush_control_receive_uart = 0;
    int lift_brush_power_receive_uart = 0;
    float lift_brush_power_send_uart = 0;
    float lift_brush_control_send_uart = 0;
    int lift_brush_mode = 0;
    std_msgs::msg::Float32 lift_brush_data;
    // lift suction param
    int lift_suction_control_receive_uart = 0;
    int lift_suction_power_receive_uart = 0;
    int lift_suction_power_send_uart = 0;
    int lift_suction_control_send_uart = 0;
    std_msgs::msg::Float32 lift_suction_data;
    int lift_suction_mode = 0;
    //temperature
    float temperature_outside =0;
    float temperature_electric_box=0;
    //water level data
    int dirty_water_level = 0;
    int clean_water_level = 0;
    // motor 
    int motor_enable=1;
    int motor_break=1;
    int motor_reset=0;
    int motor_right_disable,motor_left_disable;
    int motor_right_break,motor_left_break;
    int motor_right_state_live,motor_left_state_live;
    int motor_right_state_error,motor_left_state_error;
    float ivr=0,ivl=0,vr_out=0,vl_out=0;
    float kpvrl=0.015,kivrl=1.5,kdvrl=0;
    float vr,vl;
    float tr,tl;
    float vr_set,vl_set;
    float torqueL_set=1.92,torqueR_set=1.92;
    float vrl_max=12.0; //rad/s
    // float ts_pid;
    // sensor robot
    int mvibot_sensor_ready=0;
    // float ts_scan_sensor;
    int radar1_live_status=0;
    int radar2_live_status=0;
    int camera1_live_status=0;
    int camera2_live_status=0;
    int battery_live_status=0;
    int battery_small_live_status=0;
    int uart_live = 0;
    // battery 
    float battery_soc=-1;
    float battery_vol=0;
    float battery_cycle=0;
    float battery_mah_now=0;
    float battery_mah_max=0;
    float battery_guard=0;
    float battery_temperature=0;
    float battery_num_cell=0;
    float battery_current=0;
    float battery_cell[8];
    int battery1_charge;
    int battery_status_charge;
    //gpio
    std_msgs::msg::Float32MultiArray input_user;
    std_msgs::msg::Float32MultiArray output_user;
    // led & color
    float green,red,blue;
    float led_r,led_l,led_f,led_b;
    // odom
    nav_msgs::msg::Odometry odom_wheel;
    float distance_robot=0;
    float distance_wheel_right=0;
    float distance_wheel_left=0;
    // update software
    float software_update=0;
    float software_update_status=0;
    string robot_config_string;
    // rqt data
    std_msgs::msg::Float32MultiArray rqt_data;
    // re_connect
    int master_check_status=0;
    int n_re_connect=0;

    //brush
    int brush_receive_uart_status = 0;
    int brush_send_uart_status = 0;
    //suction
    int suction_receive_uart_status = 0;
    int suction_send_uart_status = 0;
    //valve
    int valve_receive_uart_status = 0;
    int valve_send_uart_status = 0;
    //
        //charge
    int charge_control_status = 0;                     
    int charge_receive_uart_status = 0;
    //mutex var
    std::mutex mutex_socket;
    std::mutex mutex_odom;
    std::mutex mutex_tool;
    std::mutex mutex_control;

    #define mvibot_core_init 1
#endif
