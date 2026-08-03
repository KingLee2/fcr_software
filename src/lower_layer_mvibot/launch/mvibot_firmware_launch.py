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
    seri_camera1_arg= DeclareLaunchArgument("serial_no_1", default_value=TextSubstitution(text="_336222075697"))
    seri_camera2_arg= DeclareLaunchArgument("serial_no_2", default_value=TextSubstitution(text="_243222076688"))
    mvibot_seri=LaunchConfiguration('mvibot_seri')
    serial_no_1=LaunchConfiguration('serial_no_1')
    serial_no_2=LaunchConfiguration('serial_no_2')
    #path include file
    urdf_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'urdf_launch.py'
    )
    radar_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_radar_launch.py'
    )
    camera_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_single_camera_launch.py'
    )
    pointCloud_to_laserScan_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_pointCloud_To_LaserScan_launch.py'
    )
    laserScan_multi_merger_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_laserscan_multi_merger_launch.py'
    )
    scan_filter_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_scan_filter_launch.py'
    )
    camera_2D_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_camera_2D_launch.py'
    )
    image_rec_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_image_rectify_launch.py'
    )
    apriltag_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_apriltag_launch.py'
    )
    core_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_core_launch.py'
    )
    robot_localization_include=os.path.join(
        get_package_share_directory('lower_layer_mvibot'),
        'launch',
        'mvibot_robot_localization_launch.py'
    )
    #run
    # firmware_group=GroupAction(
    #     actions=[
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(urdf_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(radar_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(camera_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri,
    #                 'serial_no_1': serial_no_1,
    #                 'serial_no_2': serial_no_2
    #             }.items()
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(pointCloud_to_laserScan_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(laserScan_multi_merger_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(scan_filter_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #         # IncludeLaunchDescription(
    #         #     PythonLaunchDescriptionSource(camera_2D_include),
    #         #     launch_arguments={
    #         #         'mvibot_seri': mvibot_seri
    #         #     }.items()
    #         # ),
    #         # IncludeLaunchDescription(
    #         #     PythonLaunchDescriptionSource(image_rec_include),
    #         #     launch_arguments={
    #         #         'mvibot_seri': mvibot_seri
    #         #     }.items()
    #         # ),
    #         # IncludeLaunchDescription(
    #         #     PythonLaunchDescriptionSource(apriltag_include),
    #         #     launch_arguments={
    #         #         'mvibot_seri': mvibot_seri
    #         #     }.items()
    #         # ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(core_include)
    #         ),
    #         IncludeLaunchDescription(
    #             PythonLaunchDescriptionSource(robot_localization_include),
    #             launch_arguments={
    #                 'mvibot_seri': mvibot_seri
    #             }.items()
    #         ),
    #     ]
    # )
    stage1 = GroupAction(
        actions=[
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(urdf_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),

            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(radar_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),

            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(core_include)
            ),
        ]
    )

    stage2 = TimerAction(
        period=2.0,
        actions=[
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(camera_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri,
                    'serial_no_1': serial_no_1,
                    'serial_no_2': serial_no_2
                }.items()
            )
        ]
    )

    stage3 = TimerAction(
        period=6.0,
        actions=[
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(pointCloud_to_laserScan_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),

            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(laserScan_multi_merger_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),

            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(scan_filter_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            ),
        ]
    )

    stage4 = TimerAction(
        period=8.0,
        actions=[
            IncludeLaunchDescription(
                PythonLaunchDescriptionSource(robot_localization_include),
                launch_arguments={
                    'mvibot_seri': mvibot_seri
                }.items()
            )
        ]
    )
    ld=LaunchDescription()
    ld.add_action(mvibot_seri_arg)
    ld.add_action(seri_camera1_arg)
    ld.add_action(seri_camera2_arg)
    ld.add_action(stage1)
    ld.add_action(stage2)
    ld.add_action(stage3)
    ld.add_action(stage4)
    return ld
