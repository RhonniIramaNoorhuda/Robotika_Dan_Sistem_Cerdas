#include <webots/camera.h>
#include <webots/distance_sensor.h>
#include <webots/motor.h>
#include <webots/robot.h>

#define SPEED 1
#define TIME_STEP 32

int main() {
  WbDeviceTag camera, distance_sensor, left_motor, right_motor;

  wb_robot_init();

  /* Get the camera device, enable it */
  camera = wb_robot_get_device("camera");
  wb_camera_enable(camera, TIME_STEP);

  /* Get the camera device, enable it */
  distance_sensor = wb_robot_get_device("distance sensor");
  wb_distance_sensor_enable(distance_sensor, TIME_STEP);

  /* get a handler to the motors and set target position to infinity (speed control). */
  left_motor = wb_robot_get_device("left wheel motor");
  right_motor = wb_robot_get_device("right wheel motor");
  wb_motor_set_position(left_motor, INFINITY);
  wb_motor_set_position(right_motor, INFINITY);

  /* Set the motors speed */
  wb_motor_set_velocity(left_motor, -SPEED);
  wb_motor_set_velocity(right_motor, SPEED);

  /* Main loop */
  while (wb_robot_step(TIME_STEP) != -1) {
    const double object_distance = wb_distance_sensor_get_value(distance_sensor) / 1000;
    wb_camera_set_focal_distance(camera, object_distance);
  }

  wb_robot_cleanup();

  return 0;
}
