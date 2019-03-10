// Copyright (c) 2019 Tim Perkins

mod raw;

#[no_mangle]
pub extern fn ros_main() {
    unsafe {
        let context = raw::rn_get_default_context();
        let node = raw::rn_create_node(context, "node".as_ptr(), 4);
        let publisher = raw::rn_create_publisher(node, "topic".as_ptr(), 5);

        let message = raw::rn_std_msg_string_default();

        for i in 0..10 {
            let data = format!("Hello Rust ROS! {}", i);
            raw::rn_std_msg_string_set_data(message, data.as_ptr(), data.len());
            raw::rn_publish(publisher, message);
            raw::rn_thread_sleep(1000);
        }
    }
}
