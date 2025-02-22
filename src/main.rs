use std::env;
use std::fs;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let output_folder = &args[2];
    process_image_on_new_thread(file_name.clone(), output_folder.clone(), TintType::RED);
    process_image_on_new_thread(file_name.clone(), output_folder.clone(), TintType::BLUE);
}

#[derive(Copy, Clone)]
enum TintType {
    RED,
    BLUE
}

impl TintType {
    fn color_byte_order_number(self) -> i32 {
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

fn process_image_on_new_thread(file_name: String, output_folder: String, tint_type: TintType) {
    thread::spawn(move || {
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
    
        let pixels_start_at = 
        18 + usize::from(*image_id_length) + usize::from(color_map_length);
    
        let mut output_bytes: Vec<u8> = Vec::new();
        let mut current_color = 0;
        for (i, byte) in input_bytes.iter().enumerate() {
            if i < pixels_start_at {
                output_bytes.push(*byte);
                continue;
            }
    
            if current_color == 3 {
                current_color = 0;
            }
            
            if current_color == tint_type.color_byte_order_number() {
                output_bytes.push(0);
            } else {
                output_bytes.push(*byte);
            }
    
            current_color = current_color + 1;
        }
    
        let output_file_path = output_folder.to_owned() + &tint_type.name() + ".tga";
        fs::write(&output_file_path, output_bytes).expect("Writing file failed");
    });
}
