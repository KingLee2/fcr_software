#pragma once
#include "mvibot_core_init.h"
#include"../common/stof.h"
#include"../common/exec.h"
#include"../common/string_Iv2.h"
#include "../common/set_get_param.h"
#include <cmath>
#include <limits>
#include <sstream>

using namespace std;
using json = nlohmann::json;
class camera_calibration_node : public rclcpp::Node{
    private:
        //declare var 
        string mvibot_seri_;
        //declare pub//
        //camera calibration
        rclcpp::Subscription<std_msgs::msg::String>::SharedPtr camera_calibration_sub_;
        rclcpp::Client<rcl_interfaces::srv::SetParameters>::SharedPtr camera1_set_param_client_;
        rclcpp::Client<rcl_interfaces::srv::SetParameters>::SharedPtr camera2_set_param_client_;
        rclcpp::CallbackGroup::SharedPtr camera_calibration_group_;
        rclcpp::Publisher<std_msgs::msg::String>::SharedPtr camera_calibration_status_pub_;
        json camera_calibration_applied_ = json::object();
        // Calibration subscription, clients and startup timer share this exclusive group.
        int camera_calibration_pending_ = 0;
        rclcpp::TimerBase::SharedPtr camera_calibration_startup_timer_;
        std::array<std::shared_ptr<std::vector<rclcpp::Parameter>>, 2> camera_startup_parameters_;
	    //declare timer//
        rclcpp::TimerBase::SharedPtr timer_;
    public:
        camera_calibration_node(const string &node_name, const string &sub_namespace) : Node(node_name, sub_namespace){
            rclcpp::QoS qos_profile(rclcpp::KeepLast(10));
            qos_profile.best_effort();
            mvibot_seri_ = this->get_namespace();
            //init service
            //calib camera
            //init sub //
            //config sub
            camera_calibration_group_ = this->create_callback_group(rclcpp::CallbackGroupType::MutuallyExclusive);
            rclcpp::SubscriptionOptions camera_calibration_options;
            camera_calibration_options.callback_group = camera_calibration_group_;
            camera_calibration_status_pub_ = this->create_publisher<std_msgs::msg::String>("camera_calibration_status", rclcpp::QoS(1).reliable().transient_local());
            camera1_set_param_client_ = this->create_client<rcl_interfaces::srv::SetParameters>(mvibot_seri_ + "/camera1/set_parameters",rmw_qos_profile_services_default, camera_calibration_group_);
            camera2_set_param_client_ = this->create_client<rcl_interfaces::srv::SetParameters>(mvibot_seri_ + "/camera2/set_parameters",rmw_qos_profile_services_default, camera_calibration_group_);
            auto camera_calib_callback = [this](std_msgs::msg::String msg)->void{
                if (camera_calibration_pending_ != 0) {
                    RCLCPP_WARN(this->get_logger(), "Camera calibration busy; retry after current update");
                    return;
                }
                try {
                    const auto calibration = json::parse(msg.data);
                    if (!calibration.is_object() || calibration.size() != 1 ||
                        (!calibration.contains("camera1") && !calibration.contains("camera2")))
                        throw std::runtime_error("JSON must contain exactly one camera: camera1 or camera2");
                    const size_t i = calibration.contains("camera1") ? 0 : 1;
                    const auto camera = "camera" + std::to_string(i + 1);
                    const auto client = i == 0 ? camera1_set_param_client_ : camera2_set_param_client_;
                    auto request = std::make_shared<std::vector<rclcpp::Parameter>>();
                    std::string config;
                    // Validate only the selected camera before sending any changes.
                    {
                        const auto &values = calibration.at(camera);
                        const auto &auto_value = values.at("enable_auto_exposure");
                        bool enable_auto_exposure;  
                        if (auto_value.is_boolean()) enable_auto_exposure = auto_value.get<bool>();
                        else if (auto_value.is_number_integer() && (auto_value == 0 || auto_value == 1))
                            enable_auto_exposure = auto_value.get<int>() == 1;
                        else throw std::runtime_error(camera + ": enable_auto_exposure must be bool or 0/1");
                        auto read_integer = [&values, &camera](const std::string &field) {
                            const auto &value = values.at(field);
                            if (!value.is_number_integer() || value.get<double>() < 0 ||
                                value.get<double>() > std::numeric_limits<int>::max())
                                throw std::runtime_error(camera + ": invalid " + field);
                            return value.get<int>();
                        };
                        const int exposure = read_integer("exposure");
                        const int gain = read_integer("gain");
                        auto &parameters = *request;
                        parameters.emplace_back("depth_module.enable_auto_exposure", false);
                        parameters.emplace_back("depth_module.exposure", exposure);
                        parameters.emplace_back("depth_module.gain", gain);
                        config = "enable_auto_exposure: " + std::to_string(enable_auto_exposure) + "\n" +
                            "exposure: " + std::to_string(exposure) + "\n" +
                            "gain: " + std::to_string(gain) + "\n";
                        // // Accept laser_power when supplied; older UI payloads remain valid.
                        // if (values.contains("laser_power")) {
                        //     const auto &value = values.at("laser_power");
                        //     if (!value.is_number() || !std::isfinite(value.get<double>()) || value.get<double>() < 0)
                        //         throw std::runtime_error(camera + ": invalid laser_power");
                        //     const double laser_power = value.get<double>();
                        //     parameters.emplace_back("depth_module.laser_power", laser_power);
                        //     config += "laser_power: " + std::to_string(laser_power) + "\n";
                        // }
                        parameters.emplace_back("depth_module.enable_auto_exposure", enable_auto_exposure);
                    }
                    // The other camera does not need to be online for this update.
                    if (!client->service_is_ready()) {
                        RCLCPP_WARN(this->get_logger(), "%s parameter service not ready; config not saved", camera.c_str());
                        return;
                    }
                    camera_calibration_pending_ = 1;
                    {
                        const auto path = "/home/mvibot/floorCleaningRobot_ws/config/calib_" + camera;
                        auto complete = [this, i, camera, path, config, request](bool success) {
                            --camera_calibration_pending_;
                            if (!success) return;
                            // A successful UI update supersedes the config queued at startup.
                            camera_startup_parameters_[i].reset();
                            publish_camera_calibration_status(camera, *request);
                            // Save each camera only after all its set_param calls succeed.
                            std::ofstream file(path);
                            if (!file.is_open()) {
                                RCLCPP_ERROR(this->get_logger(), "%s applied, but cannot open %s", camera.c_str(), path.c_str());
                                return;
                            }
                            file << config;
                            file.close();
                            if (file.fail()) {
                                RCLCPP_ERROR(this->get_logger(), "%s applied, but cannot write %s", camera.c_str(), path.c_str());
                                return;
                            }
                            RCLCPP_INFO(this->get_logger(), "%s calibration applied and saved", camera.c_str());
                        };
                        set_camera_calibration_step(client, camera, request, 0, complete);
                    }
                } catch (const std::exception &e) {
                    RCLCPP_ERROR(this->get_logger(), "Invalid camera calibration: %s", e.what());
                }
            };
            camera_calibration_sub_ = this->create_subscription<std_msgs::msg::String>(mvibot_seri_+"/camera_calibration",qos_profile,camera_calib_callback, camera_calibration_options);
            load_camera_calibration_on_startup();
        }
        void publish_camera_calibration_status(const std::string &camera, const std::vector<rclcpp::Parameter> &parameters) {
            json values = json::object();
            for (const auto &parameter : parameters) {
                if (parameter.get_name() == "depth_module.enable_auto_exposure")
                    values["enable_auto_exposure"] = parameter.as_bool() ? 1 : 0;
                else if (parameter.get_name() == "depth_module.exposure")
                    values["exposure"] = parameter.as_int();
                else if (parameter.get_name() == "depth_module.gain")
                    values["gain"] = parameter.as_int();
            }
            camera_calibration_applied_[camera] = values;
            if (!camera_calibration_applied_.contains("camera1") ||
                !camera_calibration_applied_.contains("camera2")) return;
            std_msgs::msg::String message;
            message.data = camera_calibration_applied_.dump();
            camera_calibration_status_pub_->publish(message);
        }

