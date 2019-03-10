// Copyright (c) 2019 Tim Perkins

mod raw;

#[no_mangle]
pub extern fn ros_dispatcher(func: extern fn(*mut raw::RNStdMsgString),
                             rn_std_msg_string: *mut raw::RNStdMsgString) {
    func(rn_std_msg_string);
}

extern fn callback(rn_std_msg_string: *mut raw::RNStdMsgString) {
    unsafe {
        let data_len = raw::rn_std_msg_string_get_data_len(rn_std_msg_string);
        let data_vec: Vec<u8> = vec![0; data_len];
        raw::rn_std_msg_string_get_data(rn_std_msg_string, data_vec.as_ptr(), data_len);
        raw::rn_log(data_vec.as_ptr(), data_len);
    }
}

#[no_mangle]
pub extern fn ros_main() {
    unsafe {
        let context = raw::rn_get_default_context();
        let node = raw::rn_create_node(context, "node".as_ptr(), 4);
        let _subscriber = raw::rn_create_subscription(node, "topic".as_ptr(), 5, callback);
        raw::rn_node_spin(node);
    }
}
