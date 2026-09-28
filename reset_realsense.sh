#!/bin/bash

set -u

# ============================================================
# Configuration
# ============================================================

USB_DRIVER="/sys/bus/usb/drivers/usb"

CONFIG_DIR="/home/mvibot/floorCleaningRobot_ws/config"

CAMERA1_CONFIG="$CONFIG_DIR/serial_camera1"
CAMERA2_CONFIG="$CONFIG_DIR/serial_camera2"

# Fixed USB topology established during initial setup
CAMERA1_USB_PATH="2-2.4"
CAMERA2_USB_PATH="2-2.1"


# ============================================================
# Read camera serial from config file
# ============================================================

read_camera_serial()
{
    local CONFIG_FILE="$1"

    if [ ! -f "$CONFIG_FILE" ]; then
        echo "WARNING: Config file not found: $CONFIG_FILE" >&2
        echo "UNKNOWN"
        return 0
    fi

    local SERIAL

    SERIAL=$(tr -d '[:space:]' < "$CONFIG_FILE")

    # Remove leading "_" if present
    SERIAL="${SERIAL#_}"

    if [ -z "$SERIAL" ]; then
        echo "UNKNOWN"
    else
        echo "$SERIAL"
    fi
}


# ============================================================
# Reset RealSense camera by fixed USB path
# ============================================================

reset_camera()
{
    local CAMERA_NAME="$1"
    local USB_PATH="$2"
    local SERIAL="$3"

    local DEVICE="/sys/bus/usb/devices/$USB_PATH"

    echo
    echo "========================================"
    echo "Reset RealSense camera"
    echo "Camera  : $CAMERA_NAME"
    echo "Serial  : $SERIAL"
    echo "USB Path: $USB_PATH"
    echo "========================================"

    # --------------------------------------------------------
    # Check USB device directory
    # --------------------------------------------------------

    if [ ! -d "$DEVICE" ]; then

        echo "WARNING: USB device $USB_PATH is not currently enumerated."
        echo "Attempting USB bind/unbind using fixed topology."

    else

        echo "USB device found:"
        echo "$DEVICE"

    fi


    # --------------------------------------------------------
    # Read USB information if available
    # --------------------------------------------------------

    local VID=""
    local PRODUCT=""

    if [ -f "$DEVICE/idVendor" ]; then
        VID=$(cat "$DEVICE/idVendor" 2>/dev/null || true)
    fi

    if [ -f "$DEVICE/product" ]; then
        PRODUCT=$(cat "$DEVICE/product" 2>/dev/null || true)
    fi

    echo "VID     : ${VID:-unknown}"
    echo "Product : ${PRODUCT:-unknown}"


    # --------------------------------------------------------
    # Check whether device is bound to USB driver
    # --------------------------------------------------------

    if [ -e "$USB_DRIVER/$USB_PATH" ]; then

        echo
        echo "Unbind USB device: $USB_PATH"

        if ! echo "$USB_PATH" | sudo tee "$USB_DRIVER/unbind" > /dev/null; then

            echo "ERROR: Failed to unbind USB device: $USB_PATH"
            return 1

        fi

        echo "Unbind successful"

        # Give USB subsystem time to release device
        sleep 2

    else

        echo
        echo "USB device $USB_PATH is not currently bound."

    fi


    # --------------------------------------------------------
    # Bind USB device
    # --------------------------------------------------------

    echo
    echo "Bind USB device: $USB_PATH"

    if ! echo "$USB_PATH" | sudo tee "$USB_DRIVER/bind" > /dev/null; then

        echo "ERROR: Failed to bind USB device: $USB_PATH"
        return 1

    fi

    echo "Bind command successful"


    # --------------------------------------------------------
    # Wait for USB device to return
    # --------------------------------------------------------

    echo
    echo "Waiting for USB device to return..."

    local FOUND=0

    for i in {1..15}; do

        if [ -d "$DEVICE" ]; then

            echo "USB device $USB_PATH returned after ${i}s"

            FOUND=1
            break

        fi

        sleep 1

    done


    --------------------------------------------------------
    Result
    --------------------------------------------------------

    if [ "$FOUND" -eq 0 ]; then

        echo
        echo "ERROR: USB device $USB_PATH did not return."
        echo "Camera: $CAMERA_NAME"
        echo "Serial: $SERIAL"

        return 1

    fi


    echo
    echo "========================================"
    echo "USB reset completed"
    echo "Camera  : $CAMERA_NAME"
    echo "Serial  : $SERIAL"
    echo "USB     : $USB_PATH"
    echo "========================================"

    return 0
}


# ============================================================
# Main
# ============================================================

case "${1:-}" in

    camera1)

        SERIAL=$(read_camera_serial "$CAMERA1_CONFIG")

        echo "Camera1 config : $CAMERA1_CONFIG"
        echo "Camera1 serial : $SERIAL"
        echo "Camera1 USB    : $CAMERA1_USB_PATH"

        reset_camera \
            "camera1" \
            "$CAMERA1_USB_PATH" \
            "$SERIAL"

        ;;


    camera2)

        SERIAL=$(read_camera_serial "$CAMERA2_CONFIG")

        echo "Camera2 config : $CAMERA2_CONFIG"
        echo "Camera2 serial : $SERIAL"
        echo "Camera2 USB    : $CAMERA2_USB_PATH"

        reset_camera \
            "camera2" \
            "$CAMERA2_USB_PATH" \
            "$SERIAL"

        ;;


    *)

        echo "Usage:"
        echo
        echo "  $0 camera1"
        echo "  $0 camera2"
        echo

        exit 1

        ;;

esac
