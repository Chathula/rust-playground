use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Server {
    host: String,
    port: i32,
    tls: bool
}

impl Server {
    pub fn new(host: String, port: i32) -> ServerBuilder {
        ServerBuilder { host, port, tls: false }
    }
}

pub struct ServerBuilder {
    host: String,
    port: i32,
    tls: bool
}

impl ServerBuilder {
    pub fn tls(&mut self, tls: bool) -> &mut Self {
        self.tls = tls;
        self
    }

    pub fn build(&self) -> Server {
        Server {
            host: self.host.clone(),
            port: self.port,
            tls: self.tls
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    value: RefCell<i32>,
    nodes: Vec<&'a Node<'a>>,
}

fn update_node(node: &Node) {
    let mut currunt_value = node.value.borrow_mut();
    *currunt_value += 2;

    for n in node.nodes.iter() {
        update_node(&n);
    }
}

fn binary_search(list: &Vec<i32>, target: i32, start: usize, end: usize) -> Option<usize> {
    if start > end {
        return None
    }

    let mid = (start + end) / 2;

    if list[mid] == target {
        Some(mid)
    } else if list[mid] < target {
        binary_search(&list, target, mid + 1, end)
    } else {
        binary_search(&list, target, start, mid - 1)
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let names = vec!["Alice", "John", "Peter"];
    let ages = vec![10, 20, 30];
    
    let total = numbers.iter().map(|n| n * n).collect::<Vec<i32>>();

    let even_numbers = numbers.iter().filter(|n| *n % 2 == 0).collect::<Vec<&i32>>();

    let fold_value = numbers.iter().fold(1, |acc, n| acc * n);

    let chain_numbers = numbers.iter().chain(ages.iter()).collect::<Vec<&i32>>();

    let numbers_window = numbers.windows(2).collect::<Vec<&[i32]>>();

    let cycle_numbers = numbers.iter().cycle().take(25).collect::<Vec<&i32>>();

    println!("Total: {:?}", total);
    println!("Even numbers: {:?}", even_numbers);
    println!("Fold value: {}", fold_value);
    println!("Chain numbers: {:?}", chain_numbers);
    println!("Numbers window: {:?}", numbers_window);
    println!("Cycle numbers: {:?}", cycle_numbers);

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }

    for group in numbers.chunks(3) {
        println!("Group: {:?}", group)
    }

    if ages.iter().all(|n| n % 2 == 0) {
        println!("All numbers are even");
    } else {
        println!("Not all numbers are even");
    }

    if numbers.iter().any(|n| *n > 9) {
        println!("At least one number is greater than 9");
    } else {
        println!("No number is greater than 9");
    }


    let host: String = String::from("127.0.0.1");
    let port: i32 = 8080;

    let basic_server = Server::new(host.clone(), port).build();
    let tls_server = Server::new(host.clone(), port).tls(true).build();

    println!("{:?}", basic_server);
    println!("{:?}", tls_server);

    let a = Node {
        value: RefCell::new(1),
        nodes: vec![],
    };

    let b = Node {
        value: RefCell::new(2),
        nodes: vec![&a],
    };

    let c = Node {
        value: RefCell::new(3),
        nodes: vec![&a],
    };

    update_node(&a);
    update_node(&b);
    update_node(&c);

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    let numbers_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let target = 8;

    let result = binary_search(&numbers_list, target, 0, numbers_list.len() - 1);

    match result {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the list", target),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_basic_server_without_tls() {
        let host: String = String::from("127.0.0.1");
        let port: i32 = 8080;

        let server = Server::new(host.clone(), port).build();

        assert_eq!(server.host, host);
        assert_eq!(server.port, port);
        assert_eq!(server.tls, false);
    }

    #[test]
    fn it_should_create_server_with_tls() {
        let host = String::from("127.0.0.1");
        let port = 8080;

        let server = Server::new(host.clone(), port).tls(true).build();

        assert_eq!(server.host, host);
        assert_eq!(server.port, port);
        assert_eq!(server.tls, true);
    }

    #[test]
    fn it_should_return_target_index_when_found() {
        let numbers_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let target = 8;

        let result = binary_search(&numbers_list, target, 0, numbers_list.len() - 1);

        assert_eq!(result, Some(7));
    }

    #[test]
    fn it_should_return_none_when_target_not_found() {
        let numbers_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let target = 12;

        let result = binary_search(&numbers_list, target, 0, numbers_list.len() - 1);

        assert_eq!(result, None);
    }
}