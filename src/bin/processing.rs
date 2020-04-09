// Advent of Code 2019 - Day 1 solution.

use std::env;

fn calculate_fuel(mass: &i32) -> i32 {
    ((*mass as f32 / 3.0).floor() - 2.0) as i32
}

fn calculate_fuel_advanced(mass: &i32) -> i32 {
    let mut total_fuel = calculate_fuel(mass);
    let mut new_fuel = calculate_fuel(&total_fuel);

    while new_fuel > 0 {
        total_fuel += new_fuel;

        new_fuel = i32::max(0, calculate_fuel(&new_fuel));
    }

    total_fuel
}

fn main() {
    let inputs = vec![
        147383, 111288, 130868, 140148, 79840, 63305, 98475, 66403, 68753, 136306, 94135, 51317,
        136151, 71724, 68795, 68526, 130515, 73606, 56828, 57778, 86134, 105030, 123367, 97633,
        85043, 110888, 110785, 90662, 128865, 70997, 90658, 79944, 141089, 67543, 78358, 143579,
        146971, 78795, 94097, 82473, 73216, 50919, 100248, 112751, 86227, 117399, 123833, 148570,
        141464, 123266, 94346, 53871, 51180, 112900, 119863, 106694, 129841, 75990, 63509, 50135,
        140081, 138387, 112697, 57023, 114256, 81429, 95573, 57056, 52277, 75137, 53364, 125823,
        113227, 93993, 129808, 114025, 101677, 127114, 65823, 65834, 57955, 102314, 60656, 89982,
        61068, 72089, 71745, 72460, 142318, 91951, 111759, 61177, 143739, 92202, 70168, 80164,
        77867, 64235, 141137, 102636,
    ];

    println!(
        "{}",
        inputs
            .iter()
            .map(calculate_fuel)
            .into_iter()
            .sum::<i32>()
    );

    println!(
        "{}",
        inputs
            .iter()
            .map(calculate_fuel_advanced)
            .into_iter()
            .sum::<i32>()
    );
}
