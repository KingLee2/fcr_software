import os
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription
from launch.substitutions import LaunchConfiguration
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch_xml.launch_description_sources import XMLLaunchDescriptionSource

from launch_ros.actions import Node


def generate_launch_description():

    # Load mode: navigation / slam
    mode_file = "/home/mvibot/floorCleaningRobot_ws/config/mode"
    mode = "navigation"
    ##
    map_file = "/home/mvibot/floorCleaningRobot_ws/config/map"
    map = "/assets/maps/"
    ##
    mvibot_seri_file = "/home/mvibot/floorCleaningRobot_ws/config/name_seri"
    mvibot_seri =""
    ##
    serial_camera1_file = "/home/mvibot/floorCleaningRobot_ws/config/serial_camera1"
    serial_camera1 = "_109622074579"
    ##
    serial_camera2_file = "/home/mvibot/floorCleaningRobot_ws/config/serial_camera2"
    serial_camera2 = "_317622072985"
    ##
    if os.path.exists(mode_file):
        with open(mode_file, "r") as f:
            mode = f.read().strip()
    ##
    if os.path.exists(map_file):
        with open(map_file, "r") as f:
            map = map + f.read().strip()
    ##
    if os.path.exists(mvibot_seri_file):
        with open(mvibot_seri_file, "r") as f:
            mvibot_seri = f.read().strip()
    ##
    if os.path.exists(serial_camera1_file):
        with open(serial_camera1_file, "r") as f:
            serial_camera1 = f.read().strip()
    ##
    if os.path.exists(serial_camera2_file):
        with open(serial_camera2_file, "r") as f:
            serial_camera2 = f.read().strip()
    # Paths
    higher_pkg_path = os.path.join(
        # os.getenv("HOME"),
        # "mvibot",
        "/home/mvibot/floorCleaningRobot_ws",
        "install",
        "higher_layer_mvibot"
    )
    mission_pkg_path = os.path.join(
        # os.getenv("HOME"),
        # "mvibot",
        "/home/mvibot/floorCleaningRobot_ws",
        "install",
        "mission_layer_mvibot"
    )
    lower_pkg_path = os.path.join(
        # os.getenv("HOME"),
        # "mvibot",
        "/home/mvibot/floorCleaningRobot_ws",
        "install",
        "lower_layer_mvibot"
    )
    # Launch paths
    tf2_launch = os.path.join(higher_pkg_path, "share/higher_layer_mvibot/launch/tf2_web_republisher_launch.py")
    map_saver_launch = os.path.join(higher_pkg_path, "share/higher_layer_mvibot/launch/mvibot_map_saver_launch.py")
    slam_launch = os.path.join(higher_pkg_path, "share/higher_layer_mvibot/launch/mvibot_slam_toolbox_launch.py")
    continues_mapping_launch = os.path.join(higher_pkg_path, "share/higher_layer_mvibot/launch/mvibot_continues_mapping_launch.py")
    lifelong_mapping_launch = os.path.join(higher_pkg_path, "share/higher_layer_mvibot/launch/mvibot_lifelong_launch.py")
    map_server_launch = os.path.join(higher_pkg_path, "share/higher_layer_mvibot/launch/mvibot_map_server_launch.py")
    server_launch = os.path.join(higher_pkg_path, "share/higher_layer_mvibot/launch/mvibot_server_launch.py")
    #
    kernel_launch = os.path.join(lower_pkg_path, "share/lower_layer_mvibot/launch/mvibot_kernel_launch.py")
    firmware_launch = os.path.join(lower_pkg_path, "share/lower_layer_mvibot/launch/mvibot_firmware_launch.py")
    #
    amcl_launch = os.path.join(mission_pkg_path, "share/mission_layer_mvibot/launch/mvibot_amcl_launch.py")
    navigation_launch = os.path.join(mission_pkg_path, "share/mission_layer_mvibot/launch/mvibot_navigation_launch.py")
    mission_launch = os.path.join(mission_pkg_path, "share/mission_layer_mvibot/launch/mvibot_mission_launch.py")

    # rosbridge launch (XML)
    rosbridge_launch = os.path.join(
        "/opt/ros/jazzy/share/rosbridge_server/launch/rosbridge_websocket_launch.xml"
    )

    # Bringup description
    ld = LaunchDescription()

    ld.add_action(
        IncludeLaunchDescription(
            XMLLaunchDescriptionSource(rosbridge_launch)
        )
    )

    ld.add_action(
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(tf2_launch)
        )
    )

    ld.add_action(
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(map_saver_launch),
            launch_arguments={
                'mvibot_seri': mvibot_seri
            }.items()
        )
    )

    ld.add_action(
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(kernel_launch)
        )
    )

    ld.add_action(
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(firmware_launch),
            launch_arguments={
                'mvibot_seri': mvibot_seri,
                'serial_no_1': serial_camera1,
                'serial_no_2': serial_camera2
            }.items()
        )
    )

    if mode == "mapping":
        ld.add_action(
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(slam_launch)
            )
        )
    elif mode == "remapping":
        ld.add_action(
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(continues_mapping_launch),
                launch_arguments={
                    'map_file_name': map
                }.items()
            )
        )
    elif mode == "lifelong":
        ld.add_action(
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(lifelong_mapping_launch),
                launch_arguments={
                    'map_file_name': map
                }.items()
            )
        )
    else:
        ld.add_action(
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(map_server_launch),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            )
        )

        ld.add_action(
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(amcl_launch),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            )
        )

        ld.add_action(
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(navigation_launch),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            )
        )
        ld.add_action(
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(mission_launch)
            )
        )

    ld.add_action(
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(server_launch)
        )
    )

    return ld
