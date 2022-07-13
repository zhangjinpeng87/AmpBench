use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "AmpBench",
    about = "A tool to measure the write/read/space amplification of SQL database.",
    author = "zhangjinpeng1987",
)]
pub struct Opt {
    #[structopt(long, default_value = "127.0.0.1")]
    /// Set db host
    pub dbhost: String,

    #[structopt(long, default_value = "4000")]
    /// Set db port
    pub dbport: u32,

    #[structopt(long, default_value = "tidb")]
    /// Set db
    pub db: String,

    #[structopt(long, default_value = "tidb")]
    /// Set user name
    pub user: String,

    #[structopt(long, default_value = "tidb")]
    /// Set password
    pub pwd: String,

    #[structopt(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(
        about = "prepare schema and data for database"
    )]
    Prepare {
        #[structopt(short = "t", long = "tables", default_value = "1")]
        tables: u32,

        #[structopt(
            short = "c",
            long = "columns",
            use_delimiter = true,
            require_delimiter = true,
            value_delimiter = ",",
            default_value = "int(10),int(10),int(10),int(10),varchar(255),varchar(255),varchar(255),datetime,text,text"
        )]
        columns: Vec<String>,

        #[structopt(short = "r", long = "rows", default_value = "100000")]
        rows: u32,

        #[structopt(
            short = "i", 
            long = "indexes",
            about = "index description, 1 means create index(column1), 2_3 means create index(column2, column3)",
            use_delimiter = true,
            require_delimiter = true,
            value_delimiter = ",", 
            default_value = "1,2_3,5,8",
        )]
        indexes: Vec<String>,
    },
    #[structopt(
        about = "run insert workload"
    )]
    Insert {
        #[structopt(short = "r", long = "rows", default_value = "100000")]
        rows: u32,
    },
    #[structopt(
        about = "run update workload"
    )]
    Update {
        #[structopt(short = "r", long = "rows", default_value = "100000")]
        rows: u32,

        #[structopt(short = "i", long = "index")]
        index: bool,
    },
    /* Delete {
        #[structopt(short = "c", long = "columns", default_value = "10")]
        pub columns: u32,

        #[structopt(
            short = "t",
            long = "types",
            use_delimiter = true,
            require_delimiter = true,
            value_delimiter = ",",
            default_value = "int,int,int,int,varchar(255),varchar(255),varchar(255),datetime,text,text"
        )]
        pub types: Vec<String>,

        #[structopt(short = "r", long = "rows", default_value = "100000")]
        pub rows: u32,

        #[structopt(short = "i", long = "indexes", default_value = "3")]
        pub indexes: u32
    },
    Select {

    }, */
}