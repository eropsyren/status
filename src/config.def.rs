pub const separator: &'static str = ": ";

pub fn data<'a>() -> Vec<(&'a str, &'a str, &'a dyn Fn(String) -> String)> {

    let data: [(&str, &str, &dyn Fn(String) -> String); 0] = [

    ];

    data.to_vec()
}
