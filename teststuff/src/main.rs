use std::sync::{Arc, Mutex};

pub struct DoubleBuffer<T> {
    current: Arc<Mutex<T>>,
    next: Arc<Mutex<T>>,
}

impl<T: Clone> DoubleBuffer<T> {
    pub fn new(initial_value: T) -> Self {
        let current = Arc::new(Mutex::new(initial_value.clone()));
        let next = Arc::new(Mutex::new(initial_value));
        Self { current, next }
    }

    pub fn swap(&mut self) {
        // Lock both Mutexes
        let mut current_lock = self.current.lock().unwrap();
        let mut next_lock = self.next.lock().unwrap();

        // Swap the underlying values by swapping the references
        let mut x = &(*current_lock);
        let mut y = &(*next_lock);
        let z = x;
        x = y;
        y = z;

        // std::mem::swap(&mut *current_lock, &mut *next_lock);
    }

    pub fn get_current(&self) -> std::sync::MutexGuard<'_, T> {
        self.current.lock().unwrap()
    }

    pub fn get_next(&self) -> std::sync::MutexGuard<'_, T> {
        self.next.lock().unwrap()
    }

    pub fn set_next(&mut self, mut new_next: &T) {
        // Lock the next Mutex to ensure exclusive access
        let mut next_lock = self.next.lock().unwrap();
        // Update the value of the next buffer
        self.next = new_next
    }
}

impl<T: Clone> Clone for DoubleBuffer<T> {
    fn clone(&self) -> Self {
        let current_data = self.current.lock().unwrap().clone();
        let current = Arc::new(Mutex::new(current_data));

        let next_data = self.next.lock().unwrap().clone();
        let next = Arc::new(Mutex::new(next_data));

        Self { current, next }
    }
}

fn main() {
    let mut initial: Vec<i32> = Vec::<i32>::with_capacity(5);
    let mut initial2: Vec<i32> = Vec::<i32>::with_capacity(5);
    let mut initial3: Vec<i32> = Vec::<i32>::with_capacity(5);

    for i in 0..5 - 1 {
        initial.push(i);
        initial2.push(0);
        //initial3.push(1);
    }
    println!("filled");
    let mut double_buffer = DoubleBuffer::new(initial);

    for i in 0..1 {
        unsafe {}
    }
    {
        double_buffer.set_next(initial2);
        println!("Before: a={:?})", double_buffer.get_current(),);
        println!("({:p})", &double_buffer.get_current());
        double_buffer.swap();
        println!("Before: a={:?})", double_buffer.get_current(),);
        println!("({:p})", &double_buffer.get_current());
    }
    let mut a = [1, 2, 3];
    let mut b = [4, 5, 6];
    let mut x = &mut a;
    let mut y = &mut b;

    println!(
        "Before: a={:?} ({:p}), b={:?} ({:p})",
        *x,
        &x[0..1],
        *y,
        &y[0]
    );

    std::mem::swap(&mut x, &mut y);

    println!("After:  a={:?} ({:p}), b={:?} ({:p})", *x, &x[0], *y, &y[0]);
}
