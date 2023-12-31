use factorial::Factorial;

#[allow(unused)]
fn brute_force() -> (i64, i64) {
    let input = include_str!("../input/9.txt");
    let p1 = input
        .lines()
        .map(|l| {
            let mut ns: Vec<i64> = l.split(' ').map(|s| s.parse().unwrap()).collect();
            let mut lasts = Vec::new();
            while !ns.iter().all(|n| *n == 0) {
                lasts.push(*ns.last().unwrap());
                let next: Vec<_> = ns.windows(2).map(|w| w[1] - w[0]).collect();
                ns = next;
            }
            lasts.iter().sum::<i64>()
        })
        .sum::<i64>();
    let p2 = input
        .lines()
        .map(|l| {
            let mut ns: Vec<i64> = l.split(' ').map(|s| s.parse().unwrap()).collect();
            let mut firsts = Vec::new();
            while !ns.iter().all(|n| *n == 0) {
                firsts.push(*ns.first().unwrap());
                let next: Vec<_> = ns.windows(2).map(|w| w[1] - w[0]).collect();
                ns = next;
            }
            firsts.iter().rev().fold(0, |acc, v| v - acc)
        })
        .sum();
    (p1, p2)
}

pub fn soln() -> (i64, i64) {
    let input = include_str!("../input/9.txt");

    // Use Lagrange interpolation:
    // P(x) = sum(P_j(x) for x in 0..n), where n is the number of points.
    // Since the number of points `n` is fixed in our input, we can determine the coefficients
    // in each P_j for the `n` points.
    // We make the assumption that for the y_j points given in the inputs, x_i == i for i in 0..n.
    let coefficients = {
        // n == number of points in each line.
        // u128 is necessary for n, j since factorial will otherwise overflow u64.
        let n = input.split_once('\n').unwrap().0.split(' ').count() as u128;
        // P_j (x) = y_j * coefficient, where
        // coefficient = prod((x - x_k) / (x_j - x_k), for k in 0..n where k != j)
        // Since we want to P(n) (i.e. y for the next x), x == n.
        // Calculate a vector of coefficients for [P_0, .., P_(n-1)]
        (0..n)
            .map(|j| {
                // This is a non-standard derivation of Lagrange. It is specialized for consecutive
                // integral x points.
                let n_choose_j = n.factorial() / ((n - j).factorial() * j.factorial());
                n_choose_j as i64 * (-1i64).pow((n - 1 - j) as u32)
            })
            .collect::<Vec<_>>()
    };
    let p1 = input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .zip(coefficients.iter())
                .map(|(x, y)| x * y)
                .sum::<i64>()
        })
        .sum();
    let p2 = input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .rev() // Reverse the inputs so that the 'next' point is the point before the first input
                .zip(coefficients.iter())
                .map(|(x, y)| x * y)
                .sum::<i64>()
        })
        .sum();

    (p1, p2)
}
