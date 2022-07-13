
enum ColumnType {
    Int,
    Char,
    Varchar,
    Text,
    Datetime,
}

struct Column {
    ty: ColumnType,
    len: usize,
    notnull: bool,
    default: Option<String>,
}

// secondary index
struct Index {
    // seconda
    columns: Vec<usize>,
    unique: bool,
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

const alpha: &str = "abcdefghijklmnopqrstuvwxyz";

fn gen_column_name(mut idx: usize) -> String {
    idx += 1;
    let alpha_len = alpha.len();
    let mut res = String::new("");
    while idx > 0 {
        res.push(alpha.as_bytes()[idx % alpha_len - 1] as char);
        idx /= alpha_len;
    }
    res
}

impl Schema {
    pub fn new(columns: Vec<Column>, indexes: Vec<String>, table_num: usize) -> Self {
        Self {
            columns,
            indexes,
            table_num,
        }
    }

    pub fn gen_create_sql(&self) -> String {
        let mut sql = String::new("create table t");
        sql.add(self.table_num.to_str());
        sql.add("(");

        // concat columns
        let mut i = 0;
        while i < self.columns.len() {
            let c_name = gen_column_name(i);
            sql.add(&c_name);
            sql.push(' ');
            match self.columns[i].ty {
                ColumnType::Int => {
                    sql.add("int(");
                    sql.add(self.columns[i].len.to_str());
                    sql.push(')');
                }
                ColumnType::Char => {
                    sql.add("char(");
                    sql.add(self.columns[i].len.to_str());
                    sql.push(')');
                }
                ColumnType::Varchar => {
                    sql.add("varchar(");
                    sql.add(self.columns[i].len.to_str());
                    sql.push(')');
                }
                ColumnType::Text => {
                    sql.add("text");
                }
                ColumnType::Datetime => {
                    sql.add("datetime");
                }
            }
            if self.columns[i].notnull {
                sql.add(" not null");
            }
            if self.columns[i].default.is_some() {
                sql.add(" default ");
                sql.add(self.columns[i].default.as_ref().unwrap());
            }
            sql.push(',');
        }

        // concat primary key
        sql.add("primary key (");
        i = 0;
        while i < self.primary.len() {
            sql.add(gen_column_name(self.primary[i]));
            i += 1;
            if i < self.primary.len() {
                sql.add(",");
            } else {
                sql.add(") ");
            }
        }

        // concat secondary index
        if !self.indexes.is_empty() {
            sql.add(", key _idx_" + i.to_str() + );

            i = 0;
            while i < self.indexes.len() {
                sql.add(gen_column_name(self.primary[i]));
                i += 1;
                if i < self.primary.len() {
                    sql.add(",");
                } else {
                    sql.add(") ");
                }
            }
        }

        sql
    }

    pub fn gen_insert_sql(&self, )
}

#[cfg(test)]
mod tests {

}
