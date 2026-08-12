from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.actions import GroupAction, TimerAction
from launch.actions import IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration, TextSubstitution
from launch_ros.actions import PushROSNamespace
from ament_index_python import get_package_share_directory
import os

def generate_launch_description():
    #declare arg
    mvibot_seri_arg=DeclareLaunchArgument(
        'mvibot_seri',
        default_value='mvibot'
    )
    mvibot_seri=LaunchConfiguration('mvibot_seri')
    #path include file
    planner_server_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_planner_launch.py'
    )
    coverage_server_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_opennav_coverage_launch.py'
    )
    controller_server_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_controller_launch.py'
    )
    behavior_server_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_behavior_launch.py'
    )
    smoother_server_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_smoother_launch.py'
    )
    lifecycle_manager_navigation_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_lifecycle_manager_navigation_launch.py'
    )
    waypoint_follower_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_waypoint_follower_launch.py'
    )
    velocity_smoother_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_velocity_smoother_launch.py'
    )
    collision_monitor_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_collision_monitor_launch.py'
    )
    bt_navigator_include=os.path.join(
        get_package_share_directory('mission_layer_mvibot'),
        'launch',
        'mvibot_bt_navigator_launch.py'
    )
    #run
    navigation_group=GroupAction(
        actions=[
            # PushROSNamespace(namespace=mvibot_namespace),
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(planner_server_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(coverage_server_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(smoother_server_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(controller_server_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(behavior_server_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),
            IncludeLaunchDescription(
                    PythonLaunchDescriptionSource(bt_navigator_include),
                    launch_arguments={
                        'mvibot_seri': mvibot_seri
                    }.items()
                ),
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(lifecycle_manager_navigation_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),
            #IncludeLaunchDescription(
            #    PythonLaunchDescriptionSource(waypoint_follower_include),
            #    launch_arguments={
            #        'mvibot_seri': mvibot_seri
            #    }.items()
            #),
            #IncludeLaunchDescription(
            #    PythonLaunchDescriptionSource(velocity_smoother_include),
            #    launch_arguments={
            #        'mvibot_seri': mvibot_seri
            #    }.items()
            #),
            #IncludeLaunchDescription(
            #    PythonLaunchDescriptionSource(collision_monitor_include),
            #    launch_arguments={
            #        'mvibot_seri': mvibot_seri
            #    }.items()
            #),
        ]
    )

    # stage1 = TimerAction(
    #     period=2.0,
    #     actions=[
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(planner_server_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(coverage_server_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         )
    #     ]
    # )

    # stage2 = TimerAction(
    #     period=4.0,
    #     actions=[
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(controller_server_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(smoother_server_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         )
    #     ]
    # )

    # stage3 = TimerAction(
    #     period=6.0,
    #     actions=[
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(behavior_server_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(bt_navigator_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         )
    #     ]
    # )

    # stage4 = TimerAction(
    #     period=8.0,
    #     actions=[
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(lifecycle_manager_navigation_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         )
    #     ]
    # )

    ld=LaunchDescription()
    ld.add_action(mvibot_seri_arg)
    ld.add_action(navigation_group)
    # ld.add_action(stage1)
    # ld.add_action(stage2)
    # ld.add_action(stage3)
    # ld.add_action(stage4)
    return ld
