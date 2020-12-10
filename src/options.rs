#[derive(Debug, gumdrop::Options)]
pub struct Options {
    #[options(help = "print help message")]
    pub help: bool,

    #[options(command)]
    pub command: Option<Command>,
}

#[derive(Debug, gumdrop::Options)]
pub enum Command {
    #[options(help = "search for repositories")]
    Search(SearchOptions),

    #[options(help = "show repository details")]
    Show(ShowOptions),

    #[options(help = "show repository tags")]
    Tags(TagsOptions),
}

#[derive(Debug, gumdrop::Options)]
pub struct SearchOptions {
    #[options(help = "Limit the number of results")]
    pub limit: Option<usize>,

    #[options(help = "URL to send search requests")]
    pub search_url: Option<String>,

    #[options(free)]
    pub terms: Vec<String>,
}

#[derive(Debug, gumdrop::Options)]
pub struct ShowOptions {
    #[options(help = "Only show full description")]
    pub only_description: bool,

    #[options(free)]
    pub repositories: Vec<String>,
}

#[derive(Debug, gumdrop::Options)]
pub struct TagsOptions {
    #[options(help = "Limit the number of results", default = "30")]
    pub limit: usize,

    #[options(help = "Filter by architecture")]
    pub architecture: Option<String>,

    #[options(help = "Filter by operating system")]
    pub operating_system: Option<String>,

    #[options(help = "Filter by operating system and architecture of this machine")]
    pub current_machine: bool,

    #[options(free)]
    pub repositories: Vec<String>,
}
