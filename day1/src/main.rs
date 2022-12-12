fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    let mut largest = 0;

    for group in input.replace("\r\n", "\n").split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>()?;
            sum += value;
        }

        if sum > largest {
            largest = sum;
        }
    }
    println!("Largeset group has sum: {largest}");

    Ok(())
}
