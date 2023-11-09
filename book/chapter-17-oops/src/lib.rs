pub mod collection {
    #[derive(Debug)]
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new() -> AveragedCollection {
            AveragedCollection {
                list: vec![],
                average: 0.0
            }
        }

        pub fn add(&mut self, item: i32) {
            self.list.push(item);
            self.calc_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let item = self.list.pop();
            match item {
                Some(item) => {
                    self.calc_average();
                    Some(item)
                },
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn calc_average(&mut self) {
            let sum: i32 = self.list.iter().sum();
            self.average = sum as f64 / self.list.len() as f64;
        }
    }

    impl Default for AveragedCollection {
        fn default() -> Self {
            Self::new()
        }
    }

    pub fn run() {
        println!("\nAveraged Collection");
        let mut list = AveragedCollection::new();
        list.add(1);
        println!("List: {:?}, Average: {}", list, list.average());
        list.add(2);
        println!("List: {:?}, Average: {}", list, list.average());
        list.add(3);
        println!("List: {:?}, Average: {}", list, list.average());
        list.add(4);
        println!("List: {:?}, Average: {}", list, list.average());
    }
}

pub mod graphics {
    trait Draw {
        fn draw(&self);
    }

    #[derive(Debug)]
    struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing a button: {:?}", self);
        }
    }

    #[derive(Debug)]
    struct Select {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for Select {
        fn draw(&self) {
            println!("Drawing a select: {:?}", self);
        }
    }

    struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    struct GenericScreen<T> {
        pub components: Vec<T>,
    }

    impl<T> GenericScreen<T>
    where 
        T: Draw, 
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub fn run() {
        // Static dispatch
        // Compiler generates non-generic code, so no runtime cost
        println!("\n 'Rendering Generic Screen'");
        let generic_screen = GenericScreen {
            components: vec![
                Button {
                    width: 10,
                    height: 10,
                    label: "Click Me!".into()
                }, 
                Button {
                    width: 20,
                    height: 20,
                    label: "Click Me!".into()
                }
            ],
        };

        generic_screen.run();

        // Dynamic dispatch
        // Compiler doesn't know all types, so there's a small runtime cost
        println!("\n 'Rendering Boxed Screen'");
        let screen = Screen {
            components: vec![
                Box::new(Button {
                    width: 10,
                    height: 10,
                    label: "Click Me!".into()
                }),
                Box::new(Select {
                    width: 100,
                    height: 40,
                    options: vec![
                        "Option 1".into(),
                        "Option 2".into(),
                        "Option 3".into(),
                    ]
                }),
            ]
        };

        screen.run();
    }
}

pub mod state_pattern_oops {
    trait State {
        fn can_add_text(&self) -> bool {
            false
        }
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview::new())
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn can_add_text(&self) -> bool {
            true
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct PendingReview {
        approvals: i32,
    }

    impl PendingReview {
        pub fn new() -> PendingReview {
            PendingReview { approvals: 0 }
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(mut self: Box<Self>) -> Box<dyn State> {
            self.approvals += 1;
            if self.approvals < 2 {
                self
            } else {
                Box::new(Published {})
            }
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct Post {
        state: Option<Box< dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            if self.state.as_ref().unwrap().can_add_text() {
                self.content.push_str(text);
            }
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    pub fn run() {
        println!("\nDemonstrating State Pattern (OOP)");

        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());

        post.add_text("More text after approval");
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

pub mod state_pattern_types {
    struct Post {
        content: String,
    }

    struct DraftPost {
        content: String,
    }

    struct PendingReviewPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new()
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost { content: self.content }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post { content: self.content }
        }
    }

    pub fn run() {
        println!("\nDemonstrating State Pattern with Types");

        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
