
pub fn exec1(s: String) {
    println!("Taked onwership string: {}", s);
}

pub fn exec2(s: &String) {
    println!("Borrowed string: {}", s);
}

pub fn exec3(vector: Vec<i32>) -> usize {
    return vector.len();
}

pub fn exec4(vector: &Vec<i32>) -> Option<&i32> {
    return vector.first();
}

pub fn exec5(tuple: (i32,i32)) -> i32 {
    let (a,_) = tuple;
    return a;
}

pub fn exec6(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &n in slice {
        sum += n;
    }
    return sum
}

pub fn exec7(word: String) -> usize {
    return word.len()
}

pub fn exec8(word: &str) -> Option<char> {
    return word.chars().next();
}

pub fn exec9(old_vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for &n in old_vec {
        new_vec.push(n * 2)
    }
    return new_vec
}

pub fn exec10(num1: &i32, num2: &i32) -> i32 {
    return *num1 + *num2;
}

fn main() {
    let s1 = String::from("Hello, world!");
    exec1(s1);

    let s2 = String::from("Hello, world!");
    exec2(&s2);

    let vec = vec![1,2];
    println!("Vec ownership tooked lenght: {}", exec3(vec));

    let vec2 = vec![1,2];
    match exec4(&vec2) {
        Some(first_element) => {
            println!("Vec Borrowed first element: {}", first_element);
        },
        None => {
            println!("The vector is empty.");
        }
    }

    let tuple = (1,2);
    println!("Tuple ownership tooked element: {}", exec5(tuple));

    let slice: &[i32] = &[1,2,3,4,5];
    println!("Sum of slice: {}", exec6(slice));

    let s2 = String::from("Hello, world!");
    println!("lenght of a string: {}", exec7(s2));

    let s8 = "Hello, world!";

    match exec8(&s8) {
        Some(first_element) => {
            println!("first element of a string: {}", first_element);
        },
        None => {
            println!("The vector is empty.");
        }
    }

    let vec9: Vec<i32> = vec![1,2];
    println!("Vec double: {:?}", exec9(&vec9));

    let num1: i32 = 8;
    let num2: i32 = 7;
    println!("sum of {} and {} is {}", num1, num2, exec10(&num1, &num2));
}
