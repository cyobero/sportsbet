use substring::Substring;

fn main() {
    let dt_str = "2022-06-15T16:30:00.12345678";
    let sub = dt_str.substring(0, dt_str.len() - 9);

    println!("dt_str: {}", sub);
    println!("year: {:?}", &sub[0..4].to_owned().parse::<i32>().unwrap());
    println!("month: {:?}", &sub[5..7].to_owned().parse::<i32>().unwrap());
    println!("day: {:?}", &sub[8..10].to_owned().parse::<i32>().unwrap());
    println!(
        "hour: {:?}",
        &sub[11..13].to_owned().parse::<i32>().unwrap()
    );
    println!(
        "minute: {:?}",
        &sub[14..16].to_owned().parse::<i32>().unwrap()
    );
}
