use std::io;
use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line 1");
    let total_cases: u32 = line.trim().parse().expect("not a number.");
    for case in 1..total_cases+1 {
        line.truncate(0);
        io::stdin().read_line(&mut line).expect("failed to read line");
        let depth: u32 = line.trim().parse().expect("Not a number.");
        let mut soldier_groups = HashSet::new();
        for _ in 0..2*depth {
            line.truncate(0);
            io::stdin().read_line(&mut line).expect("failed to read line");
            let soldier_group: Vec<u32> = line.trim().split(' ').filter_map(|s| s.parse().ok()).collect();
            soldier_groups.insert(soldier_group);
        }
        println!("{:?}", soldier_groups);
        //println!("Case #{}: {}", case, max);
    }
}

fn bffs(students: u32, bff_ids: &[u32]) -> u32 {
    let mut bff_list = HashSet::new();
    for id in 1..bff_ids.len()+1 {
        bff_list.insert(vec![id, bff_ids[id-1] as usize]);
    }
    println!("{} {:?}", students, bff_list);
    return 0;
}

