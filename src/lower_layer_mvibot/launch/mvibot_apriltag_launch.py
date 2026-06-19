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
    apriltag_param_file_arg=DeclareLaunchArgument(
        'apriltag_param_file',
        default_value=os.path.join('/home/mvibot/floorCleaningRobot_ws/src/lower_layer_mvibot/param','tags_36h11.yaml')
    )
    apriltag_node = Node(
        package='apriltag_ros',
        executable='apriltag_node',
        output='screen',
        name='apriltag_node',
        namespace=LaunchConfiguration("mvibot_seri"),
        parameters=[LaunchConfiguration('apriltag_param_file')]
    )
    ld = LaunchDescription()
    ld.add_action(name_seri_arg)
    ld.add_action(apriltag_param_file_arg)
    ld.add_action(apriltag_node)
    return ld