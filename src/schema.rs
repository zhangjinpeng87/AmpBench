#[derive(PartialEq, Debug)]
enum ColumnType {
    Int,
    Char,
    Varchar,
    Text,
    Datetime,
}

struct Column {
    tp: ColumnType,
    len: usize,
    default: Option<String>,
}

impl From<&str> for Column {
    // The input is looks like "int(10)_default_5" or "varchar(32)"
    fn from(s: &str) -> Column {
        let column_str: Vec<_> = s.split('_').collect();
        let mut len = 0;
        // int(10)
        let column_info: Vec<_> = column_str[0].split(&['(', ')'][..]).collect();
        let tp = match column_info[0] {
            "int" => ColumnType::Int,
            "char" => ColumnType::Char,
            "varchar" => ColumnType::Varchar,
            "text" => ColumnType::Text,
            "datetime" => ColumnType::Datetime,
            _ => {
                unimplemented!("unimplemented column type");
            }
        };
        if column_info.len() >= 2 {
            len = column_info[1].parse::<usize>().unwrap();
        }

        // default_5
        let mut default = None;
        if column_str.len() == 3 {
            assert_eq!(column_str[1], "default");
            default = Some(String::from(column_str[2]));
        }

        Column { tp, len, default }
    }
}

// Secondary index
struct Index {
    // columns of this index
    columns: Vec<usize>,
    unique: bool,
}

impl From<&str> for Index {
    // The input is looks like "unique_1_2" or "3".
    fn from(s: &str) -> Index {
        let mut columns_str: Vec<_> = s.split('_').collect();
        let mut unique = false;
        if columns_str[0] == "unique" {
            unique = true;
            columns_str.remove(0);
        }
        let columns: Vec<usize> = columns_str
            .iter()
            .map(|s| str::parse::<usize>(s).unwrap())
            .collect();
        Index { columns, unique }
    }
}

struct Schema {
    columns: Vec<Column>,
    // primary key
    // if the len of vec is greater than 1, then it is a combined primary key, and
    // the vec contains all column idx in the combined order.
    primary: Vec<usize>,
    indexes: Vec<Index>,
    table_num: usize,
}

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

fn gen_column_name(mut idx: usize) -> String {
    idx += 1;
    let alpha_len = ALPHA.len();
    let mut res = String::new();
    while idx > 0 {
        res.push(ALPHA.as_bytes()[idx % alpha_len - 1] as char);
        idx /= alpha_len;
    }
    res
}

fn gen_secondary_index_sql(idx_num: usize, index: &Index) -> String {
    let mut sql = String::new();
    if index.unique {
        sql.push_str(" unique ");
    }
    sql.push_str(&(String::from(" key _idx_") + &idx_num.to_string() + " ("));
    for (i, col) in index.columns.iter().enumerate() {
        sql.push_str(&gen_column_name(*col));
        if i + 1 < index.columns.len() {
            sql.push_str(",");
        } else {
            sql.push_str(")");
        }
    }
    sql
}

impl Schema {
    pub fn from_str(
        columns: Vec<String>,
        primary: Vec<String>,
        indexes: Vec<String>,
        table_num: usize,
    ) -> Self {
        // columns = ["int(10)" "int(10)" "char(32)" "varchar(255)" "text" "datetime" ...]
        let columns: Vec<_> = columns.iter().map(|s| Column::from(s.as_str())).collect();
        // primary = [1, 3, 2] means this is a combined primary key comprised by 3 columns and the columns order is 1,3,2
        let primary = primary
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        // indexes = ["unique_1_2" "6"] means there are 2 secondary indexes, the first one is
        let indexes = indexes.iter().map(|s| Index::from(s.as_str())).collect();
        Self {
            columns,
            primary,
            indexes,
            table_num,
        }
    }

