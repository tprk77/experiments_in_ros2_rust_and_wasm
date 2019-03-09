// Copyright (c) 2019 Tim Perkins

use std::collections::HashMap;
use std::rc::Rc;
use wasmer_runtime::{
    Ctx,
    ImportObject,
    func,
    imports,
    memory::MemoryView,
};
use rclrs;
use std_msgs;

/// This module contains the ROS interface to be provided to the WebAssembly environment. We will
/// need to add each to the `ImportObject` for them to be accessible. This is a very minimal
/// interface. Basically just enough to run the publisher and subscriber node demos.

/// This will get all the relavent imports from this module.
pub fn get_imports() -> ImportObject {
    imports! {
        "env" => {
            "rn_get_default_context" => func!(rn_get_default_context),
            "rn_create_node" => func!(rn_create_node),
            "rn_create_publisher" => func!(rn_create_publisher),
            "rn_std_msg_string_default" => func!(rn_std_msg_string_default),
            "rn_std_msg_string_set_data" => func!(rn_std_msg_string_set_data),
            "rn_publish" => func!(rn_publish),
            "rn_thread_sleep" => func!(rn_thread_sleep),
        },
    }
}

/// This will be used to store all of our ROS stuff.
pub struct RosData {
    contexts: HashMap<u32, rclrs::Context>,
    nodes: HashMap<u32, rclrs::Node>,
    publishers: HashMap<u32, rclrs::Publisher<std_msgs::msg::String>>,
    messages: HashMap<u32, std_msgs::msg::String>,
}

impl RosData {
    pub fn new() -> RosData {
        RosData {
            contexts: HashMap::new(),
            nodes: HashMap::new(),
            publishers: HashMap::new(),
            messages: HashMap::new(),
        }
    }
}

fn register_at_index<T>(map: &mut HashMap<u32, T>, item: T) -> u32 {
    // WARNING Pretty janky...
    let mut new_id = 0;
    while map.contains_key(&new_id) { new_id += 1; }
    map.insert(new_id, item);
    new_id
}

fn get_string_from_memory(ctx: &mut Ctx, ptr: u32, len: u32) -> String {
    let memory = ctx.memory(0);
    let view: MemoryView<u8> = memory.view();
    let str_vec: Vec<u8> = view[ptr as usize..(ptr + len) as usize]
        .iter().map(|cell| cell.get()).collect();
    String::from_utf8(str_vec).unwrap()
}

// NOTE You can verify what the signatures should be by compiling the app to WASM, and then
// disassembling it to WAST. As you will see, pointers become just `u32` ints.

pub fn rn_get_default_context(ctx: &mut Ctx) -> u32 {
    // Create a new context and store it
    println!("[TRACE] rn_get_default_context");
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let context = rclrs::Context::default();
    register_at_index(&mut ros_data.contexts, context)
}

pub fn rn_create_node(ctx: &mut Ctx, rn_context_ptr: u32, name_ptr: u32, name_len: u32) -> u32 {
    // Look up the context, create a new node, and store it
    println!("[TRACE] rn_create_node ({})", rn_context_ptr);
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let context = ros_data.contexts.get_mut(&rn_context_ptr).unwrap();
    let node_name = get_string_from_memory(ctx, name_ptr, name_len);
    let node = context.create_node(&node_name).unwrap();
    register_at_index(&mut ros_data.nodes, node)
}

pub fn rn_create_publisher(ctx: &mut Ctx, rn_node_ptr: u32,
                           topic_ptr: u32, topic_len: u32) -> u32 {
    // Look up the node, create a new publisher, and store it
    println!("[TRACE] rn_create_publisher ({})", rn_node_ptr);
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let node = ros_data.nodes.get_mut(&rn_node_ptr).unwrap();
    let topic = get_string_from_memory(ctx, topic_ptr, topic_len);
    let publisher = node
        .create_publisher::<std_msgs::msg::String>(&topic, rclrs::QOS_PROFILE_DEFAULT)
        .unwrap();
    register_at_index(&mut ros_data.publishers, publisher)
}

pub fn rn_std_msg_string_default(ctx: &mut Ctx) -> u32 {
    // Create a new message and store it
    println!("[TRACE] rn_std_msg_string_default");
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let message = std_msgs::msg::String::default();
    register_at_index(&mut ros_data.messages, message)
}

pub fn rn_std_msg_string_set_data(ctx: &mut Ctx, rn_std_msg_string_ptr: u32,
                                  data_ptr: u32, data_len: u32) {
    // Look up a message and update the data
    println!("[TRACE] rn_std_msg_string_set_data ({})", rn_std_msg_string_ptr);
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let message = ros_data.messages.get_mut(&rn_std_msg_string_ptr).unwrap();
    message.data = get_string_from_memory(ctx, data_ptr, data_len);
}

pub fn rn_publish(ctx: &mut Ctx, rn_publisher_ptr: u32, rn_std_msg_string_ptr: u32) {
    // Look up the publisher, the message, and then send the message
    println!("[TRACE] rn_publish ({}, {})", rn_publisher_ptr, rn_std_msg_string_ptr);
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let publisher = ros_data.publishers.get_mut(&rn_publisher_ptr).unwrap();
    let message = ros_data.messages.get_mut(&rn_std_msg_string_ptr).unwrap();
    publisher.publish(message).unwrap();
}

pub fn rn_thread_sleep(_ctx: &mut Ctx, millis: u32) {
    // Just sleep for a while
    println!("[TRACE] rn_thread_sleep");
    std::thread::sleep(std::time::Duration::from_millis(millis.into()));
}
