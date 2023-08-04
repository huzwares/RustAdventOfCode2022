use std::collections::BTreeMap;

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    inspected: usize,
    operation: String,
    test: Test,
}

#[derive(Clone)]
struct Test {
    divisible: usize,
    if_true: String,
    if_false: String,
}

impl Monkey {
    fn new(items: Vec<usize>, operation: String, test: Test) -> Monkey {
        Monkey {
            items,
            inspected: 0,
            operation,
            test,
        }
    }

    fn parse_operation(&self) -> Box<dyn Fn(usize) -> usize + 'static> {
        let (_, op) = self.operation.trim().split_at(21);
        let op = op.split(' ').collect::<Vec<&str>>();
        match op[0] {
            "*" => match op[1] {
                "old" => Box::new(|old: usize| old * old),
                num => {
                    let num = num.parse::<usize>().unwrap();
                    Box::new(move |old: usize| old * num)
                }
            },
            "+" => match op[1] {
                "old" => Box::new(|old: usize| old + old),
                num => {
                    let num = num.parse::<usize>().unwrap();
                    Box::new(move |old: usize| old + num)
                }
            },
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = input.lines();
    let mut monkeys: BTreeMap<String, Monkey> = BTreeMap::new();
    while let (Some(name), Some(items), Some(op), Some(test), Some(if_true), Some(if_false)) = (
        input.next(),
        input.next(),
        input.next(),
        input.next(),
        input.next(),
        input.next(),
    ) {
        let mut name = name.trim().split(' ').collect::<Vec<&str>>().join("_");
        name.pop();
        monkeys.entry(name).or_insert_with(|| {
            let (_, items) = items.trim().split_at(16);
            let items = items
                .split(", ")
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let operation = op.to_owned();
            let divisible = test
                .trim()
                .chars()
                .filter(|ch| ch.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let if_true = format!("Monkey_{}", if_true.chars().last().unwrap());
            let if_false = format!("Monkey_{}", if_false.chars().last().unwrap());
            let test = Test {
                divisible,
                if_true,
                if_false,
            };
            Monkey::new(items, operation, test)
        });
        input.next();
    }
    let mut monkeys_p2 = monkeys.clone();

    // Part 1
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let name = format!("Monkey_{}", i);
            let mut monkey = monkeys.remove(&name).unwrap();
            let op = monkey.parse_operation();
            monkey.items.reverse();
            while let Some(w_lvl) = monkey.items.pop() {
                monkey.inspected += 1;
                let w_lvl = op(w_lvl) / 3;
                if w_lvl % monkey.test.divisible == 0 {
                    if let Some(m) = monkeys.get_mut(&monkey.test.if_true) {
                        m.items.push(w_lvl);
                    }
                } else if let Some(m) = monkeys.get_mut(&monkey.test.if_false) {
                    m.items.push(w_lvl);
                }
            }
            monkeys.insert(name, monkey);
        }
    }
    println!("Part 1: {}", calculate_business(monkeys));

    // Part 2
    let cm: usize = monkeys_p2
        .values()
        .map(|monkey| monkey.test.divisible)
        .product();
    for _ in 0..10000 {
        for i in 0..monkeys_p2.len() {
            let name = format!("Monkey_{}", i);
            let mut monkey = monkeys_p2.remove(&name).unwrap();
            let op = monkey.parse_operation();
            monkey.items.reverse();
            while let Some(w_lvl) = monkey.items.pop() {
                monkey.inspected += 1;
                let w_lvl = op(w_lvl) % cm;
                if w_lvl % monkey.test.divisible == 0 {
                    if let Some(m) = monkeys_p2.get_mut(&monkey.test.if_true) {
                        m.items.push(w_lvl);
                    }
                } else if let Some(m) = monkeys_p2.get_mut(&monkey.test.if_false) {
                    m.items.push(w_lvl);
                }
            }
            monkeys_p2.insert(name, monkey);
        }
    }
    println!("Part 2: {}", calculate_business(monkeys_p2));
}

fn calculate_business(monkeys: BTreeMap<String, Monkey>) -> usize {
    let mut insp = monkeys
        .values()
        .map(|v| v.inspected)
        .collect::<Vec<usize>>();
    insp.sort();
    insp.iter().rev().take(2).product()
}
