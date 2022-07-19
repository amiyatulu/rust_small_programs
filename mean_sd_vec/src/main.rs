use num_integer::Roots;

fn mean(data: &Vec<i32>) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &Vec<i32>) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}

fn mean_integer(data: &Vec<i64>) -> Option<i64> {
    let data_mul_sum = data.iter().sum::<i64>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(data_mul_sum  / count as i64),
        _ => None,
    }
}


fn std_deviation_interger(data: &Vec<i64>) -> Option<i64> {
    match (mean_integer(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean.checked_sub(*value as i64).unwrap();
                diff * diff
            }).sum::<i64>() / count as i64;

            Some(variance.sqrt())
        },
        _ => None
    }
}







fn main() {
    let data = vec![3, 1, 6, 1, 5, 8, 1, 8, 10];
    let data_mean = mean(&data);
    println!("Mean is {:?}", data_mean); // Mean is Some(4.7777777)
    // let data_integer = vec![3000, 1000, 6000, 1000, 5000, 8000, 1000, 8000, 10000];
    let data_initial = vec![3, 1, 6, 1, 5, 8, 1, 8, 10];
    let data_integer = data_initial.into_iter().map(|x| x*1000).collect::<Vec<i64>>();
    let data_mean_integer = mean_integer(&data_integer);
    println!("Mean integer is {:?}", data_mean_integer); // Mean integer is Some(4777)

    let data_std_deviation = std_deviation(&data);
    println!("Standard deviation is {:?}", data_std_deviation); // Standard deviation is Some(3.2584174)

    let data_std_deviation_integer = std_deviation_interger(&data_integer);
    println!("Standard deviation integer is {:?}", data_std_deviation_integer); // Standard deviation integer is Some(3258)

}
