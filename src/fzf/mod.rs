use fzf_wrapped::{run_with_output, Border, Fzf};

pub struct FzfHandler;

impl FzfHandler {
    pub fn execute_fzf(dirs: &Vec<String>) -> String {
        let fzf = Fzf::builder()
            .border(Border::Rounded)
            .border_label("Pick dir")
            .build()
            .unwrap();
        let dir = run_with_output(fzf, dirs);
        if dir.is_none() {
            panic!("Something went wrong");
        }
        dir.unwrap()
    }
}
