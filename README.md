# Floor Cleaning Robot

## Description

`floorCleaningRobot_ws` is a ROS 2 Jazzy workspace for the MVIbot floor cleaning robot. The system integrates hardware control, sensor data acquisition, mapping, localization, navigation, and cleaning mission execution.

The main source code is organized into three layers:

| Package | Function |
| --- | --- |
| `lower_layer_mvibot` | UART communication with the controller, robot control, odometry, LiDAR, RealSense cameras, AprilTag, and the URDF model. |
| `higher_layer_mvibot` | System bringup, SLAM, map management, server functionality, MySQL connectivity, and communication through rosbridge/TF. |
| `mission_layer_mvibot` | Mission management, Nav2 navigation, coverage planning, and brush, suction, lift, and charging operations. |

Supporting packages include LiDAR/RealSense drivers, laser scan processing, OpenNav Coverage, and Fields2Cover.

## quick start

1. **Prepare the environment and install dependencies**

   Use an Ubuntu machine with a ROS 2 Jazzy environment. The project installation script installs ROS 2 Jazzy Desktop, build tools, CppLinuxSerial, MySQL, and related dependencies. Internet access and `sudo` privileges are required.

   ```bash
   cd /home/mvibot/floorCleaningRobot_ws
   chmod +x install_fcr.sh
   ./install_fcr.sh
   ```

2. **Build the workspace**

   ```bash
   cd /home/mvibot/floorCleaningRobot_ws
   source /opt/ros/jazzy/setup.bash
   colcon build --symlink-install --cmake-args -DCMAKE_BUILD_TYPE=Release
   source install/setup.bash
   ```

3. **Create the map storage directory**

   ```bash
   sudo mkdir -p /assets/maps/
   ```

4. **Configure camera, LiDAR, and UART connections**

   **Configure the USB ports for UART and LiDAR**

   Create a file named `99-local.rules` with the following contents:

   ```udev
   SUBSYSTEM=="tty", ENV{ID_PATH}=="pci-0000:00:14.0-usb-0:1.1:1.0", SYMLINK+="rplidar2", MODE="0666"
   SUBSYSTEM=="tty", ENV{ID_PATH}=="pci-0000:00:14.0-usb-0:1.4:1.0", SYMLINK+="rplidar1", MODE="0666"
   SUBSYSTEM=="tty", ENV{ID_PATH}=="pci-0000:00:14.0-usb-0:4.2:1.0", SYMLINK+="stm", MODE="0666"
   ```

   Replace each example `ID_PATH` with the value for the corresponding USB port on your machine. For example, inspect `/dev/ttyUSB0` with:

   ```bash
   udevadm info /dev/ttyUSB0 | grep "ID_PATH"
   ```

   From the directory containing `99-local.rules`, copy it to `/etc/udev/rules.d/` and reload the rules:

   ```bash
   sudo cp 99-local.rules /etc/udev/rules.d/
   sudo udevadm control --reload-rules
   ```

   Reconnect the devices to apply the rules.

   **Configure the D435 and D435i cameras**

   Open RealSense Viewer from a terminal:

   ```bash
   realsense-viewer
   ```

   Load and apply the camera connection configuration file in RealSense Viewer.

   Open the workspace configuration directory:

   ```bash
   cd ~/floorCleaningRobot_ws/config/
   ```

   Enter the serial numbers of the two cameras in `serial_camera1` and `serial_camera2`, respectively. Keep the `_` prefix expected by the launch configuration.

5. **Install and enable the startup services**

   Copy `config_internet.service` and `robot_startup.service` to `/etc/systemd/system/`:

   ```bash
   cd ~/floorCleaningRobot_ws/robot_startup
   sudo cp config_internet.service robot_startup.service /etc/systemd/system/
   sudo systemctl daemon-reload
   ```

   Enable both services to run at boot:

   ```bash
   sudo systemctl enable config_internet.service
   sudo systemctl enable robot_startup.service
   ```

6. **Check the robot configuration**

   Update the files in `config/` to match the actual robot before launching:

   | File | Contents |
   | --- | --- |
   | `name_seri` | Robot name/serial identifier. |
   | `mode` | Operating mode: `mapping`, `remapping` or `navigation`. |
   | `map` | Name of the current map; the relevant launch files load maps from `/assets/maps/`. |
   | `serial_camera1`, `serial_camera2` | RealSense camera serial numbers, using the `_` prefix expected by the current launch configuration. |
   | `robot_*` | Mechanical parameters, motion limits, battery settings, and connectivity settings. |
   | `lan_*`, `wifi_*`, `ip_*` | Robot network settings. |

   Connect the hardware and check serial port access permissions. The LiDAR launch currently uses `/dev/rplidar1` and `/dev/rplidar2`; configure the corresponding devices/udev rules. For navigation mode, prepare the map and the MySQL configuration/schema used by the server. The installation script only installs the MySQL software.


## folder structure

```text
floorCleaningRobot_ws/
├── README.md
├── install_fcr.sh                 # Install ROS 2 and dependencies
├── config/                       # Robot, network, mode, and camera serial settings
├── robot_startup/                # .rc scripts and systemd services for robot/network startup
├── src/
│   ├── lower_layer_mvibot/
│   │   ├── src/                  # mvibot_kernel.cpp, mvibot_core.cpp
│   │   ├── include/              # UART, control, odometry, and utilities
│   │   ├── launch/               # Hardware and sensor launch files
│   │   ├── param/                # Sensor, filter, and localization parameters
│   │   ├── urdf/                 # Robot model and meshes
│   │   └── mp3/                  # Notification sounds
│   ├── higher_layer_mvibot/
│   │   ├── src/                  # mvibot_server.cpp
│   │   ├── include/              # Server, robot information, and MySQL
│   │   ├── launch/               # Bringup, SLAM, maps, and services
│   │   └── param/                # SLAM Toolbox parameters
│   ├── mission_layer_mvibot/
│   │   ├── src/                  # mvibot_mission.cpp
│   │   ├── include/              # Mission functions and management
│   │   ├── launch/               # Mission, Nav2, and coverage launch files
│   │   └── param/                # Nav2 parameters and Behavior Tree XML files
│   ├── sllidar_ros2/             # SLAMTEC LiDAR driver
│   ├── realsense-ros/            # RealSense camera drivers and messages
│   ├── ira_laser_tools/          # Laser scan merging/processing tools
│   ├── pointcloud_to_laserscan/  # Point cloud to laser scan conversion
│   ├── tf2_web_republisher/      # TF data for web clients
│   ├── opennav_coverage/         # Coverage planning and execution
│   └── Fields2Cover/            # Coverage path planning library
├── build/                        # Intermediate files generated by colcon
├── install/                      # Built packages and environment setup scripts
└── log/                          # Colcon build logs
```
