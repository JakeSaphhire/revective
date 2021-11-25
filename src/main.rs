use serialport as sp;
mod point;

fn main() {
    let ports = sp::available_ports().expect("Failed to list ports");
    let pt : point::Point = point::Point::new(0b01000000u8, 2300, 3456);
    for port in ports.iter() {
        println!("{}", port.port_name);
    }
    let mut wport = sp::new(&ports[0].port_name, 115200).open().expect("Failed to open port");
    pt.send(wport.as_mut()).expect("Write failed!!");
    println!("{}", pt.flags);
}
