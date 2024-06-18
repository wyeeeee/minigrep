pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool, //忽略字符大小写的差别。 -i
    pub invert_match: bool, // 显示不包含匹配文本的所有行。 -v
    pub line_number:bool,//在显示符合样式的那一行之前，标示出该行的列数编号。 -n
    pub recursive: bool, // 递归地运用grep命令到所有子目录中。 -r
    pub file_with_matches:bool, // 只显示匹配的文件名。 -l
    pub count:bool, //计算符合样式的列数。 -c

}
impl Config {
    pub fn new(mut args:impl Iterator<Item = String>) -> Result<Config, &'static str>{
        args.next();
        let mut ignore_case = false;
        let mut invert_match = false;
        let mut line_number = false;
        let mut recursive = false;
        let mut file_with_matches = false;
        let mut count = false;
        let mut file_path = String::new();
        let mut query = String::new();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i"=>{ignore_case = true;},
                "-v"=>{invert_match = true;},
                "-n"=>{line_number = true;},
                "-r"=>{recursive = true;},
                "-l"=>{file_with_matches = true;},      
                "-c"=>{count = true;},
                "-p"|"--path"=>{
                    file_path=match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a query string"),
                    };
                },
                "-q"|"--query"=>{
                    query=match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a query string"),
                    };
                },
                _ => return Err("Unknown argument"),
            }
        }

        Ok(Config {query,file_path,ignore_case,invert_match,line_number,recursive,file_with_matches,count})
    }
}