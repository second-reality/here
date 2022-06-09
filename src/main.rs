slint::slint! {
    HelloWorld := Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
    HelloWorld::new().run();
}
