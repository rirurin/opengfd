use allocator_api2::alloc::{Allocator, Global};
use std::{
    alloc::Layout,
    fmt::{Display, Formatter},
    error::Error,
    ptr::NonNull
};
use std::fmt::Debug;
use opengfd::{
    kernel::allocator::GfdAllocator,
    utility::{
        item_array::ItemArray,
        stream::{ChunkHeader, StreamFactory, GfdSerialize}
    }
};
use opengfd::object::mesh::Mesh;
use opengfd::utility::misc::RGBAFloat;
use opengfd::utility::stream::SerializationSingleAllocator;

fn main() {
    let file = std::fs::read("E:/Metaphor/base_cpk/COMMON/model/character/0001/c_0001_001_B.GFS").unwrap();
    let factory = StreamFactory::new(Global);
    let mut stream = factory.read_from_memory(&file).unwrap();
    let mesh = unsafe { Mesh::<GfdAllocator>::stream_read(&mut stream, &mut SerializationSingleAllocator::new(GfdAllocator)).unwrap().into_raw().as_ref() };
    println!("{:?}", mesh);
    /*
    let mut array = ItemArray::<u32, Global>::new(Global);
    let mut other_array = (10..20).map(|v| v * 2).collect::<Vec<_>>();
    for i in 0..10 {
        array.push(i * 2).unwrap();
    }
    array.extend(&other_array).unwrap();
    for i in 0..10 {
        array.remove(i).unwrap();
    }
    println!("{:?}", array);
    */
}