// must have 
#include "../include/lower_layer_mvibot/common/library_basic.h"
#include "../include/lower_layer_mvibot/common/thread_v2.h"
#include "../include/lower_layer_mvibot/common/string_Iv2.h"
//
#include "../include/lower_layer_mvibot/mvibot_kernel/uart.h"
// #include "../include/lower_layer_mvibot/mvibot_kernel/backup.h"
#include "../include/lower_layer_mvibot/mvibot_kernel/keyboard.h"
// #include "../include/lower_layer_mvibot/mvibot_kernel/monitor.h"
// #include "../include/lower_layer_mvibot/mvibot_kernel/security.h"
//
#include "../include/lower_layer_mvibot/mvibot_kernel/mvibot_kernel_init.h"
#include <array>
#include <cerrno>
#include <fcntl.h>
//  
using namespace std;
// socket var 
#define PORT 9080 
int socket_port = -1;
int server_fd;
struct sockaddr_in address; 
int n_client;
float time_out_socket=0.0;
bool socket_timeout_reported = false;
vector<uint8_t> socket_frame_buffer;
//
// create function system
void modify_socket();
void send_data_socket_server();
void check_time_out_socket();
void read_data_socket_server();
void function1();
void function2();
void function3();
void function4();
void function5();
void time_now(string data);
//
void modify_socket(){
    server_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server_fd < 0)
    { 
        perror("socket failed"); 
        exit(EXIT_FAILURE); 
    }  
    int reuse_address = 1;
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &reuse_address, sizeof(reuse_address)) < 0)
    { 
        perror("setsockopt"); 
        close(server_fd);
        exit(EXIT_FAILURE); 
    } 
    address.sin_family = AF_INET; 
    address.sin_addr.s_addr = htonl(INADDR_LOOPBACK); 
    address.sin_port = htons( PORT );  
    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0)
    { 
        perror("bind failed"); 
        close(server_fd);
        exit(EXIT_FAILURE); 
    }
    if (listen(server_fd, 1) < 0)
    {
        perror("listen failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }
}
void disconnect_socket_client(const char *reason){
    if(socket_port >= 0){
        cerr << "Socket client disconnected: " << reason << endl;
        close(socket_port);
        socket_port = -1;
    }
    n_client = 0;
    time_out_socket = 0;
    socket_timeout_reported = false;
    socket_frame_buffer.clear();
    data_receive_socket.clear();
    data_tran_uart = data_tran_uart_defaunt;
}
void send_data_socket_server(){
    if(socket_port < 0 || data_tran_socket.size() != num_byte_stm_pc) return;
    view_data("data_send_socket: ", data_tran_socket);
    const ssize_t sent = send(socket_port, data_tran_socket.data(),
                              data_tran_socket.size(), MSG_NOSIGNAL);
    if(sent < 0 && errno != EAGAIN && errno != EWOULDBLOCK && errno != EINTR){
        const string reason = strerror(errno);
        disconnect_socket_client(reason.c_str());
    }else if(sent >= 0 && static_cast<size_t>(sent) != data_tran_socket.size()){
        disconnect_socket_client("partial frame send");
    }
}
void check_time_out_socket(){
    if(socket_port >= 0){
        time_out_socket += static_cast<float>(ts_socket);
        if(time_out_socket >= 0.250F){
            time_out_socket = 0.250F;
            data_tran_uart = data_tran_uart_defaunt;
            if(!socket_timeout_reported){
                cerr << "Time out socket: no complete valid frame for 250 ms" << endl;
                socket_timeout_reported = true;
            }
        }else if(!data_receive_socket.empty()){
            data_tran_uart = data_receive_socket;
        }
    }else{
        data_tran_uart = data_tran_uart_defaunt;
    }
}
void read_data_socket_server(){
    lock();
    if(socket_port < 0){
        unlock();
        return;
    }

    uint8_t support[999];
    const ssize_t num_byte = read(socket_port, support, sizeof(support));
    if(num_byte == 0){
        disconnect_socket_client("peer closed connection");
        unlock();
        return;
    }
    if(num_byte < 0){
        if(errno != EAGAIN && errno != EWOULDBLOCK && errno != EINTR){
            const string reason = strerror(errno);
            disconnect_socket_client(reason.c_str());
        }
        unlock();
        return;
    }

    socket_frame_buffer.insert(socket_frame_buffer.end(), support, support + num_byte);
    bool received_valid_frame = false;
    const std::array<uint8_t, 3> start_marker{byte_start0, byte_start1, byte_start2};
    while(socket_frame_buffer.size() >= 3){
        auto start = std::search(socket_frame_buffer.begin(), socket_frame_buffer.end(),
                                 start_marker.begin(), start_marker.end());
        if(start == socket_frame_buffer.end()){
            if(socket_frame_buffer.size() > 2){
                socket_frame_buffer.erase(socket_frame_buffer.begin(), socket_frame_buffer.end() - 2);
            }
            break;
        }
        if(start != socket_frame_buffer.begin()){
            socket_frame_buffer.erase(socket_frame_buffer.begin(), start);
        }
        if(socket_frame_buffer.size() < static_cast<size_t>(num_byte_pc_stm)) break;

        const size_t end = static_cast<size_t>(num_byte_pc_stm);
        if(socket_frame_buffer[end - 3] == byte_end0 &&
           socket_frame_buffer[end - 2] == byte_end1 &&
           socket_frame_buffer[end - 1] == byte_end2){
            data_receive_socket.assign(socket_frame_buffer.begin(),
                                       socket_frame_buffer.begin() + end);
            socket_frame_buffer.erase(socket_frame_buffer.begin(),
                                      socket_frame_buffer.begin() + end);
            received_valid_frame = true;
        }else{
            socket_frame_buffer.erase(socket_frame_buffer.begin());
        }
    }

    if(received_valid_frame){
        time_out_socket = 0;
        socket_timeout_reported = false;
        data_tran_uart = data_receive_socket;
        view_data("data_receive_socket: ", data_receive_socket);
    }
    unlock();
}
int main(int argc, char** argv){
    // check match hardware
    //if(check_security()==0){
    //    cout<<"Don 't match hardware!"<<endl;
    //    security_code();
    //}
    ////CAP NHAT SAU
    // get_list_monitor();
    // // history startup
    // system_command="echo ";
    // system_command=system_command+"robot startup at: `date` "+">> "+define_path+"history/startup.log";
    // system(system_command.c_str());
    // // send mail other
    // system_command="scp /home/mvibot/floorCleaningRobot_ws/src/lower_layer_mvibot/history/battery.log /home/mvibot/floorCleaningRobot_ws/src/lower_layer_mvibot/history/battery_send.log";
    // system(system_command.c_str());
    // //
    // system_command="cd /home/mvibot/floorCleaningRobot_ws/src/lower_layer_mvibot/history/ &&";
    // system_command+=" (echo $(date) | mail -s \"Battery history\" mvpkouki@gmail.com -A battery_send.log )";
    // system(system_command.c_str());
    /////
    // modify uart
    serialPort.SetTimeout(5); //set thoi gian cho 5ms
    uart_open();
    // update time scan
    modify_socket();
    ts_socket=0.05;
    ts_uart=0.05;
    //modify data
    data_tran_uart_defaunt.resize(num_byte_pc_stm);
    data_tran_uart_defaunt[0]=byte_start0;
    data_tran_uart_defaunt[1]=byte_start1;
    data_tran_uart_defaunt[2]=byte_start2;
    data_tran_uart_defaunt[data_tran_uart_defaunt.size()-3]=byte_end0;
    data_tran_uart_defaunt[data_tran_uart_defaunt.size()-2]=byte_end1;
    data_tran_uart_defaunt[data_tran_uart_defaunt.size()-1]=byte_end2;
    //
    data_tran_uart_defaunt[robot_color_red]=100;
    data_tran_uart_defaunt[robot_color_green]=100;
    data_tran_uart_defaunt[robot_color_blue]=100;
    data_tran_uart_defaunt[robot_led_left]=1;
    data_tran_uart_defaunt[robot_led_right]=1;
    data_tran_uart_defaunt[robot_led_back]=1;
    //data_tran_uart_defaunt[41]=1;
    //
    data_tran_socket_defaunt.resize(num_byte_stm_pc);
    data_tran_socket_defaunt[0]=byte_start0;
    data_tran_socket_defaunt[1]=byte_start1;
    data_tran_socket_defaunt[2]=byte_start2;
    data_tran_socket_defaunt[data_tran_socket_defaunt.size()-3]=byte_end0;
    data_tran_socket_defaunt[data_tran_socket_defaunt.size()-2]=byte_end1;
    data_tran_socket_defaunt[data_tran_socket_defaunt.size()-1]=byte_end2;
    for(size_t i=0;i<data_tran_socket_defaunt.size();i++){
        static uint8_t data;
        data=data_tran_socket_defaunt[i];
        if(data!=byte_start0 && data != byte_start1 && data!=byte_start2){
            if(data!=byte_end0 && data != byte_end1 && data!=byte_end2){
                data_tran_socket_defaunt[i]=byte_uart_time_out;
            }
        }
    }
    // creat thread
    thread thread1,thread2,thread3,thread4,thread5;
    my_thread my_thread1("Thread1",0.05,function1,false,-1); //listen and accept from socket client 
    my_thread my_thread2("Thread2",0.05,function2,false,-1); //read and write uart
    my_thread my_thread3("Thread3",0.05,function3,false,-1); //read data from socket client
    my_thread my_thread4("Thread4",0.05,function4,false,-1); //write data to socket client
    my_thread my_thread5("Thread5",0.05,function5,false,-1); //backup program robot
    my_thread1.start(thread1);
    my_thread2.start(thread2);
    my_thread3.start(thread3);
    my_thread4.start(thread4);
    my_thread5.start(thread5);
    while (1)
    {
        sleep(1);
    }
    return 0;
}
//
void time_now(string name){
    lock();
        static struct timespec realtime;
        clock_gettime(CLOCK_REALTIME, &realtime);
        cout<<name+"|Time:"<<std::fixed << std::setprecision(5)<<((long double)realtime.tv_sec+(long double)realtime.tv_nsec*1e-9)<<endl;
    unlock();
}
void function1(){
    while(1){
        sockaddr_in client_address{};
        socklen_t client_address_length = sizeof(client_address);
        const int accepted_socket = accept(server_fd,
                                           reinterpret_cast<sockaddr *>(&client_address),
                                           &client_address_length);
        if(accepted_socket < 0){
            if(errno == EINTR) continue;
            perror("accept");
            usleep(100000);
            continue;
        }

        const int socket_flags = fcntl(accepted_socket, F_GETFL, 0);
        if(socket_flags < 0 || fcntl(accepted_socket, F_SETFL, socket_flags | O_NONBLOCK) < 0){
            perror("fcntl accepted socket");
            close(accepted_socket);
            continue;
        }

        char client_ip[INET_ADDRSTRLEN]{};
        inet_ntop(AF_INET, &client_address.sin_addr, client_ip, sizeof(client_ip));
        lock();
        if(socket_port >= 0) disconnect_socket_client("replaced by a new local client");
        socket_port = accepted_socket;
        n_client = 1;
        time_out_socket = 0;
        socket_timeout_reported = false;
        socket_frame_buffer.clear();
        data_receive_socket.clear();
        data_tran_uart = data_tran_uart_defaunt;
        cout << "accept: 1 from " << client_ip << ":" << ntohs(client_address.sin_port) << endl;
        unlock();
    }  

}
void function2(){
        lock();
           cout<<"Function 2 at |"<<get_time()<<endl;
        unlock();   
        // communicate with stm
        lock();
            uart_write();
        unlock();
            usleep(30000);
        // check time out data from stm
        lock();
            uart_read();
            uart_timeout();
        unlock();
}
void function3(){
        lock();
           cout<<"Function 3 at |"<<get_time()<<endl;
        unlock();
        // read data socket
        read_data_socket_server();
}
void function4(){
        lock();
            cout<<"Function 4 at |"<<get_time()<<endl;
            // check time out socket
            check_time_out_socket();
            // send data socket
            send_data_socket_server();
        unlock();
}
void function5(){
    lock();
        cout<<"Function 5 at |"<<get_time()<<endl;
        // backup request
        if(request_backup==1){
            static int finish_backup=0;
            if(finish_backup==0){
                // backup();//CAP NHAT SAU
                finish_backup=1;
            }else{
                time_out_backup-=0.05;
                if(time_out_backup<=0){
                    robot_backup_reboot=1;
                }
            }
        }
        //
        //if(check_monitor_connect()){
        //    cout<<"Some one is connect to output display..."<<endl;
        //    send_mail("mvpkouki@gmail.com",name_seri_fix+" Alarm Security",name_seri_fix+" : someone is plugined display port At `date`","");
        //    security_code();
        //}
    unlock();
}
