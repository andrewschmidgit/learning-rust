struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping pointer with data: `{}`!", self.data);
    }
}

pub fn run() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("Pointers created");
    drop(c);
    println!("Pointer dropped before end of main");
}
