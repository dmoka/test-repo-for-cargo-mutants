use std::fmt;

#[cfg(test)]
use mutagen::mutate;


pub struct Repository {
    pub data: Vec<String>
}

impl Repository {

    pub fn new() -> Self {
        Repository{
            data: Vec::<String>::new()
        }
    }

    pub fn store(&mut self, data_entry: &str) {
        self.data.push(data_entry.to_string());
    }

}

pub struct Main {}

impl Main {

    #[cfg_attr(test, mutate)]
    pub fn do_logic(repo: &mut Repository, mut number: u32) -> u32 {
        number = number + 5;

        repo.store("data from do_logic");

        number = number + 10;

        number
    }

    #[cfg_attr(test, mutate)]
    pub fn do_logic2(repo: &mut Repository, mut number: u32) -> u32 {
        number = number + 5;

        repo.store("data from do_logic2");

        number = number + 20;

        number
    }
}

fn main() {
}


#[test]
fn do_logic_should_increment_number_with_15() {
    //Arrange
    let mut repo = Repository{
        data: Vec::<String>::new()
    };

    let mut number: u32 = 5;

    //Act
    let modified_number = Main::do_logic(&mut repo, number);

    //Assert
    assert_eq!(modified_number, number + 15);
}


#[test]
fn do_logic_should_store_data() {
    //Arrange
    let mut repo = Repository{
        data: Vec::<String>::new()
    };

    let mut number: u32 = 5;

    //Act
    let modified_number = Main::do_logic(&mut repo, number);

    //Assert
    assert_eq!(&repo.data[0], "data from do_logic");
}


#[test]
fn do_logic2_should_increment_number_with_25() {
    //Arrange
    let mut repo = Repository{
        data: Vec::<String>::new()
    };

    let mut number: u32 = 5;

    //Act
    let modified_number = Main::do_logic2(&mut repo, number);

    //Assert
    assert_eq!(modified_number, number + 25);
}