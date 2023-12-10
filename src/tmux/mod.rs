use tmux_interface::{AttachSession, HasSession, NewSession, Tmux};
pub struct TmuxHandler;

impl TmuxHandler {
    pub fn execute_tmux_sessionize(dir: String) {
        let session_name = Self::create_session_name(dir.clone());
        let status = Tmux::with_command(HasSession::new().target_session(session_name.clone()))
            .output()
            .unwrap()
            .success();
        if !status {
            let _ = Tmux::with_command(
                NewSession::new()
                    .session_name(session_name)
                    .start_directory(dir),
            )
            .output();
        } else {
            let _ = Tmux::with_command(AttachSession::new().target_session(session_name)).output();
        }
    }
    fn clean_name(unclean_name: String) -> String {
        unclean_name
            .replace('.', "")
            .replace(['/', ' '], "_")
            .replace('\n', "")
    }
    fn create_session_name(dir: String) -> String {
        Self::clean_name(String::from(dir.split('/').last().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_name_removes_slash_and_replace_with_underline() {
        let dir = String::from("/foo/bar");
        let expected = String::from("_foo_bar");
        let res = TmuxHandler::clean_name(dir);
        assert_eq!(res, expected);
    }
    #[test]
    fn test_clean_name_removes_space_and_replace_with_underline() {
        let dir = String::from("foo bar");
        let expected = String::from("foo_bar");
        let res = TmuxHandler::clean_name(dir);
        assert_eq!(res, expected);
    }
    #[test]
    fn test_clean_name_removes_dot() {
        let dir = String::from(".foo.bar");
        let expected = String::from("foobar");
        let res = TmuxHandler::clean_name(dir);
        assert_eq!(res, expected);
    }
    #[test]
    fn test_clean_name_removes_newline() {
        let dir = String::from("foo\n");
        let expected = String::from("foo");
        let res = TmuxHandler::clean_name(dir);
        assert_eq!(res, expected);
    }
    #[test]
    fn test_create_session_name_gets_name_of_dir_in_dir_path() {
        let dir = String::from("foo/bar/baz");
        let expected = String::from("baz");
        let res = TmuxHandler::create_session_name(dir);
        assert_eq!(res, expected);
    }
}
