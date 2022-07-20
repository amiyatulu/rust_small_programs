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
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

fn mean_integer(data: &Vec<i64>) -> Option<i64> {
    let data_mul_sum = data.iter().sum::<i64>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(data_mul_sum / count as i64),
        _ => None,
    }
}

fn std_deviation_interger(data: &Vec<i64>) -> Option<i64> {
    match (mean_integer(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean.checked_sub(*value as i64).unwrap();
                    diff * diff
                })
                .sum::<i64>()
                / count as i64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

// The values less than one standard deviation away from the mean account for 68.27% of the set
// So we calculate mean of this 68.27% of data removing outlier
fn calculate_new_mean(data: &Vec<i64>, mean: Option<i64>, sd: Option<i64>) -> Option<i64> {
    let mut new_items = vec![];
    for x in data {
        if *x >= mean.unwrap().checked_sub(sd.unwrap()).unwrap()
            && *x <= mean.unwrap().checked_add(sd.unwrap()).unwrap()
        {
            new_items.push(*x);
        }
    }

    // println!("new items {:?}", new_items);
    let new_mean = mean_integer(&new_items);
    new_mean
}

fn main() {
    let data = vec![-10, 1, 1, 1, 5, 1, 1, 7];
    let data_mean = mean(&data);
    println!("Mean is {:?}", data_mean); 
    let data_std_deviation = std_deviation(&data);
    println!("Standard deviation is {:?}", data_std_deviation); 
    
    // let data_integer = vec![3000, 1000, 6000, 1000, 5000, 8000, 1000, 8000, 10000];
    let data_initial = vec![-10, 1, 1, 1, 5, 1, 1, 7];
    let data_integer = data_initial
        .into_iter()
        .map(|x| x * 1000)
        .collect::<Vec<i64>>();
    println!("data integer {:?}", data_integer);
    let data_mean_integer = mean_integer(&data_integer);
    println!("Mean integer is {:?}", data_mean_integer); 

   

    let data_std_deviation_integer = std_deviation_interger(&data_integer);
    println!(
        "Standard deviation integer is {:?}",
        data_std_deviation_integer
    ); 

    let new_mean_integer = calculate_new_mean(&data_integer, data_mean_integer, data_std_deviation_integer);
    println!("new mean {:?}", new_mean_integer)

}
