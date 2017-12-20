use std::io;
use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line 1");
    let total_cases: usize = line.trim().parse().expect("not a number.");
    for case in 1..total_cases+1 {
        line.truncate(0);
        io::stdin().read_line(&mut line).expect("failed to read line");
        let total_students: usize = line.trim().parse().expect("Not a number.");
        line.truncate(0);
        io::stdin().read_line(&mut line).expect("failed to read line");
        let bff_ids: Vec<usize> = line.trim().split(' ').filter_map(|s| s.parse().ok()).collect();
        let max = bffs(total_students, bff_ids.as_slice());
        println!("Case #{}: {}", case, max);
    }
}

fn bffs(students: usize, bff_ids: &[usize]) -> usize {
    let mut bff_list = HashSet::new();
    for id in 1..bff_ids.len()+1 {
        bff_list.insert(vec![id, bff_ids[id-1]]);
    }
    let mut shared_bffs: HashSet<usize> = HashSet::new();
    for id in 1..bff_ids.len()+1 {
        if shared_bffs.contains(&bff_ids[id-1]) { continue; }
        if bff_ids[bff_ids[id-1]-1] == id {
            shared_bffs.insert(id);
        }
    }
    println!("{} {:?}", students, shared_bffs);
    return 0;
}

