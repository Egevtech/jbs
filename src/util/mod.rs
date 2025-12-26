use std::collections::HashMap;

enum CommandType {
    Build,
    Run,

    Clean,

    Configure,
}

type Derives = HashMap<String, String>;

struct Command {
    command: CommandType,
    args: Vec<String>,

    derives: Derives,
}