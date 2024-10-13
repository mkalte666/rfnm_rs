use rfnm::discover_usb_boards;

fn main() {
    let boards = discover_usb_boards();
    println!("Found {} RFNM boards.", boards.len());
    for (num,board) in boards.iter().enumerate() {
        println!("Board {num}:");
        println!("    {:#?}",board);
    }
}