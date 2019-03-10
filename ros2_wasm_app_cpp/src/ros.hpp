// Copyright (c) 2019 Tim Perkins

#ifndef ROS2_WASM_APP_CPP_ROS_HPP
#define ROS2_WASM_APP_CPP_ROS_HPP

#include <cstdint>

#include "raw.h"

namespace ros {

class Context;
class Node;
class Publisher;
class StdMsgString;

class Context {
 public:
  static Context Default();
  Node CreateNode(const std::string& name);

 private:
  Context() = default;
  RNContext* rn_context;
};

class Node {
  friend Context;
 public:
  Publisher CreatePublisher(const std::string& topic);

 private:
  Node() = default;
  RNNode* rn_node;
};

class Publisher {
  friend Node;
 public:
  void Publish(const StdMsgString& message);

 private:
  Publisher() = default;
  RNPublisher* rn_publisher;
};

class StdMsgString {
  friend Publisher;
 public:
  StdMsgString();
  void Set(const std::string& data);

 private:
  RNStdMsgString* rn_message;
};

Context Context::Default() {
  Context context;
  context.rn_context = rn_get_default_context();
  return context;
}

Node Context::CreateNode(const std::string& name) {
  Node node;
  node.rn_node = rn_create_node(rn_context, (const uint8_t*) name.c_str(), name.length());
  return node;
}

Publisher Node::CreatePublisher(const std::string& topic) {
  Publisher publisher;
  publisher.rn_publisher = rn_create_publisher(
      rn_node, (const uint8_t*) topic.c_str(), topic.length());
  return publisher;
}

void Publisher::Publish(const StdMsgString& message) {
  rn_publish(rn_publisher, message.rn_message);
}

StdMsgString::StdMsgString() : rn_message(rn_std_msg_string_default()) {
  // Do nothing
}

void StdMsgString::Set(const std::string& data) {
  rn_std_msg_string_set_data(rn_message, (const uint8_t*) data.c_str(), data.length());
}

void Sleep(std::uint32_t millis) {
  rn_thread_sleep(millis);
}

}  // namespace ros

#endif
