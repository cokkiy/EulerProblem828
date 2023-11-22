use std::rc::Rc;

use apr::{resolve, Operand};

fn main() {
    println!("欢迎使用任意数解算程序！");
    println!("本程序能够根据任意一组数据通过 +、－、×、÷ 运算得到给定的数值！");
    println!("如给定24和一组数1，2，3，4，可解算出满足要求的所有算式：");
    println!("(1 + 2 + 3) * 4 = 24");
    println!("1 * 2 * 3 * 4 = 24");
    println!("1 * 2 * 4 * 3 = 24");
    println!("3 * 4 * 1 * 2 = 24");
    println!("....");

    'outer: loop {
        println!("请按照提示输入数据，输入x或q退出，输入完成后按回车键继续。");
        'inner: loop {
            println!("请输入要解算的数值，输入完成后按回车键继续：");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "x" || input.trim() == "q" {
                return;
            }

            let expect_result = input.trim().parse::<i32>();
            if expect_result.is_err() {
                println!("输入的数值不合法，请重新输入！");
                continue 'inner;
            } else {
                'array: loop {
                    println!("请输空格分隔的一组数，输入完成后按回车键继续：");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if input.trim() == "x" || input.trim() == "q" {
                        return;
                    }
                    let expect_result = expect_result.as_ref().unwrap();
                    let mut numbers = vec![];
                    for number in input.trim().split_whitespace() {
                        let result = number.parse::<i32>();
                        if result.is_err() {
                            println!("输入的数据不合法，请重新输入！");
                            continue 'array;
                        } else {
                            numbers.push(Rc::new(Operand::Number(result.unwrap())));
                        }
                    }

                    let mut results_steps = vec![];
                    resolve(*expect_result, &numbers, &mut results_steps);
                    if results_steps.len() == 0 {
                        println!("没有找到解算结果！");
                        continue 'outer;
                    }
                    for (index, result) in results_steps.iter().enumerate() {
                        println!("{}): {}", index + 1, result);
                    }
                    continue 'outer;
                }
            }
        }
    }
}
