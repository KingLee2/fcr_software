from launch import LaunchDescription
from launch_ros.actions import Node
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration, TextSubstitution
import os

def generate_launch_description():
    name_seri_arg=DeclareLaunchArgument(
		"mvibot_seri",
		default_value="mvibot"
	)
    camera_2D_param_file_arg=DeclareLaunchArgument(
        'camera_2D_param_file',
        default_value=os.path.join('/home/mvibot/floorCleaningRobot_ws/src/lower_layer_mvibot/param','param_camera_2D.yaml')
    )
    camera_node = Node(
        package='usb_cam',
        executable='usb_cam_node_exe',
        output='screen',
        name='camera',
        namespace=LaunchConfiguration("mvibot_seri"),
        parameters=[LaunchConfiguration('camera_2D_param_file'),
            {"frame_id": [LaunchConfiguration("mvibot_seri"),"/camera"]}
        ]
    )
    ld = LaunchDescription()
    ld.add_action(name_seri_arg)
    ld.add_action(camera_2D_param_file_arg)
    ld.add_action(camera_node)
    return ld