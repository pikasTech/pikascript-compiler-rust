use crate::class_info::ClassInfo;
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct Compiler {
    pub class_list: BTreeMap<String, ClassInfo>,
    pub class_now_name: Option<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        let compiler = Compiler {
            class_now_name: None,
            class_list: BTreeMap::new(),
        };
        return compiler;
    }
    pub fn analyze_line(mut compiler: Compiler, line: String) -> Compiler {
        if line.starts_with("#") {
            return compiler;
        }
        if line.starts_with("class") {
            let class_now = match ClassInfo::new(&line) {
                Some(s) => s,
                None => return compiler,
            };
            let class_name = class_now.this_class_name.clone();
            compiler
                .class_list
                .entry(class_name.clone())
                .or_insert(class_now);
            compiler.class_now_name = Some(class_name.clone());
            return compiler;
        }
        if line.starts_with("    def ") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            let class_now = compiler
                .class_list
                .get_mut(&compiler.class_now_name.clone().unwrap())
                .unwrap();
            class_now.push_method(line);
            return compiler;
        }
        if line.starts_with("    ")
            && line.contains("(")
            && line.contains(")")
            && line.contains("=")
        {
            let line = line.strip_prefix("    ").unwrap().to_string();
            let class_now = compiler
                .class_list
                .get_mut(&compiler.class_now_name.clone().unwrap())
                .unwrap();
            class_now.push_object(line);
            return compiler;
        }
        if line.starts_with("    ") && line.contains("(") && line.contains(")") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            let class_now = compiler
                .class_list
                .get_mut(&compiler.class_now_name.clone().unwrap())
                .unwrap();
            class_now.push_import(line);
            return compiler;
        }
        return compiler;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_analyze() {
        let compiler = Compiler {
            class_now_name: None,
            class_list: BTreeMap::new(),
        };
        let compiler = Compiler::analyze_line(compiler, String::from("class Test(SuperTest):"));
        let compiler = Compiler::analyze_line(compiler, String::from("    def test()"));
        let compiler = Compiler::analyze_line(compiler, String::from("    testObj = TestObj()"));
        let compiler = Compiler::analyze_line(compiler, String::from("    TestImport()"));

        let class_info = compiler.class_list.get("Test").unwrap();
        let method_info = class_info.method_list.get("test").unwrap();
        let object_info = class_info.object_list.get("testObj").unwrap();
        let import_info = class_info.import_list.get("TestImport").unwrap();
        assert_eq!(class_info.this_class_name, "Test");
        assert_eq!(class_info.super_class_name, "SuperTest");
        assert_eq!(method_info.name, "test");
        assert_eq!(method_info.class_name, "Test");
        assert_eq!(object_info.name, "testObj");
        assert_eq!(object_info.class_name, "Test");
        assert_eq!(import_info.class_name, "Test");
        assert_eq!(import_info.import_class_name, "TestImport");
    }
}
