use color_eyre::Report;
use tracing::{debug, info};

pub(crate) fn solve(input: String) -> Result<(), Report> {
    // let numbers = lines.iter().map(|l| u32::from_str_radix(l, 2)).collect::<Result<Vec<_>, _>>()?;
    let numbers: Vec<Vec<u8>> = input.split('\n').map(|l| l.bytes().map(|c| c - b'0').collect()).collect();

    let mut column_sums = vec![0u32; numbers[0].len()];
    for number in &numbers {
        for (&n, sum) in number.iter().zip(column_sums.iter_mut()) {
            *sum += n as u32;
        }
    }

    let half = (numbers.len() / 2) as u32;

    let mut gamma = 0;
    let mut epsilon = 0;
    for &sum in &column_sums {
        gamma <<= 1;
        epsilon <<= 1;
        if sum > half {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    info!(day=3, part=1, gamma=gamma, epsilon=epsilon, answer=gamma*epsilon);

    let mut oxygen_numbers = numbers.clone();
    for position in 0..numbers[0].len() {
        let sum: u32 = oxygen_numbers.iter().map(|n| n[position] as u32).sum();
        let keep = if sum * 2 >= oxygen_numbers.len() as u32 {
            1
        } else {
            0
        };
        debug!("in position {}: sum {} of {} so keeping {}", position, sum, oxygen_numbers.len(), keep);
        oxygen_numbers.retain(|n| n[position] == keep);
        debug!("now we have {:?}", oxygen_numbers);
        if oxygen_numbers.len() == 1 {
            break;
        }
    }

    let mut co2_numbers = numbers.clone();
    for position in 0..numbers[0].len() {
        let sum: u32 = co2_numbers.iter().map(|n| n[position] as u32).sum();
        let keep = if sum * 2 >= co2_numbers.len() as u32 {
            0
        } else {
            1
        };
        co2_numbers.retain(|n| n[position] == keep);
        if co2_numbers.len() == 1 {
            break;
        }
    }

    let mut oxygen: u32 = 0;
    let mut co2: u32 = 0;
    for position in 0..numbers[0].len() {
        oxygen <<= 1;
        co2 <<= 1;
        oxygen |= oxygen_numbers[0][position] as u32;
        co2 |= co2_numbers[0][position] as u32;
    }

    info!(day=3, part=2, oxygen=oxygen, co2=co2, answer=oxygen*co2);

    Ok(())
}
