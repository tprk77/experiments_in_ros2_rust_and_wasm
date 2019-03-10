// Copyright (c) 2019 Tim Perkins

mod raw;
mod ros;

#[no_mangle]
pub extern fn ros_main() {
    let mut context = ros::Context::default();
    let mut node = context.create_node("node");
    let mut publisher = node.create_publisher("topic");

    let mut message = ros::StdMsgString::default();

    for i in 0..10 {
        let data = format!("Hello Rust ROS! {}", i);
        message.set(&data);
        publisher.publish(&message);
        ros::sleep(1000);
    }
}
