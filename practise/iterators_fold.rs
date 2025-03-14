

fn sol_bal_total(transactions: Vec<i32>) -> i32 {
    let sum_bal = transactions.iter().fold(0,|tot, x| tot+x);
    sum_bal
}

fn main(){
    let transact = vec![23,45,26,75,90];
    let total = sol_bal_total(transact);
    println!("total: {}",total);
}