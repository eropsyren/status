pub struct Data {
    tag: &'static str,
    path: Option<&'static str>,
    map: Option<Box<dyn Fn(String) -> String>>,
}

impl Data {
    pub fn new_tag(tag: &'static str) -> Data {
        Data {
            tag,
            path: None,
            map: None,
        }
    }

    pub fn new_tag_path(tag: &'static str, path: &'static str) -> Data {
        Data {
            tag,
            path: Some(path),
            map: None,
        }
    }

    pub fn new_tag_path_map(
        tag: &'static str,
        path: &'static str,
        map: Box<dyn Fn(String) -> String>,
    ) -> Data {
        Data {
            tag,
            path: Some(path),
            map: Some(map),
        }
    }

    pub fn get_tag(&self) -> &'static str {
        self.tag
    }

    pub fn get_path(&self) -> Option<&'static str> {
        self.path
    }

    pub fn exec_map(&self, arg: String) -> String {
        match &self.map {
            Some(boxed_f) => boxed_f(arg),
            None => arg,
        }
    }

    pub fn is_tag_only(&self) -> bool {
        self.path.is_none() && self.map.is_none()
    }
}
