use crate::{split_task, split_task_length};
use std::fmt::Debug;

#[test]
fn test_split_task_len() {
    let res = split_task_length(4, 3);
    let cor = vec![2, 1, 1];
    assert_eq!(res, cor);
    let res = split_task_length(8, 3);
    let cor = vec![3, 3, 2];
    assert_eq!(res, cor);
    let res = split_task_length(7, 2);
    let cor = vec![4, 3];
    assert_eq!(res, cor);
    let res = split_task_length(7, 4);
    let cor = vec![2, 2, 2, 1];
    assert_eq!(res, cor);
    let res = split_task_length(3, 5);
    let cor = vec![1, 1, 1, 0, 0];
    assert_eq!(res, cor);
}

#[test]
fn test_split_task() {
    //let mut a = vec![1; 8];
    let mut a = vec![1; 7];
    let res = split_task(&mut a, 4);
    let parts = ([1, 1], [1, 1], [1, 1], [1]);
    let cor = vec![
        parts.0.as_slice(),
        parts.1.as_slice(),
        parts.2.as_slice(),
        parts.3.as_slice(),
    ];
    print_result(&res);
    assert_eq!(res, cor);

    #[derive(Clone, Debug, PartialEq)]
    struct B {}
    let mut b = vec![B {}; 2];
    let res = split_task(&mut b, 5);
    let parts = ([B {}], [B {}], [B {}; 0], [B {}; 0], [B {}; 0]);
    let cor = vec![
        &parts.0[..],
        &parts.1[..],
        &parts.2[..],
        &parts.3[..],
        &parts.4[..],
    ];
    print_result(&res);
    assert_eq!(res, cor)
}

fn print_result<T: Debug>(res: &Vec<&mut [T]>) {
    for i in 0..res.len() {
        print!("[");
        for j in 0..res[i].len() {
            print!("{:?}", res[i][j]);
            if j < res[i].len() - 1 {
                print!(", ")
            }
        }
        print!("] ")
    }
    println!()
}
