struct NamedVec {
    name: char,
    stack: Vec<i32>,
}

fn move_disk(from: &mut NamedVec, to: &mut NamedVec) {
    let disk = from.stack.pop();
    // Deal with the case where we pop from an empty vector
    if disk == None {
        panic!("Attempt to move from an empty stack");
    }
    // Turn the Optional<i32> into a plain i32; disk here shadows the earlier usage
    let disk = disk.unwrap();
    if to.stack.len() > 0 {
        if to.stack[to.stack.len() - 1] <= disk {
            panic!("Illegal move: disk {} from {} to {}", disk, from.name,
                   to.name)
        }
    }
    to.stack.push(disk);
    println!("Move disk {} from {} to {}", disk, from.name, to.name);
}

fn hanoi(depth: i32, source: &mut NamedVec, dest: &mut NamedVec, swap: &mut NamedVec) {
    if depth == 1 {
        move_disk(source, dest);
    } else {
        hanoi(depth - 1, source, swap, dest);
        move_disk(source, dest);
        hanoi(depth - 1, swap, dest, source);
    }
}

fn main() {
    let mut source = NamedVec{name: 'a', stack: vec![3, 2, 1]};
    let mut dest = NamedVec{name: 'b', stack: vec![]};
    let mut swap = NamedVec{name: 'c', stack: vec![]};

    hanoi(3, &mut source, &mut dest, &mut swap);
}
