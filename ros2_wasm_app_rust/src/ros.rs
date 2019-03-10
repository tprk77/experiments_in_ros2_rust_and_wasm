// Copyright (c) 2019 Tim Perkins

use crate::raw;

pub struct Context {
    rn_context: *mut raw::RNContext,
}

pub struct Node<'a> {
    rn_node: *mut raw::RNNode,
    _context: &'a Context,
}

pub struct Publisher<'a, 'b> {
    rn_publisher: *mut raw::RNPublisher,
    _node: &'a Node<'b>,
}

pub struct StdMsgString {
    rn_std_msg_string: *mut raw::RNStdMsgString,
}

impl Context {
    pub fn default() -> Context {
        Context {
            rn_context: unsafe {
                raw::rn_get_default_context()
            },
        }
    }

    pub fn create_node(&mut self, name: &str) -> Node {
        Node {
            rn_node: unsafe {
                raw::rn_create_node(self.rn_context, name.as_ptr(), name.len())
            },
            _context: self,
        }
    }
}

impl<'a> Node<'a> {
    pub fn create_publisher(&mut self, topic: &str) -> Publisher {
        Publisher {
            rn_publisher: unsafe {
                raw::rn_create_publisher(self.rn_node, topic.as_ptr(), topic.len())
            },
            _node: self,
        }
    }
}

impl<'a, 'b> Publisher<'a, 'b> {
    pub fn publish(&mut self, message: &StdMsgString) {
        unsafe {
            raw::rn_publish(self.rn_publisher, message.rn_std_msg_string);
        }
    }
}

impl StdMsgString {
    pub fn default() -> StdMsgString {
        StdMsgString {
            rn_std_msg_string: unsafe {
                raw::rn_std_msg_string_default()
            },
        }
    }

    pub fn set(&mut self, data: &str) {
        unsafe {
            raw::rn_std_msg_string_set_data(self.rn_std_msg_string, data.as_ptr(), data.len());
        }
    }
}

pub fn sleep(millis: u32) {
    unsafe {
        raw::rn_thread_sleep(millis);
    }
}
