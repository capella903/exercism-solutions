pub fn is_armstrong_number(num: u32) -> bool {
    let ns = disassembly(num);
    ns.iter().map(|n| n.pow(ns.len() as u32)).sum::<u32>() == num
}

fn disassembly(n: u32) -> Vec<u32> {
    let q = n / 10;
    if q == 0 {
        vec![n]
    } else {
        let mut ns = disassembly(q);
        ns.push(n % 10);
        ns
    }
}
