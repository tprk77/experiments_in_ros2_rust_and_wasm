// Copyright (c) 2019 Tim Perkins

#ifndef ROS2_WASM_APP_CPP_RAW_H
#define ROS2_WASM_APP_CPP_RAW_H

#include <stdint.h>

/*
 * This module contains the "raw" interface to the functions provided by the WebAssembly
 * environment. These will become "import" functions when we generate the WASM. For now, we are only
 * providing a very minimal ROS interface. Basically just enough to run the publisher and subscriber
 * node demos you usually see in tutorials.
 */

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RNContext RNContext;
typedef struct RNNode RNNode;
typedef struct RNPublisher RNPublisher;

/*
 * This is a major limitation of this interface. We are going to limit the available messages to
 * just the `std_msgs::String` message. If this were a real library, we would need to generate code
 * for each message for WebAssembly. I haven't really thought about how that would work yet.
 */
typedef struct RNStdMsgString RNStdMsgString;

RNContext* rn_get_default_context(void);
RNNode* rn_create_node(RNContext* rn_context, const uint8_t* name, size_t name_len);
RNPublisher* rn_create_publisher(RNNode* rn_node, const uint8_t* topic, size_t topic_len);
RNStdMsgString* rn_std_msg_string_default(void);
void rn_std_msg_string_set_data(RNStdMsgString* rn_std_msg_string, const uint8_t* data,
                                size_t data_len);
void rn_publish(RNPublisher* rn_publisher, const RNStdMsgString* rn_std_msg_string);
void rn_thread_sleep(uint32_t millis);

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif
