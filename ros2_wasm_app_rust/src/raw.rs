// Copyright (c) 2019 Tim Perkins

/// This module contains the "raw" interface to the functions provided by the WebAssembly
/// environment. These will become "import" functions when we generate the WASM. For now, we are
/// only providing a very minimal ROS interface. Basically just enough to run the publisher and
/// subscriber node demos you usually see in tutorials.

pub enum RNContext {}
pub enum RNNode {}
pub enum RNPublisher {}

// This is a major limitation of this interface. We are going to limit the available messages to
// just the `std_msgs::String` message. If this were a real library, we would need to generate code
// for each message for WebAssembly. I haven't really thought about how that would work yet.
pub enum RNStdMsgString {}

extern "C" {
    pub fn rn_get_default_context() -> *mut RNContext;
    pub fn rn_create_node(rn_context: *mut RNContext, name: *const u8,
                          name_len: usize) -> *mut RNNode;
    pub fn rn_create_publisher(rn_node: *mut RNNode,
                               topic: *const u8, topic_len: usize) -> *mut RNPublisher;
    pub fn rn_std_msg_string_default() -> *mut RNStdMsgString;
    pub fn rn_std_msg_string_set_data(rn_std_msg_string: *mut RNStdMsgString,
                                      data: *const u8, data_len: usize) -> ();
    pub fn rn_publish(rn_publisher: *mut RNPublisher,
                      rn_std_msg_string: *const RNStdMsgString) -> ();
    pub fn rn_thread_sleep(millis: u32) -> ();
}
