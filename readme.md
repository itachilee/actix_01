# this is a project for learning actix lib

## features

- route

- log

- database

- json

## libraries

    [dependencies]
    actix-web = "4"
    serde_json = "1"
    futures = "0.3"
    serde = { version = "1.0", features = ["derive"] }
    actix-multipart = "0.6.0"
    actix-files = "0.6.0"
    derive_more = "0.99"
    env_logger = "0.10"
    utoipa-swagger-ui = { version = "5", features = ["actix-web"] }
    utoipa = "4.1"
    log = "0.4.0"
    chrono = "0.4"

## Auto-Reloading Development Server

During development it can be very handy to have cargo automatically recompile the code on changes. This can be accomplished very easily by using cargo-watch.

    cargo watch -x run

Historical Note
An old version of this page recommended using a combination of systemfd and listenfd, but this has many gotchas and was difficult to integrate properly, especially when part of a broader development workflow. We consider cargo-watch to be sufficient for auto-reloading purposes.

or use the following

Run server with auto-reloading:

    cargo install systemfd cargo-watch
    systemfd --no-pid -s http::8000 -- cargo watch -x run

---

## 通用惯例

1. 使用借用类型作为参数
    当你决定为一个函数参数使用哪种参数类型时，使用解引用强制转换的目标可以增加你代码的灵活性。 通过这种方式，该函数将接受更多的输入类型。

    这并不限于可切片或胖指针类型。 事实上，你应该总是倾向于使用借用类型而不是借用所有类型。 例如&str而不是&String，&[T]而不是&Vec<T>，以及&T而不是&Box<T>。

    使用借用类型，你可以避免已经提供一层间接性的所有类型上的多层间接。例如，String有一层间接，所以&String会有两层间接。我们可以通过使用&str来避免这种情况，并且让&String在函数被调用时强制变成&str。

---

2. 用format!串联字符串

---

3. Default Trait

<code lang="rust">
    use std::{path::PathBuf, time::Duration};

    // note that we can simply auto-derive Default here.
    #[derive(Default, Debug, PartialEq)]
    struct MyConfiguration {
        // Option defaults to None
        output: Option<PathBuf>,
        // Vecs default to empty vector
        search_path: Vec<PathBuf>,
        // Duration defaults to zero time
        timeout: Duration,
        // bool defaults to false
        check: bool,
    }

    impl MyConfiguration {
        // add setters here
    }

    fn main() {
        // construct a new instance with default values
        let mut conf = MyConfiguration::default();
        // do something with conf here
        conf.check = true;
        println!("conf = {:#?}", conf);
        // partial initialization with default values, creates the same instance
        let conf1 = MyConfiguration {
            check: true,
            ..Default::default()
        };
        assert_eq!(conf, conf1);
    }
</code>

4. 集合是智能指针
