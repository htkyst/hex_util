mod address_range;
mod data_manager;
mod intel_hex;
mod memory_map;
mod srecord;

fn main() {
    let file_path = String::from("data/test.hex");

    let mut data_mgr = data_manager::HexDataManager::new();

    intel_hex::read_intelhex_file(&file_path, &mut data_mgr).unwrap();

    let data = data_mgr.get_data(0x100, 16);

    println!("{}", data.len());
    for i in data {
        println!("{:x}", i);
    }
}
