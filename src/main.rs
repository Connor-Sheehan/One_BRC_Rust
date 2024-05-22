use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::time::Instant;


struct Stat { 
    min: i16,
    max: i16,
    sum: i32,
    count: i32,
}

impl Stat {
    fn add(&mut self, v: i16){
        self.min = self.min.min(v);
        self.max = self.max.max(v);
        self.sum += v as i32;
        self.count += 1;
    }
}

fn main() {
    let total_time = Instant::now();

    let filename = "million rows.txt";
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut stations: HashMap<String, Stat> = HashMap::new();

    for line in reader.lines() {//loads all data points into the hash map
        let line_str = line.expect("Error reading line");
        let parts: Vec<&str> = line_str.split(';').collect();
        add_2(parts[0].to_string(), (parts[1].trim().parse::<f32>().expect("failed to parse") * 10.0) as i16, &mut stations);
    }

    for stat in stations{// primts the output
        println!("{};{};{};{}", stat.0, stat.1.min as f32 / 10.0, stat.1.max as f32 / 10.0, (stat.1.sum / stat.1.count) as f32 / 10.0 ); 
    }

    println!("Total time : {:?}", total_time.elapsed());
}

fn add_2(s: String, v: i16, hm: &mut HashMap<String, Stat>){
    if !hm.contains_key(&s) {
        let station  = Stat {min: v, max: v, sum: v as i32, count: 1};
        hm.insert(s, station);
    }else{
    hm.get_mut(&s).expect("getting struct from hashmap failed").add(v);
    }
}