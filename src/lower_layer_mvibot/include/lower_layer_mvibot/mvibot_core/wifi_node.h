#include "mvibot_core_init.h"
using namespace std;
class wifi_node : public rclcpp::Node{      
    private:
        //wifwi
        struct WifiInfo
            {
                std::string ssid;
                int signal;
                std::string active;
                std::string security;
            };
        //timer
        rclcpp::TimerBase::SharedPtr scan_wifi_timer_;
        //pub
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr list_wifi_pub_;
        //
    public:
        wifi_node(const string &node_name, const string &sub_namespace) : Node(node_name, sub_namespace){
            //get namspace
            // mvibot_seri_ = this->get_namespace();
            //pub scan wifi
            list_wifi_pub_ = this->create_publisher<std_msgs::msg::String>("robot_list_wifi",1);
            //timer scan wifi
            auto timer_callback = [this]()->void{
                scan_wifi();
            };
            scan_wifi_timer_ = this->create_wall_timer(1000ms, timer_callback);
        }
        //
        string exec_cmd(const std::string &cmd);
        vector<std::string> splitNmcliLine(const std::string &line);
        void scan_wifi(); 
};
//
string wifi_node::exec_cmd(const std::string &cmd){
    std::array<char, 512> buffer;
    std::string result;
    FILE *pipe = popen(cmd.c_str(), "r");
    if (!pipe){
        RCLCPP_ERROR(this->get_logger(),"popen failed");
        return "";
    }
    while (fgets(buffer.data(), buffer.size(), pipe) != nullptr){
        result += buffer.data();
    }
    pclose(pipe);
    return result;
}
vector<std::string> wifi_node::splitNmcliLine(const std::string &line){
    std::vector<std::string> fields;
    std::string current;
    bool escape = false;
    for (char c : line){
        if (escape)
        {
            current += c;
            escape = false;
        }
        else if (c == '\\')
        {
            escape = true;
        }
        else if (c == ':')
        {
            fields.push_back(current);
            current.clear();
        }
        else
        {
            current += c;
        }
    }
    fields.push_back(current);
    return fields;
}
void wifi_node::scan_wifi(){
    std::string cmd ="nmcli -t -e yes -f SSID,SIGNAL,ACTIVE,SECURITY device wifi list"; // --rescan yes";
    std::string output = exec_cmd(cmd);
    if (output.empty())
    {
        RCLCPP_WARN(this->get_logger(),"wifi scan empty");
        return;
    }
    std::stringstream ss(output);
    std::string line;
    // keep strongest signal for each SSID
    std::map<std::string, WifiInfo> wifi_map;
    while (std::getline(ss, line))
    {
        if (line.empty()) continue;
        auto fields = splitNmcliLine(line);
        if (fields.size() < 4) continue;
        std::string ssid = fields[0];
        // skip hidden SSID
        if (ssid.empty()) continue;
        int signal = 0;
        string active;
        try{
            signal = std::stoi(fields[1]);
            active = fields[2];
        }
        catch (...){
            continue;
        }
        WifiInfo wifi;
        wifi.ssid = ssid;
        wifi.signal = signal;
        wifi.active = fields[2];
        wifi.security = fields[3];
        // keep strongest AP
        if (wifi_map.find(ssid) == wifi_map.end()){
            wifi_map[ssid] = wifi;
        }
        else{
            if(active == "yes") wifi_map[ssid] = wifi;
            else if(signal > wifi_map[ssid].signal && active != "yes"){
                wifi_map[ssid] = wifi;
            }
        }
    }
    // map -> vector
    std::vector<WifiInfo> wifi_list;

    for (auto &item : wifi_map)
    {
        wifi_list.push_back(item.second);
    }
    // sort by signal descending
    std::sort(
        wifi_list.begin(),
        wifi_list.end(),
        [](const WifiInfo &a, const WifiInfo &b)
        {
            return a.signal > b.signal;
        });
    // Create JSON
    json root;
    root["wifi_list"] = json::array();
    for (const auto &wifi : wifi_list)
    {
        json wifi_json;
        wifi_json["ssid"] = wifi.ssid;
        wifi_json["signal"] = wifi.signal;
        wifi_json["active"] = wifi.active;
        wifi_json["security"] = wifi.security;
        root["wifi_list"].push_back(wifi_json);
    }
    // Publish
    std_msgs::msg::String msg;
    msg.data = root.dump();
    list_wifi_pub_->publish(msg);
    RCLCPP_INFO(this->get_logger(),"wifi scan published");
}