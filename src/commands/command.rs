pub trait Command {
    fn execute(&self) -> String;
}
