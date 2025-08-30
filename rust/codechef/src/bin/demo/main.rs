use codechef::{InputReader, OutputWriter};

fn main() -> std::io::Result<()> {
    let mut reader = InputReader::from_local_file()?;
    let mut writer = OutputWriter::to_local_file()?;


    let n: usize = reader.next();
    writer.writeln(format!("n = {}", n))?;

    for _ in 0..n {
        let x: i32 = reader.next();
        let y: i32 = reader.next();
        writer.writeln(format!("x = {}, y = {}", x, y))?;
    }


    // // Read single integer
    // let n: usize = reader.next();
    // writer.writeln(format!("n = {}", n))?;

    // // Read vector of n integers
    // let arr: Vec<i32> = reader.next_vec(n);
    // writer.writeln(format!("arr = {:?}", arr))?;

    // // Read string
    // let s: String = reader.next();
    // writer.writeln(format!("s = {}", s))?;

    // // Read integer + float
    // let x: i32 = reader.next();
    // let y: f64 = reader.next();
    // writer.writeln(format!("x = {}, y = {}", x, y))?;

    Ok(())
}
