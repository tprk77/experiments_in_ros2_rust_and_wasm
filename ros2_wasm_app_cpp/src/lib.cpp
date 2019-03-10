// Copyright (c) 2019 Tim Perkins

#include <cstdint>
#include <string>

#include "ros.hpp"

extern "C" void ros_main() {
  ros::Context context = ros::Context::Default();
  ros::Node node = context.CreateNode("node");
  ros::Publisher publisher = node.CreatePublisher("topic");

  ros::StdMsgString message;

  for (std::size_t i = 0; i < 10; ++i) {
    std::string data = "Hello C++ ROS! " + std::to_string(i);
    message.Set(data);
    publisher.Publish(message);
    ros::Sleep(1000);
  }
}
