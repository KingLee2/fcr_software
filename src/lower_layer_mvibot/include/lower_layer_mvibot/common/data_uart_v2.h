#include "library_basic.h"
#if !defined(data_uart_define)
        // define byte send pc to stm
        #define num_byte_pc_stm                    120
        #define num_byte_stm_pc                    
        #define byte_start0                        0xff
        #define byte_start1                        0xee
        #define byte_start2                        
        #define byte_end0                          0xdd
        #define byte_end1                          0xee
        #define byte_end2                          0xff

        uint8_t byte_uart_time_out=0xcc;

        // frame byte pc to stm
        // motor1
        #define motor1_enable                      3
        #define motor1_stop                        4
        #define motor1_cw_cww                      5
        #define motor1_set_speedH                  6
        #define motor1_set_speedL                  7
        #define motor1_set_torqueH                 8
        #define motor1_set_torqueL                 9
        #define motor1_reset_error                 10
        #define motor1_break                       11
        // motor2              
        #define motor2_enable                      16
        #define motor2_stop                        17
        #define motor2_cw_cww                      18
        #define motor2_set_speedH                  19
        #define motor2_set_speedL                  20
        #define motor2_set_torqueH                 21
        #define motor2_set_torqueL                 22
        #define motor2_reset_error                 23
        #define motor2_break                       24
        // IO system               
        #define robot_shutdow                      30
        #define robot_led_left                     31
        #define robot_led_right                    32
        #define robot_led_front                    33
        #define robot_color_red                    34
        #define robot_color_green                  35
        #define robot_color_blue                   36
        // IO user             
        #define num_out_put_user                   12
        #define robot_io_user_out0                 40
        #define robot_io_user_out1                 41
        #define robot_io_user_out2                 42
        #define robot_io_user_out3                 43
        #define charge_control_pc_to_stm           50
        #define suction_state_pc_to_stm            51
        #define brush_state_pc_to_stm              52
        #define valve_state_pc_to_stm              
        #define cylinder_brush_power_pc_to_stm     54
        #define cylinder_brush_control_pc_to_stm   55
        #define cylinder_suction_control_pc_to_stm 56

        // stm -> pc
        #define motor1_enable_re	               3
        #define motor1_brake_re		               4
        #define motor1_status_re	               5
        #define motor1_speed_H_re	               6
        #define motor1_speed_L_re	               7
        #define motor1_torque_H_re	               8
        #define motor1_torque_L_re	               9
        #define motor1_ccw_re		               10
        #define motor1_error_H_re	               11
        #define motor1_error_L_re	               12
            
        #define motor2_enable_re	               16
        #define motor2_brake_re		               17
        #define motor2_status_re	               18
        #define motor2_speed_H_re	               19
        #define motor2_speed_L_re	               20
        #define motor2_torque_H_re	               21
        #define motor2_torque_L_re	               22
        #define motor2_ccw_re		               23
        #define motor2_error_H_re	               24
        #define motor2_error_L_re	               25
        // system
        #define button1_re                         30
        //water level
        #define dirty_water_level_re               31
        #define clean_water_level_re               32
        //             
        #define other_action                       34
        //charge
        #define charge_state_re                    35
        //brush
        #define brush_state_re                     36
        //brush
        #define suction_state_re                   37
        //brush
        #define valve_state_re                     38
        //cylinder
        #define cylinder_brush_power_re            39
        #define cylinder_brush_control_re          40
        #define cylinder_suction_control_re        41
        #define cylinder_brush_high_value_re       42
        #define cylinder_brush_low_value_re        43
        #define cylinder_suction_high_value_re     44
        #define cylinder_suction_low_value_re      45
        #define FCR_temperature				       46
        #define Ebox_temperature			       47
        #define battery_temperature_re		       48

        // LifePo4 battery
        #define battery_vol_H_re		           50
        #define battery_vol_L_re		           51
        #define battery_current_H_re	           52
        #define battery_current_L_re	           53
        #define battery_soc_re			           54
        #define battery_vol_diff_H_re		       55
        #define battery_vol_diff_L_re		       56
        #define battery_Mos_temp_re			       57
        #define battery_Bat_temp_re			       58
        #define battery_warning_re		 	       59
        #define battery_average_cell_H_re	       60
        #define battery_average_cell_L_re	       61
        #define battery_balance_re			       62
        #define battery_charging_re			       63
        #define battery_discharge_re		       64
    
        #define battery_num_cell_re			       65
        #define battery_vol_cell1_re		       66
        #define battery_vol_cell2_re		       67
        #define battery_vol_cell3_re		       68
        #define battery_vol_cell4_re		       69
        #define battery_vol_cell5_re		       70
        #define battery_vol_cell6_re		       71
        #define battery_vol_cell7_re		       72
        #define battery_vol_cell8_re		       73
        //Lead-acid battery

        // io
        #define num_in_put_user                    12
        #define io1_re                             74
        #define io2_re                             75
        #define io3_re                             76
        #define io4_re                             77
        #define io5_re                             78
        #define io6_re                             79
        #define io7_re                             80
        #define io8_re                             81
        #define io9_re                             82
        #define io10_re                            83
        #define io11_re                            84
        #define io12_re                            85
        //     
        #define version_software                   113
        //
    #define data_uart_define 1
#endif
