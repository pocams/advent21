use color_eyre::Report;

pub(crate) fn solve(lines: &[String]) -> Result<(), Report> {
    let numbers = lines.iter().map(|l| u32::from_str_radix(l, 2)).collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", numbers);
    Ok(())
}