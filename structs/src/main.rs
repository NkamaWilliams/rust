fn main() {
    let student1 = Student {
        name: String::from("Nkama Williams"),
        matric_no: String::from("190802304"),
        years_spent: 0,
    };

    let student2 = Student {
        name: String::from("Amaka Wallace"),
        matric_no: String::from("190802305"),
        ..student1
    };

    student1.view();
    student2.view();

    let student3 = Student::returnee(String::from("Desyo Lika"), String::from("190820303"), 4);

    student3.view();
}

// #[derive(Debug)]
struct Student {
    name: String,
    matric_no: String,
    years_spent: u32,
}
impl Student {
    fn view(&self) {
        println!("Name: {}", self.name);
        println!("Matric No.: {}", self.matric_no);
        println!("Level: {}", (self.years_spent + 1) * 100);
        println!();
    }

    fn returnee(name: String, matric_no: String, years_spent: u32) -> Self {
        Student {
            name,
            matric_no,
            years_spent,
        }
    }
}
