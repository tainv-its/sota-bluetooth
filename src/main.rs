extern crate bluez;

use bluez::adapter::Adapter; 
use bluez::bluetooth_device::{BluetoothDevice, GattService, GattCharacteristic};
use std::{thread, time};

fn main() {
    let adapter = Adapter::new().unwrap();
    let device = BluetoothDevice::new("your-device-name", &adapter).unwrap();

    // Tạo service để gửi qua lại dữ liệu giữa Pi và các thiết bị khác.
    let service = GattService::new("your-service-uuid", true);

    // Tạo characteristic dùng để đại diện cho service.
    let props = ["read", "write"];
    let characteristic = GattCharacteristic::new("your-characteristic-uuid", &props, &service.get_path());

    // Thêm characteristic vào service.
    service.add_characteristic(characteristic.clone()).unwrap();

    // Đăng ký service với bluez.
    device.register_service(service).unwrap();

    // Bắt đầu thực hiện quảng bá advertiser.
    println!("Starting BLE advertise");
    device.start_advertising().unwrap();

    // Chờ đợi cho tới khi tự động connect được thiết bị khác.
    loop {
        for connection in &adapter.get_connections().unwrap() {
            if connection.device == device.get_object_path().unwrap() {
                println!("Connection from {}", connection.address);
                
                // Xác nhận đường kết nối (accept the pairing request without prompting).
                device.accept_connection(&connection.address).unwrap();

                // Thực hiện xử lý sau khi thiết bị kết nối thành công.
                handle_connection(connection);
            }
        }

        // Ngủ 1000 miliseconds.
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn handle_connection(conn: &bluez::adapter::Connection) {
    println!("Connected to {}\nRequesting data...", conn.address);

    // TODO: Thực hiện các xử lý cần thiết khi kết nối thành công.
    // Ví dụ: Lấy thông tin từ connection và thao tác với các characteristic trong service của Pi.
}
