use bytes::{Buf, BufMut, Bytes, BytesMut};
///
/// bytes 不支持类似position的回退。因为bytes不存在类似的position。
/// 内部有一个指针来充当类似的postion，具体可以看inc_start(cnt)的函数实现。
fn main() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    //将两个buf连接在一起,其实并不是真正组成一个新的数组
    let chain = buf.chain(&b"world"[..]);
    println!("{:?}", chain);
    assert_eq!(chain.chunk(), &b"hello world"[..]);
    println!("{:?}", chain.last_ref());
    println!("{:?}", chain.chunk());
    use_write_rust_type();
    use_bytes_in_read();
}
//构建一个写buff,往里面写数据、并读取
fn use_write_rust_type() {
    let mut buf = BytesMut::with_capacity(64);
    //将a写到buff中
    buf.put_bytes(b'a', 1);
    //通过get_u8获取到对应的数据，同时buf内部游标指向1
    println!("{:?}", buf.get_u8() as char);
    //由于此时内部remaining为0，再次调用会出现painc
    //println!("{:?}",buf.get_u8() as char);
}
//构建一个可读buff,利用bytes来实现各种数据操作
fn use_bytes_in_read() {
    let mut bytes = Bytes::from("hello world, this is a test 123445");
    //提取hello world,返回的bytes的切片视图
    let hello = bytes.slice(0..12);
    //输出hello world,
    println!("{:?}", hello);
    //将bytes切分成两个bytes,这时候bytes真正被分开.数据分布结构如下
    //
    //    Arc ptrs                   ┌─────────┐
    //    ________________________ / │ Bytes 2 │
    //   /                           └─────────┘
    //  /          ┌───────────┐     |         |
    // |_________/ │  Bytes 1  │     |         |
    // |           └───────────┘     |         |
    // |           |           | ___/ data     | tail
    // |      data |      tail |/              |
    // v           v           v               v
    // ┌─────┬─────┬───────────┬───────────────┬─────┐
    // │ Arc │     │           │               │     │
    // └─────┴─────┴───────────┴───────────────┴─────┘
    let other_byte = bytes.split_to(12);
    //输出hello world,
    println!("{:?}", other_byte);
    //输出this is a test 123445
    println!("{:?}", bytes);
    //将数据truncate
    bytes.truncate(15);
    // 输出 this is a test
    println!("{:?}", bytes);
    // 输出0到remain之间的数据
    println!("{:?}", String::from_utf8_lossy(bytes.chunk()));
    // advance 调整内部游标
    bytes.advance(6);
    println!("{:?}",bytes.len());
    //输出结果是is a test
    println!("{:?}", String::from_utf8_lossy(bytes.chunk()));
}
