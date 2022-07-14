use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "AmpBench",
    about = "A tool to measure the write/read/space amplification of SQL database.",
    author = "zhangjinpeng1987"
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
    #[structopt(about = "prepare schema and data for database")]
    Prepare {
        #[structopt(short = "t", long = "tables", default_value = "1")]
        tables: u32,

        #[structopt(
            short = "c",
            long = "columns",
            use_delimiter = true,
            value_delimiter = ",",
            default_value = "int(10),int(10),int(10),int(10),varchar(255),varchar(255),varchar(255),datetime,text,text"
        )]
        columns: Vec<String>,

        #[structopt(
            short = "p",
            long = "primary",
            about = "primary key description, looks like \"1,3,2\", it menas the priamry key comprised\
                    of column1, column3 and column2, and in this order",
            use_delimiter = true,
            value_delimiter = ",",
            // "1,3,2" means this is a combined primary key comprised by columns 1,3,2 and associated order.
            default_value = "1,3,2" 
        )]
        primary: Vec<String>,

        #[structopt(short = "r", long = "rows", default_value = "100000")]
        rows: u32,

        #[structopt(
            short = "i", 
            long = "indexes",
            about = "indexes description, looks like \"unique_2_3,5\", it means there are 2 the first \
                    one is a unique index comprised of column2 and column3",
            use_delimiter = true,
            value_delimiter = ",", 
            // there are 2 secondary indexes, first one is a unique key comprised column2 and column3.
            default_value = "unique_2_3,5",
        )]
        indexes: Vec<String>,
    },
    #[structopt(about = "run insert workload")]
    Insert {
        #[structopt(short = "r", long = "rows", default_value = "100000")]
        rows: u32,
    },
    #[structopt(about = "run update workload")]
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
