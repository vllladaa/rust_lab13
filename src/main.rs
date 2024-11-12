fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None;
    }

    let average = (total / n) as i32;
    let mut moves = 0;
    let mut balance = 0;

    for &shipment in shipments {
        balance += shipment as i32 - average;
        moves += balance.abs() as usize;
    }

    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let average_load = 10;
    let mut shipments = vec![average_load; n];

    for i in 0..n {
        shipments[i] += (i % 3) as u32;
    }

    shipments
}

fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];

    match count_permutation(&example1) {
        Some(moves) => println!("Для вирівнювання вантажу в першому прикладі потрібно: {} операцій", moves),
        None => println!("Розподіл неможливий"),
    }

    match count_permutation(&example2) {
        Some(moves) => println!("Для вирівнювання вантажу в другому прикладі потрібно: {} операцій", moves),
        None => println!("Розподіл неможливий"),
    }

    let generated_shipments = gen_shipments(5);
    println!("Згенерований масив вантажів: {:?}", generated_shipments);
    match count_permutation(&generated_shipments) {
        Some(moves) => println!("Необхідні операції для вирівнювання: {}", moves),
        None => println!("Розподіл неможливий"),
    }
}

#[test]
fn test_count_permutation() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];
    assert_eq!(count_permutation(&example1), Some(4));
    assert_eq!(count_permutation(&example2), Some(7));
}

//Чи завжди можливо вирівняти вантажі на кораблях?
// Ні, рівномірне розподілення можливе тільки якщо загальна кількість вантажу
// ділиться на кількість кораблів без залишку.
// Як виглядатиме сигнатура в іншому випадку?
// Щоб врахувати випадки, коли вирівнювання неможливе, можна використовувати Option<usize>.
// У цьому випадку функція поверне None, якщо рівномірний розподіл неможливий.