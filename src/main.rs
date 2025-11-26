use slab::Slab;
use std::collections::{BTreeMap, VecDeque};

// Simple scooter
#[derive(Debug, Clone, Copy)]
struct Order {
    id: u64,
    price: u64,
    quantity: u64,
}

// Pointer: strong typing for index
type OrderIndex = usize;

// Queue of pointers
// Stays hot in L1 cache
#[derive(Debug)]
struct Level {
    orders: VecDeque<OrderIndex>,
}

struct OrderBook {
    // Contiguous memory block
    // O(1) access by index
    arena: Slab<Order>,

    // Maps price -> level
    // Sorted by price
    bids: BTreeMap<u64, Level>,
    asks: BTreeMap<u64, Level>,
}

impl OrderBook {
    fn new() -> OrderBook {
        Self {
            arena: Slab::new(),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn add_order(&mut self, price: u64, quantity: u64) {
        // 1. Allocate in Arena (Data)
        let entry = self.arena.vacant_entry();
        let id = entry.key();
        entry.insert(Order {
            id: id as u64,
            price: price,
            quantity: quantity,
        });

        // 2. Insert Index into Level (Ordering)
        self.bids
            .entry(price)
            .or_insert_with(|| Level {
                orders: VecDeque::new(),
            })
            .orders
            .push_back(id);
    }
}

fn main() {
    println!("Hello world");
}
