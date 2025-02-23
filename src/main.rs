use std::env;
use std::fs;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let output_folder = &args[2];
    let handle1 = process_image_on_new_thread(file_name.clone(), output_folder.clone(), TintType::RED);
    let handle2 = process_image_on_new_thread(file_name.clone(), output_folder.clone(), TintType::BLUE);

    handle1.join();
    handle2.join();
}

#[derive(Copy, Clone)]
enum TintType {
    RED,
    BLUE
}

impl TintType {
    fn color_byte_order_number(self) -> usize {
        match self {
            TintType::RED => 1,
            TintType::BLUE => 2
        }
    }

    fn name(self) -> String {
        match self {
            TintType::RED => String::from("red"),
            TintType::BLUE => String::from("blue")
        }
    }
}

fn process_image_on_new_thread(file_name: String, output_folder: String, tint_type: TintType) -> JoinHandle<()> {
    return thread::spawn(move || {
        let input_bytes = fs::read(file_name).expect("Reading from file failed");
        let data_type = input_bytes.get(2).unwrap();
    
        if data_type != &2 {
            panic!("Only supported file type is .tga uncompressed RGB images.");
        }
    
        let image_id_length = input_bytes.get(0).unwrap();
        let color_map_length_byte_1 = input_bytes.get(5).unwrap();
        let color_map_length_byte_2 = input_bytes.get(6).unwrap();
        let color_map_length_bytes = [*color_map_length_byte_1, *color_map_length_byte_2];
        let color_map_length = u16::from_ne_bytes(color_map_length_bytes);
    
        let pixels_start_at = 18 + usize::from(*image_id_length) + usize::from(color_map_length);
    
        let mut output_bytes: Vec<u8> = Vec::with_capacity(input_bytes.len());
        for (i, byte) in input_bytes.iter().enumerate() {
            if i < pixels_start_at {
                output_bytes.push(*byte);
                continue;
            }
    
            let color_channel = (i - pixels_start_at) % 3;
            if color_channel == tint_type.color_byte_order_number() {
                output_bytes.push(0);
            } else {
                output_bytes.push(*byte);
            }
        }
    
        let output_file_path = format!("{}{}.tga", output_folder, tint_type.name());
        fs::write(&output_file_path, output_bytes).expect("Writing file failed");
    });
}
