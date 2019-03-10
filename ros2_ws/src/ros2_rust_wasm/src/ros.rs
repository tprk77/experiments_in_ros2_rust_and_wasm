// Copyright (c) 2019 Tim Perkins

use std::collections::HashMap;
use std::rc::Rc;
use wasmer_runtime::{
    Ctx,
    ImportObject,
    Instance,
    Value,
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
            "rn_create_subscription" => func!(rn_create_subscription),
            "rn_std_msg_string_default" => func!(rn_std_msg_string_default),
            "rn_std_msg_string_set_data" => func!(rn_std_msg_string_set_data),
            "rn_std_msg_string_get_data_len" => func!(rn_std_msg_string_get_data_len),
            "rn_std_msg_string_get_data" => func!(rn_std_msg_string_get_data),
            "rn_publish" => func!(rn_publish),
            "rn_thread_sleep" => func!(rn_thread_sleep),
            "rn_node_spin" => func!(rn_node_spin),
            "rn_log" => func!(rn_log),
        },
    }
}

/// This will be used to store all of our ROS stuff.
pub struct RosData {
    contexts: HashMap<u32, rclrs::Context>,
    nodes: HashMap<u32, rclrs::Node>,
    publishers: HashMap<u32, rclrs::Publisher<std_msgs::msg::String>>,
    subscriptions: HashMap<u32, Rc<rclrs::Subscription<std_msgs::msg::String>>>,
    messages: HashMap<u32, std_msgs::msg::String>,
    // Storing the instance is my janky solution for callbacks. The instance lets me get call
    // exported functions from the native callback.
    instance: *mut Instance,
}

impl RosData {
    pub fn new(instance: *mut Instance) -> RosData {
        RosData {
            contexts: HashMap::new(),
            nodes: HashMap::new(),
            publishers: HashMap::new(),
            subscriptions: HashMap::new(),
            messages: HashMap::new(),
            instance,
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

fn set_memory_to_string(ctx: &mut Ctx, ptr: u32, len: u32, string: &str) {
    let memory = ctx.memory(0);
    let view: MemoryView<u8> = memory.view();
    let strbuff = &view[ptr as usize..(ptr + len) as usize];
    for i in 0..std::cmp::min(string.len(), len as usize) {
        strbuff[i].replace(string.as_bytes()[i]);
    }
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

pub fn rn_create_subscription(ctx: &mut Ctx, rn_node_ptr: u32,
                              topic_ptr: u32, topic_len: u32,
                              func_ptr: u32) -> u32 {
    // Look up the node, create a new subscription, and store it
    println!("[TRACE] rn_create_subscription ({})", rn_node_ptr);
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let node = ros_data.nodes.get_mut(&rn_node_ptr).unwrap();
    let topic = get_string_from_memory(ctx, topic_ptr, topic_len);
    // WARNING We will move this pointer to the closure
    let ros_data_ptr = ctx.data as *mut RosData;
    let subscription = node.create_subscription::<std_msgs::msg::String, _>(
        &topic, rclrs::QOS_PROFILE_DEFAULT,
        move |message: &std_msgs::msg::String| {
            // WARNING This is very, very unsafe, but this is a demo
            let ros_data: &mut RosData = unsafe { &mut *ros_data_ptr };
            let message_to_store = std_msgs::msg::String {
                data: message.data.clone(),
            };
            let index = register_at_index(&mut ros_data.messages, message_to_store);
            let instance: &mut Instance = unsafe { &mut *ros_data.instance };
            let args = &[Value::I32(func_ptr as i32), Value::I32(index as i32)];
            instance.call("ros_dispatcher", args).unwrap();
        }).unwrap();
    register_at_index(&mut ros_data.subscriptions, subscription)
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

pub fn rn_std_msg_string_get_data_len(ctx: &mut Ctx, rn_std_msg_string_ptr: u32) -> u32 {
    // Look up a message and return the pointer
    println!("[TRACE] rn_std_msg_string_get_data_len ({})", rn_std_msg_string_ptr);
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let message = ros_data.messages.get_mut(&rn_std_msg_string_ptr).unwrap();
    message.data.len() as u32
}

pub fn rn_std_msg_string_get_data(ctx: &mut Ctx, rn_std_msg_string_ptr: u32,
                                  data_ptr: u32, data_len: u32) {
    // Look up a message and copy the contents into WASM memory
    println!("[TRACE] rn_std_msg_string_get_data ({})", rn_std_msg_string_ptr);
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let message = ros_data.messages.get_mut(&rn_std_msg_string_ptr).unwrap();
    set_memory_to_string(ctx, data_ptr, data_len, &message.data);
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

pub fn rn_node_spin(ctx: &mut Ctx, rn_node_ptr: u32) {
    // Look up the node and spin it
    println!("[TRACE] rn_node_spin");
    let ros_data: &mut RosData = unsafe { &mut *(ctx.data as *mut RosData) };
    let node = ros_data.nodes.get_mut(&rn_node_ptr).unwrap();
    rclrs::spin(&node).unwrap();
}

pub fn rn_log(ctx: &mut Ctx, text_ptr: u32, text_len: u32) {
    // Just print the input as if this were a logger
    println!("[TRACE] rn_log");
    let text = get_string_from_memory(ctx, text_ptr, text_len);
    println!("[LOG] {}", text);
}
