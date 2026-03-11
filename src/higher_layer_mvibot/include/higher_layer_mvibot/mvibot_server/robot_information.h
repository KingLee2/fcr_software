#include "mvibot_server_init.h"
using namespace std;
extern void database_update(string table,string coloum,string data,string name_seri);
extern void database_combined(string table,string coloum,string data,string name_seri);
extern int update_battery_chart;
//
string robot_information::get_cmd_update_database(string data1_,string data2_){
    static string string_return;
    //
    static string_Iv2 data;
    static string string_cmd_mysql;
    int robot_id;
    data.detect(data2_,"","|","");
    //
    try{
        free_res();
        res=stmt->executeQuery("SELECT robot_id FROM my_robot WHERE name_seri='"+data.data1[0]+"'");
        if (res->next()) {
            robot_id = res->getInt("robot_id");
        } else {
            cout << "not robot" << endl;
        }
    }catch(sql::SQLException &e){
        cout << "# ERR: " << e.what();
        cout << " (MySQL error code: " << e.getErrorCode();
        cout << ", SQLState: " << e.getSQLState() << " )" << endl;
    }
    try{
        free_res();
        res = stmt->executeQuery("SELECT COUNT(*) AS cnt FROM `"+data1_+"` WHERE name_seri='" + data.data1[0] + "'");
        if(res->next() && res->getInt("cnt") > 0){
            std::cout << data1_<<" had" << std::endl;
        }
        else{
            std::cout << data1_<<" is not" << std::endl;
            string cmd;
            cmd ="";
            cmd = cmd+"INSERT INTO `"+data1_+"` (robot_id) VALUES("+to_string(robot_id)+")";
            send_cmd_to_msyql(cmd);
        }
    }catch(sql::SQLException &e){
        cout << "# ERR: " << e.what();
        cout << " (MySQL error code: " << e.getErrorCode();
        cout << ", SQLState: " << e.getSQLState() << " )" << endl;
    }
    //
    string_cmd_mysql="";
    string_cmd_mysql=string_cmd_mysql+"UPDATE "+data1_+" SET ";
    for(int i=1;i<data.data1.size()-1;i++){
        static string_Iv2 data2;
        data2.detect(data.data1[i],"",":","");
        if(data2.data1[0]!="")
        string_cmd_mysql=string_cmd_mysql+data2.data1[0]+"="+"'"+data2.data1[1]+"', ";
    }
    static string_Iv2 data2;
    data2.detect(data.data1[data.data1.size()-1],"",":","");
    if(data2.data1[0]!="")
    string_cmd_mysql=string_cmd_mysql+data2.data1[0]+"="+"'"+data2.data1[1]+"' ";
    string_cmd_mysql=string_cmd_mysql+"WHERE robot_id = "+to_string(robot_id)+" ";
    string_return=string_cmd_mysql;
    cout<<"update_robot: "<<string_return<<endl;
    return string_return;
}

std::vector<string>  robot_information::cmd_update_database(){
    static vector<string> string_return;
    //
    string_return.resize(0);
    //
    if(sensor_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("sensor_status",sensor_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        sensor_status="";
    }
    //
    if(robot_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("robot_status",robot_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        robot_status="";
    }
    //
    if(battery_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("battery_status",battery_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        battery_status="";
    }
    //
    if(battery_cell_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("battery_cell_status",battery_cell_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        battery_cell_status="";
    }
    //
    if(motor_left_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("motor_left_status",motor_left_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        motor_left_status="";
    }
    //
    if(motor_right_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("motor_right_status",motor_right_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        motor_right_status="";
    }
    //
    if(input_user_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("input_user_status",input_user_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        input_user_status="";
    }
    //
    if(output_user_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("output_user_status",output_user_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        output_user_status="";
    }
    //
    if(robot_config_status!=""){
        string_return.resize(string_return.size()+1);
        string_return[string_return.size()-1]=get_cmd_update_database("robot_config_status",robot_config_status);
        send_cmd_to_msyql(string_return[string_return.size()-1]);
        robot_config_status="";
    }
    return string_return;
}
std::vector<string>  robot_information::cmd_insert_database(){
    string cmd;
    static vector<string> string_return;
    //
    string_return.resize(13);
    // string_return[0]="INSERT INTO my_robot (name_seri,type,robot_id)  VALUES('"+name_seri+"','"+type+"',"+to_string(id)+")";
    cmd="INSERT INTO my_robot (name_seri,type)  VALUES('"+name_seri+"','"+type+"')";
    send_cmd_to_msyql(cmd);
    free_res();
    res=stmt->executeQuery("SELECT robot_id FROM my_robot WHERE name_seri='"+name_seri+"'");
    if (res->next()) {
        int robot_id = res->getInt("robot_id");
        cout << "Robot ID: " << robot_id << endl;
        string_return[0]="INSERT INTO robot_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[1]="INSERT INTO sensor_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[2]="INSERT INTO battery_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[3]="INSERT INTO battery_cell_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[4]="INSERT INTO motor_left_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[5]="INSERT INTO motor_right_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[6]="INSERT INTO input_user_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[7]="INSERT INTO output_user_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[8]="INSERT INTO robot_config_status (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[9]="INSERT INTO history_robot (robot_id)  VALUES("+to_string(robot_id)+")";
        string_return[10]="INSERT INTO battery_status_chart (robot_id)  VALUES("+to_string(robot_id)+")";
    } else {
        cout << "not robot" << endl;
        string_return.resize(0);
    }
    return string_return;
}
void robot_information::update_status_robot(int n){
    static string cmd;
    int robot_id;
    try{
        free_res();
        res=stmt->executeQuery("SELECT robot_id FROM my_robot WHERE name_seri='"+name_seri+"'");
        if (res->next()) {
            robot_id = res->getInt("robot_id");
        } else {
            cout << "not robot" << endl;
        }
    }catch(sql::SQLException &e){
        cout << "# ERR: " << e.what();
        cout << " (MySQL error code: " << e.getErrorCode();
        cout << ", SQLState: " << e.getSQLState() << " )" << endl;
    }
    if(n==1) cmd="update `robot_status` set status='1' where robot_id="+robot_id;
    else cmd="update `robot_status` set status='0' where robot_id="+robot_id;
    try{
        stmt->execute(cmd);
    }catch (sql::SQLException &e) {
        cout << "# ERR: " << e.what();
        cout << " (MySQL error code: " << e.getErrorCode();
        cout << ", SQLState: " << e.getSQLState() << " )" << endl;
    }
}
void robot_information::send_cmd_to_msyql(string cmd){
    try{
        stmt->execute(cmd);
    }catch (sql::SQLException &e) {
        cout << "# ERR: " << e.what();
        cout << " (MySQL error code: " << e.getErrorCode();
        cout << ", SQLState: " << e.getSQLState() << " )" << endl;
    }
}