        // Parse the flat key: value files written by the calibration subscription.
        static json read_camera_calibration_file(std::istream &file) {
            json values = json::object();
            std::string line;
            while (std::getline(file, line)) {
                const auto comment = line.find('#');
                if (comment != std::string::npos) line.erase(comment);
                if (line.find_first_not_of(" \t\r") == std::string::npos) continue;
                const auto colon = line.find(':');
                if (colon == std::string::npos) throw std::runtime_error("expected key: value");
                std::istringstream key_stream(line.substr(0, colon));
                std::string key, extra;
                if (!(key_stream >> key) || (key_stream >> extra) ||
                    (key != "enable_auto_exposure" && key != "exposure" && key != "gain"))
                    throw std::runtime_error("unsupported config key");
                if (values.contains(key)) throw std::runtime_error("duplicate key: " + key);
                values[key] = json::parse(line.substr(colon + 1));
            }
            if (file.bad()) throw std::runtime_error("cannot read calibration file");
            // Validate all fields before submitting any camera changes.
            const auto &auto_value = values.at("enable_auto_exposure");
            if (!auto_value.is_boolean() &&
                !(auto_value.is_number_integer() && (auto_value == 0 || auto_value == 1)))
                throw std::runtime_error("enable_auto_exposure must be bool or 0/1");
            for (const auto &field : {"exposure", "gain"}) {
                const auto &value = values.at(field);
                if (!value.is_number_integer() || value.get<double>() < 0 ||
                    value.get<double>() > std::numeric_limits<int>::max())
                    throw std::runtime_error(std::string("invalid ") + field);
            }
            // if (values.contains("laser_power")) {
            //     const auto &value = values.at("laser_power");
            //     if (!value.is_number() || !std::isfinite(value.get<double>()) || value.get<double>() < 0)
            //         throw std::runtime_error("invalid laser_power");
            // }
            return values;
        }

