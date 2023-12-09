fn main() {
    let input = include_str!("input.txt").trim().split_once('\n').unwrap();

    let times = input
        .0
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());
    let records = input
        .1
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());

    let res = times
        .zip(records)
        .map(|(time, record)| {
            // Within a time limit l, the distance traveled, where p <= l is the time of release of the button,
            // is p(l - p) or lp - p^2. If the record distance is r, lp - p^2 = r means
            // p^2 - lp + r = 0, and so p = (-l ± sqrt(l^2 - 4r))/2.
            // We are only trying to find the range of solutions or p2-p1
            let l = time as f64;
            let r = record as f64;
            let p_big = (l + (l.powf(2.0) - (4.0 * r)).sqrt()) / 2.0;
            let p_small = (l - (l.powf(2.0) - (4.0 * r)).sqrt()) / 2.0;

            // If the record thresholds land on a whole millisecond, we can't use that interval
            // and have to consider the adjacent qualifying times. Else just round towards the center.
            // We only do this for p_big because chances are, if one root is whole, both are,
            // and so you'd have (p_big - 1) - (p_small + 1) + 1
            // = (p_big - 1) - p_small
            // Otherwise, floor(p_big) - ceil(p_small) + 1 = floor(p_big) - floor(p_small)
            let i_big = if p_big % 10.0 < 0.0001 {
                p_big as u64 - 1
            } else {
                p_big as u64
            };
            let i_small = p_small.floor() as u64;
            i_big - i_small
        })
        .product::<u64>();

    dbg!(res);
}
