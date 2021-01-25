extern crate snowflake_multi_threaded;

use snowflake_multi_threaded::SnowFlakeId;

fn main() {
    let mut id_gen = SnowFlakeId::kubernetes(1);

    let begin = std::time::SystemTime::now();
    let mut id = id_gen.generate_id().unwrap();
    for _i in 0..1000 {
        id = id_gen.generate_id().unwrap();
    }

    let millis = begin.elapsed().unwrap().as_millis();
    let micros = begin.elapsed().unwrap().as_micros();
    let nanos = begin.elapsed().unwrap().as_nanos();

    println!("duration, millis: {}, micros: {}, nanos: {}, final_id: {:?}", millis, micros, nanos, id);
    // 5050505/s, 0.5kw/s
}