// Copyright (c) 2019 Tim Perkins

#include <cstdint>
#include <string>

#include "raw.h"

extern "C" void ros_main() {
  RNContext* context = rn_get_default_context();
  RNNode* node = rn_create_node(context, (const uint8_t*) "node", 4);
  RNPublisher* publisher = rn_create_publisher(node, (const uint8_t*) "topic", 5);

  RNStdMsgString* message = rn_std_msg_string_default();

  for (std::size_t i = 0; i < 10; ++i) {
    std::string data = "Hello C++ ROS! " + std::to_string(i);
    rn_std_msg_string_set_data(message, (const uint8_t*) data.c_str(), data.length());
    rn_publish(publisher, message);
    rn_thread_sleep(1000);
  }
}
