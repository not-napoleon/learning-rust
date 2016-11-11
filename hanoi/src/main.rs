struct NamedVec {
    name: char,
    stack: Vec<i32>,
}

fn move_disk(mut from: NamedVec, mut to: NamedVec) {
    let disk = from.stack.pop();
    if to.stack.len() > 0 {
        if to.stack[to.stack.len() - 1] <= disk {
            panic!("Illegal move: disk {} from {} to {}", disk, from.name,
                   to.name)
        }
    }
    to.stack.push(disk);
    println!("Move disk {} from {} to {}", disk, from.name, to.name);
}

fn hanoi(depth: i32, mut source: NamedVec, mut dest: NamedVec,
          swap: NamedVec) {
    if depth == 1 {
        move_disk(source, dest);
    } else {
        hanoi(depth - 1, source, swap, dest);
        move_disk(source, dest);
        hanoi(depth - 1, swap, dest, source);
    }
}

fn main() {
    let source = NamedVec{name: 'a', stack: vec![3, 2, 1]};
    let dest = NamedVec{name: 'b', stack: vec![]};
    let swap = NamedVec{name: 'c', stack: vec![]};

    hanoi(3, source, dest, swap);
}