        void load_camera_calibration_on_startup() {
            for (size_t i = 0; i < 2; ++i) {
                const auto camera = "camera" + std::to_string(i + 1);
                const auto path = "/home/mvibot/floorCleaningRobot_ws/config/calib_" + camera;
                std::ifstream file(path);
                if (!file.is_open()) {
                    RCLCPP_WARN(this->get_logger(), "Cannot open %s; keeping camera launch settings", path.c_str());
                    continue;
                }
                try {
                    const auto values = read_camera_calibration_file(file);
                    auto parameters = std::make_shared<std::vector<rclcpp::Parameter>>();
                    const auto &auto_value = values.at("enable_auto_exposure");
                    const bool enable_auto = auto_value.is_boolean() ?
                        auto_value.get<bool>() : auto_value.get<int>() == 1;
                    parameters->emplace_back("depth_module.enable_auto_exposure", false);
                    parameters->emplace_back("depth_module.exposure", values.at("exposure").get<int>());
                    parameters->emplace_back("depth_module.gain", values.at("gain").get<int>());
                    // if (values.contains("laser_power"))
                    //     parameters->emplace_back("depth_module.laser_power", values.at("laser_power").get<double>());
                    parameters->emplace_back("depth_module.enable_auto_exposure", enable_auto);
                    camera_startup_parameters_[i] = parameters;
                } catch (const std::exception &error) {
                    RCLCPP_ERROR(this->get_logger(), "Invalid %s: %s; keeping camera launch settings",path.c_str(), error.what());
                }
            }
            if (!camera_startup_parameters_[0] && !camera_startup_parameters_[1]) return;
            // Retry independently: a missing camera must not hold up the other camera.
            camera_calibration_startup_timer_ = this->create_wall_timer(std::chrono::seconds(2), [this]() {
                    if (camera_calibration_pending_ != 0) return;
                    if (!camera_startup_parameters_[0] && !camera_startup_parameters_[1]) {
                        camera_calibration_startup_timer_->cancel();
                        return;
                    }
                    for (size_t i = 0; i < 2; ++i) {
                        const auto client = i == 0 ? camera1_set_param_client_ : camera2_set_param_client_;
                        if (!camera_startup_parameters_[i] || !client->service_is_ready()) continue;
                        ++camera_calibration_pending_;
                        const auto camera = "camera" + std::to_string(i + 1);
                        const auto parameters = camera_startup_parameters_[i];
                        set_camera_calibration_step(client, camera, parameters, 0,
                            [this, i, camera, parameters](bool success) {
                                --camera_calibration_pending_;
                                if (success) {
                                    camera_startup_parameters_[i].reset();
                                    publish_camera_calibration_status(camera, *parameters);
                                    RCLCPP_INFO(this->get_logger(), "%s startup calibration applied", camera.c_str());
                                } else {
                                    RCLCPP_WARN(this->get_logger(), "%s startup calibration failed; will retry",
                                        camera.c_str());
                                }
                            });
                    }
                }, camera_calibration_group_);
        }

        // Chain async calls: disable auto -> manual settings -> requested auto mode.
        void set_camera_calibration_step(
            const rclcpp::Client<rcl_interfaces::srv::SetParameters>::SharedPtr &client,
            const std::string &camera,
            std::shared_ptr<std::vector<rclcpp::Parameter>> parameters,
            size_t index,
            std::function<void(bool)> complete)
        {
            if (index == parameters->size()) {
                complete(true);
                return;
            }
            const auto parameter = parameters->at(index);
            auto next = [this, client, camera, parameters, index, complete](bool success) {
                if (!success) {
                    RCLCPP_ERROR(this->get_logger(), "%s: failed to set %s; config not saved",
                        camera.c_str(), parameters->at(index).get_name().c_str());
                    complete(false);
                    return;
                }
                set_camera_calibration_step(client, camera, parameters, index + 1, complete);
            };
            switch (parameter.get_type()) {
                case rclcpp::ParameterType::PARAMETER_BOOL:
                    set_param<bool>(client, parameter.get_name(), parameter.as_bool(), next);
                    break;
                case rclcpp::ParameterType::PARAMETER_INTEGER:
                    set_param<int64_t>(client, parameter.get_name(), parameter.as_int(), next);
                    break;
                case rclcpp::ParameterType::PARAMETER_DOUBLE:
                    set_param<double>(client, parameter.get_name(), parameter.as_double(), next);
                    break;
                default:
                    complete(false);
            }
        }
};