    pub fn gen_create_sql(&self) -> String {
        let mut sql = String::from("create table t");
        sql.push_str(&self.table_num.to_string());
        sql.push_str(" (\n");

        // concat columns
        let mut i = 0;
        while i < self.columns.len() {
            let c_name = gen_column_name(i);
            sql.push_str(" ");
            sql.push_str(&c_name);
            sql.push_str(" ");
            match self.columns[i].tp {
                ColumnType::Int => {
                    sql.push_str("int(");
                    sql.push_str(&self.columns[i].len.to_string());
                    sql.push(')');
                }
                ColumnType::Char => {
                    sql.push_str("char(");
                    sql.push_str(&self.columns[i].len.to_string());
                    sql.push(')');
                }
                ColumnType::Varchar => {
                    sql.push_str("varchar(");
                    sql.push_str(&self.columns[i].len.to_string());
                    sql.push(')');
                }
                ColumnType::Text => {
                    sql.push_str("text");
                }
                ColumnType::Datetime => {
                    sql.push_str("datetime");
                }
            }
            sql.push_str(" not null");
            if self.columns[i].default.is_some() {
                sql.push_str(" default ");
                sql.push_str(self.columns[i].default.as_ref().unwrap());
            }
            sql.push_str(",\n");
            i += 1;
        }

        // concat primary key
        sql.push_str(" primary key (");
        i = 0;
        while i < self.primary.len() {
            sql.push_str(&gen_column_name(self.primary[i]));
            i += 1;
            if i < self.primary.len() {
                sql.push_str(",");
            } else {
                sql.push_str(")");
            }
        }

        // concat secondary index
        if !self.indexes.is_empty() {
            sql.push_str(",\n");
            for (i, index) in self.indexes.iter().enumerate() {
                sql.push_str(&gen_secondary_index_sql(i, index));
                if i + 1 < self.indexes.len() {
                    sql.push_str(",\n");
                } else {
                    sql.push_str("\n");
                }
            }
        }

        sql.push_str(")");
        sql
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_column() {
        let col = Column::from("int(10)_default_1");
        assert_eq!(col.tp, ColumnType::Int);
        assert_eq!(col.len, 10);
        assert_eq!(col.default, Some(String::from("1")));

        let col2 = Column::from("varchar(32)_default_aaa");
        assert_eq!(col2.tp, ColumnType::Varchar);
        assert_eq!(col2.len, 32);
        assert_eq!(col2.default, Some(String::from("aaa")));

        let col3 = Column::from("datetime_default_now()");
        assert_eq!(col3.tp, ColumnType::Datetime);
        assert_eq!(col3.len, 0);
        assert_eq!(col3.default, Some(String::from("now()")));

        let col3 = Column::from("datetime");
        assert_eq!(col3.tp, ColumnType::Datetime);
        assert_eq!(col3.len, 0);
        assert_eq!(col3.default, None);
    }

    #[test]
    fn test_parse_index() {
        let idx = Index::from("unique_1_3");
        assert!(idx.unique);
        assert_eq!(idx.columns, vec![1, 3]);

        let idx2 = Index::from("2_3");
        assert!(!idx2.unique);
        assert_eq!(idx2.columns, vec![2, 3]);
    }

    #[test]
    fn test_schema() {
        let columns: Vec<String> = vec![
            "int(10)".to_owned(),
            "int(10)".to_owned(),
            "char(32)".to_owned(),
            "varchar(255)".to_owned(),
            "text".to_owned(),
            "datetime".to_owned(),
        ];
        let primary = vec!["1".to_owned(), "3".to_owned(), "2".to_owned()];
        let indexes = vec!["unique_1_4".to_owned(), "5".to_owned()];
        let schema = Schema::from_str(columns, primary, indexes, 0);
        let create_sql = "create table t0 (\n a int(10) not null,\n b int(10) not null,\n c char(32) not null,\n d varchar(255) not null,\n e text not null,\n f datetime not null,\n primary key (b,d,c),\n unique  key _idx_0 (b,e),\n key _idx_1 (f)\n)";
        assert_eq!(create_sql, schema.gen_create_sql());
    }
}
