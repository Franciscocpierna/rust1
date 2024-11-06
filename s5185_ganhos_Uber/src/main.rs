#[derive(Debug, PartialEq, Eq)]
struct Passenger {
    start: i32,
    end: i32,
    earn: i64,
}

impl Passenger {
    pub fn new(start: i32, end: i32, tip: i32) -> Self {
        Self {
            start,
            end,
            earn: (tip + end - start) as i64,
        }
    }
}

impl PartialOrd for Passenger {
    fn partial_cmp(&self, other: &Passenger) -> Option<std::cmp::Ordering> {
        self.end.partial_cmp(&other.end)
    }
}

impl Ord for Passenger {
    fn cmp(&self, other: &Passenger) -> std::cmp::Ordering {
        self.end.cmp(&other.end)
    }
}


pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
    let mut passengers: Vec<Passenger> = rides
        .iter()
        .map(|r| Passenger::new(r[0], r[1], r[2]))
        .collect();
    passengers.sort_unstable();
    let mut dp = vec![0; passengers.len() + 1];
    for i in 0..passengers.len() {
        let j = passengers.partition_point(|p| p.end <= passengers[i].start);
        dp[i + 1] = dp[i].max(dp[j] + passengers[i].earn);
    }
    dp[dp.len() - 1]
}


fn main() {
    let n1 = 5;
    let rides1 = vec![
        vec![2, 5, 4],
        vec![1, 5, 1],
    ];
    let resultado1 = max_taxi_earnings(n1, rides1);
    println!("Saída para Exemplo 1: {}", resultado1); // Saída: 7

    let n2 = 20;
    let rides2 = vec![
        vec![1, 6, 1],
        vec![3, 10, 2],
        vec![10, 12, 3],
        vec![11, 12, 2],
        vec![12, 15, 2],
        vec![13, 18, 1],
    ];
    let resultado2 = max_taxi_earnings(n2, rides2);
    println!("Saída para Exemplo 2: {}", resultado2); // Saída: 20
}
