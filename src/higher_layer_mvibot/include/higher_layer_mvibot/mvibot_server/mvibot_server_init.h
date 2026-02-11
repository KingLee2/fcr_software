#include "../common/library_basic.h"
#include "../common/library_ros.h"
#include "../common/string_Iv2.h"
#include "../common/stoi.h"
// #include "../common/send_tranfrom/send_tranfrom.h"

#include "mysql_connection.h"
#include <mysql_driver.h>
#include <cppconn/driver.h>
#include <cppconn/exception.h>
#include <cppconn/resultset.h>
#include <cppconn/statement.h>
using namespace std;
using namespace sql;

#if !defined(mvibot_server_define)
    // mysql var
    sql::mysql::MySQL_Driver *driver;
    Connection *con;
    sql::Statement *stmt;
    sql::ResultSet  *res;
    string user="maruei",password="marueiRnd",host="tcp://127.0.0.1:3360",db="moshi_db";
    void free_res(){
        if(res!=nullptr){
            delete res;
            res=nullptr;
        }
    }
    // class table
    class colume_{
        public:
            string colume_name;
            string colume_type;
            int is_have=0;
    };
    class table_{
        public:
            string table_name;
            vector<colume_> table_colume;
            //
            void add_colume(string name, string type);
            void init_table_PK();
            void init_table_FK(string table_FK, string colume_FK);
    };
    class robot_information{
        public:
            //
            int update_database;
            int id;
            float time_out=0;
            //
            string name_seri;
            string type;
            //
            string sensor_status;
            string robot_status;
            string battery_status;
            string battery_cell_status;
            string input_user_status;
            string output_user_status;
            string motor_left_status;
            string motor_right_status;
            string robot_config_status;
            string history_robot;
            string battery_status_chart;

            std::vector<string> cmd_update_database();
            std::vector<string> cmd_insert_database();
            string get_cmd_update_database(string data1_,string data2_);
            void update_status_robot(int n);
            void send_cmd_to_msyql(string cmd);
    };
    //
    class custom_layerv2{
        public:
            string name_layer;
            string type_layer;
            string name_map;
            float heigth,width;
            float xo,yo,yawo;
            int is_have=0;
            int is_pub=0;
    };
    class fcr_server;
    class fcr_database;
    vector<robot_information> my_robots;
    vector<custom_layerv2> my_layers;
    float ts_my_robots;
    // void layer_process();
    int layer_change=1;
    string name_map_active="";
    string name_map="";
    int request_map=0;
    string action_map="";
    int process_map_active = 0;
    int update_map_database= 0;
    string package_path = "/home/mvibot/floorCleaningRobot_ws/src/higher_layer_mvibot/";
    std::recursive_mutex mutex_common;
    std::recursive_mutex mutex_robot;
    std::recursive_mutex mutext_layer_map;
    #define mvibot_server_define 1
#endif
