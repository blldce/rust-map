use std::alloc::{alloc, dealloc};
use std::{alloc::Layout, fmt::Debug};

pub struct Node<K: Debug + PartialEq, V: Debug> {
    key: K,
    value: V,
    next_node: *mut Node<K, V>,
    previous_node: *mut Node<K, V>,
}

pub struct Map<K: Debug + PartialEq, V: Debug> {
    size: usize,
    head: *mut Node<K, V>,
}

impl<K: Debug + PartialEq, V: Debug> Map<K, V> {
    pub fn new() -> Map<K, V> {
        Map {
            size: 0,
            head: 0 as *mut Node<K, V>,
        }
    }
    pub unsafe fn add(&mut self, key: K, value: V) -> bool {
        if self.check_key(&key) {
            return false;
        }
        let mut new_node_ptr = alloc(Layout::new::<Node<K, V>>()) as *mut Node<K, V>;
        (*new_node_ptr).key = key;
        (*new_node_ptr).value = value;

        // first time?
        if self.head == (0 as *mut Node<K, V>) {
            // we do not need to check key, its first addition anyway
            (*new_node_ptr).next_node = 0 as *mut Node<K, V>;
            (*new_node_ptr).previous_node = 0 as *mut Node<K, V>;
        } else {
            (*self.head).next_node = new_node_ptr;
            (*new_node_ptr).previous_node = self.head;
            (*new_node_ptr).next_node = 0 as *mut Node<K, V>;
        }
        self.head = new_node_ptr;
        self.size += 1;
        true
    }

    pub unsafe fn remove_item(&mut self, key: K) -> bool {
        if !self.check_key(&key) {
            false;
        }
        let mut iter = self.head as *mut Node<K, V>;
        while iter != (0 as *mut Node<K, V>) {
            if (*iter).key == key {
                let curr_next_node = (*iter).next_node;
                let curr_previous_node = (*iter).previous_node;
                if curr_next_node != (0 as *mut Node<K, V>) {
                    (*curr_next_node).previous_node = curr_previous_node;
                }
                if curr_previous_node != (0 as *mut Node<K, V>) {
                    (*curr_previous_node).next_node = curr_next_node;
                }
                dealloc(iter as *mut u8, Layout::new::<Node<K, V>>());
                self.size -= 1;
                true;
            }
            iter = (*iter).previous_node;
        }
        false
    }

    pub unsafe fn get(&mut self, key: K) -> Option<&V> {
        let mut iter = self.head as *mut Node<K, V>;
        while iter != (0 as *mut Node<K, V>) {
            if (*iter).key == key {
                return Some(&(*iter).value);
            }
            iter = (*iter).previous_node;
        }
        return None;
    }

    unsafe fn check_key(&self, key: &K) -> bool {
        if self.head == (0 as *mut Node<K, V>) {
            return false;
        }
        let mut iter = self.head as *mut Node<K, V>;
        while iter != (0 as *mut Node<K, V>) {
            if &(*iter).key == key {
                return true;
            }
            iter = (*iter).previous_node;
        }
        return false;
    }

    pub unsafe fn print_items(&self) -> () {
        println!("**********************");
        let mut iter = self.head as *mut Node<K, V>;
        while iter != (0 as *mut Node<K, V>) {
            println!("Address : {:p} -- Key : {:?} -- Value : {:?} -- Previous Node : {:p} -- Next Node : {:p}", iter,(*iter).key, (*iter).value, (*iter).previous_node , (*iter).next_node);
            iter = (*iter).previous_node;
        }
    }
}
