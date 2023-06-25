use std::{sync::Arc, cell::{UnsafeCell, RefCell}, ops::Range, slice::from_raw_parts_mut};

use parking_lot::{RawRwLock as CustomizableLock, lock_api::{RawRwLock, RawRwLockUpgrade}};



pub struct RegionGuard<'a, T> {
    pub locks: RefCell<RegionLock>,
    pub data: Vec<Arc<UnsafeCell<&'a mut [T]>>>,
}

impl<'a, T> Clone for RegionGuard<'a, T> {
    fn clone(&self) -> Self {
        Self { locks: RefCell::new(self.locks.borrow().clone()), data: self.data.clone()}
    }
}

unsafe impl<'a, T> Sync for RegionGuard<'a, T> {}
unsafe impl<'a, T> Send for RegionGuard<'a, T> {}

impl<'a, T> RegionGuard<'a, T> {

    pub fn new(data: &'a mut [T], region_bounds: Vec<(usize, usize)>) -> Self {
        let tree_regions: Vec<(usize, usize)> = region_bounds.iter().enumerate().map(|(i, (rs, _))| (*rs, i)).collect();

        let regions = RegionArray::from_regions(tree_regions.clone());
        let data = get_mutable_from_regions(data, region_bounds);
        let sync: Arc<Vec<CustomizableLock>> = Arc::new((0..data.len()).into_iter().map(|_| CustomizableLock::INIT ).collect());

        let locks = RefCell::new(RegionLock {
            lock: None,
            sync: sync,
            regions,
            current_bounds: None
        });

        Self { 
            locks,
            data,
        }
    }

    pub fn index(&self, index: usize) -> &T {
        let (region, key) = self.locks.borrow_mut().read(index);
        unsafe {
            &(*self.data[region].get())[index - key]
        }
    }

    pub fn index_2(&self, (region, entry): (usize,usize)) -> &T {
        self.locks.borrow_mut().read(region);
        unsafe {
            &(*self.data[region].get())[entry]
        }
    }

    pub fn index_mut(&mut self, index: usize) -> &mut T {
        let (region, key) = self.locks.borrow_mut().write(index);
        unsafe {
            &mut (*self.data[region].get())[index - key]
        }
    }

    pub fn index_mut_2(&mut self, (region, entry): (usize,usize)) -> &mut T {
        self.locks.borrow_mut().write(region);
        unsafe {
            &mut (*self.data[region].get())[entry]
        }
    }

    pub fn index_range(&mut self, index: Range<usize>) -> &[T] {
        let (key, region) = self.locks.borrow_mut().read(index.start);
        let (_, end_region) = self.locks.borrow_mut().read(index.end-1);

        if region != end_region {
            panic!("Regions need to be the same: {}..{}:  {},{}", index.start, index.end, region, end_region);
        }

        self.locks.borrow_mut().write(region);
        unsafe {
            &(*self.data[region].get())[(index.start - key)..(index.end - key)]
        }
    }

    pub fn set(&mut self, index: usize, val: T) {
        let (key, region) = self.locks.borrow_mut().write(index);

        unsafe {
            (*self.data[region].get())[index - key] = val;
        }
    }

    pub fn set_range(&mut self, index: usize, val: Vec<T>) {
        let (key, region) = self.locks.borrow_mut().write(index);

        unsafe {
            val.into_iter()
                .enumerate()
                .for_each(|(i, n)| (*self.data[region].get())[index - key + i] = n);
        }
    }

    pub fn free(&mut self) {
        self.locks.borrow_mut().free();
    }
}

#[derive(Clone)]
pub struct RegionLock {
    pub lock: Option<(usize, bool)>,
    pub sync: Arc<Vec<CustomizableLock>>,
    pub regions: RegionArray,
    pub current_bounds: Option<(usize, usize, usize)>
}

impl Drop for RegionLock {
    fn drop(&mut self) {
        if let Some((l, writable)) = self.lock {
            unsafe {
                if writable {
                    self.sync[l].unlock_exclusive();
                } else {
                    self.sync[l].unlock_upgradable();
                }
            }
            
        }
    }
}

pub enum LockState {
    Read,
    Write
}

impl RegionLock {
    fn free(&mut self) {
        if let Some((l, writable)) = self.lock {
            unsafe {
                if writable {
                    self.sync[l].unlock_exclusive();
                } else {
                    self.sync[l].unlock_upgradable();
                }
            }
            
        }
        self.lock = None;
    }

    fn get_region(&mut self, index: usize) -> (usize, usize) {
        if let Some((k, l, u)) = self.current_bounds {
            if l <= index && u > index {
                (l, k)
            } else {
                self.regions.find(index)
            }
        } else {
            self.regions.find(index)
        }
    }

    fn read(&mut self, index: usize) -> (usize, usize) {
        let (lower, idx) = self.get_region(index);

        if let Some((l, writable)) = self.lock {
            if l == idx {
                return (idx, lower);
            }

            unsafe {
                if writable {
                    self.sync[l].unlock_exclusive();
                } else {
                    self.sync[l].unlock_upgradable();
                }
            }
            
        }
        self.sync[idx].lock_upgradable();
        self.lock = Some((idx, false));
        self.current_bounds = Some((idx, lower, self.regions.upper_bound(idx)));

        (idx, lower)
    }

    fn write(&mut self, index: usize) -> (usize, usize) {
        let (lower, idx) = self.get_region(index);

        if let Some((l, writable)) = self.lock {
            if l == idx && writable {
                return (idx, lower);
            } else if l == idx {
                unsafe {
                    self.sync[l].upgrade();
                    self.lock = Some((idx, true));
                    return (idx, lower)
                }
            }

            unsafe {
                if writable {
                    self.sync[l].unlock_exclusive();
                } else {
                    self.sync[l].unlock_upgradable();
                }
            }
            
        }
        self.sync[idx].lock_exclusive();
        self.lock = Some((idx, true));
        self.current_bounds = Some((idx, lower, self.regions.upper_bound(idx)));

        (idx, lower)
    }
}

#[derive(Clone, Debug)]
pub struct RegionArray(Vec<(usize, usize)>, usize, usize);

impl RegionArray {
    pub fn from_regions(regions: Vec<(usize, usize)>) -> Self {
        let mut t = regions;
        t.sort_by(|a,b| a.0.cmp(&b.0));
        let len = t.len();
        
        for i in 0..len {
            if i == len -1 {
                t[i].1 = usize::MAX;
            }
            else {
                t[i].1 = t[i+1].0;
            }
        }

        let avg =  t.iter().map(|(a,b)| *b - *a).sum::<usize>() / len;
        Self(t, len, avg)   
    }

    pub fn upper_bound(&self, region: usize) -> usize {
        self.0[region].1
    }

    pub fn find(&self, key: usize) -> (usize, usize) {
        let mut current = key / self.2;
        let mut last = self.1;

        loop {
            let t = self.0[current];
            if t.0 > key {
                last = current;
                current = current / 2;
            } else if t.1 <= key {
                current += (last - current) / 2;
            } else {
                return (t.0, current)
            }
        }
    }
}

fn get_mutable_from_regions<'a, T>(data: &'a mut [T], regions: Vec<(usize, usize)>) -> Vec<Arc<UnsafeCell<&'a mut [T]>>> {
    let ptr = data.as_mut_ptr();

        unsafe {
            regions
                .iter()
                .map(|(start, end)| {
                    Arc::new(UnsafeCell::new(from_raw_parts_mut(ptr.clone().add(*start), *end - *start)))
                }).collect()
        }
}