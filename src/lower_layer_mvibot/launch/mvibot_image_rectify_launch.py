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
    image_rectify_node = Node(
        package='image_proc',
        executable='rectify_node',
        output='screen',
        name='image_rectify_node',
        namespace=LaunchConfiguration("mvibot_seri"),
        remappings=[
			(["/",LaunchConfiguration("mvibot_seri"),"/image"], ["/",LaunchConfiguration("mvibot_seri"),"/image_raw"])
		]
    )
    ld = LaunchDescription()
    ld.add_action(name_seri_arg)
    ld.add_action(image_rectify_node)
    return ld