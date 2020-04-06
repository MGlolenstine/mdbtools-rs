#![allow(dead_code)]
use std::collections::HashMap;
use std::process::Command;
// fn main(){
//     let output = if cfg!(target_os = "windows") {
//     Command::new("cmd")
//             .args(&["/C", "echo hello"])
//             .output()
//             .expect("failed to execute process")
//     } else {
//     Command::new("sh")
//             .arg("-c")
//             .arg("echo hello")
//             .output()
//             .expect("failed to execute process")
//     };
// }
#[derive(Debug)]
struct MDatabase {
    pub file: String,
    pub tables: HashMap<String, MTable>,
    pub sql_schema: Option<String>,
}

#[derive(Clone, Default, Debug)]
struct MTable {
    pub sql: Option<String>,
    pub csv: Option<String>,
}

impl MTable{
    pub fn set_sql(&mut self, data: String){
        self.sql = Some(data);
    }

    pub fn set_csv(&mut self, data: String){
        self.csv = Some(data);
    }
}

impl MDatabase {
    fn open_database(path: &str) -> MDatabase {
        MDatabase {
            file: path.to_string(),
            tables: MDatabase::fetch_tables(path),
            sql_schema: None,
        }
    }

    pub fn fetch_sql_schema(&mut self){
        self.sql_schema = Some(std::str::from_utf8(&Command::new("mdb-schema").arg(self.file.clone()).arg("sqlite").output().expect("Something went wrong when querying the table!").stdout).expect("Something went wrong when querying the table!").to_string());
    }

    fn fetch_csv(&mut self, table: &str) {
        self.tables.get_mut(table).unwrap().set_csv(std::str::from_utf8(&Command::new("mdb-export").arg(self.file.clone()).arg(table).output().expect("Something went wrong when querying the table!").stdout).expect("Something went wrong when querying the table!").to_string());
    }

    fn fetch_sql(&mut self, table: &str) {
        println!("{} {} {} {} {}", "mdb-export", "-H", "-I sqlite", self.file.clone(), table);
        let cmd = Command::new("mdb-export").args(&["-H", "-I", "sqlite", &self.file.clone(), table]).output().expect("Something went wrong when querying the table!");
        println!("Err: {:#?}", std::str::from_utf8(&cmd.stderr));
        self.tables.get_mut(table).unwrap().set_sql(std::str::from_utf8(&cmd.stdout).expect("Something went wrong when querying the table!").to_string());
    }

    fn fetch_tables(path: &str) -> HashMap<String, MTable> {
        let out = Command::new("mdb-tables")
            .arg(path)
            .output()
            .expect("Something went wrong while reading the database file!")
            .stdout;
        let out_string = std::str::from_utf8(&out)
            .expect("The output wasn't in UTF-8")
            .to_string();
        let tables = out_string.split_whitespace();
        let mut hashmap = HashMap::<String, MTable>::new();
        for t in tables {
            hashmap.insert(t.to_string(), MTable::default());
        }
        hashmap
    }

    pub fn get_sql_schema(&mut self) -> String{
        if let Some(v) = self.sql_schema.clone() {
            return v;
        }else{
            self.fetch_sql_schema();
            return self.sql_schema.clone().unwrap();
        }
    }

    fn get_csv(&mut self, table: &str) -> String {
        if let Some(v) = self.tables.get(table).unwrap().csv.clone() {
            return v;
        } else {
            self.fetch_csv(table);
            return self.tables.get(table).unwrap().csv.clone().unwrap();
        }
    }

    fn get_sql(&mut self, table: &str) -> String {
        if let Some(v) = self.tables.get(table).unwrap().sql.clone() {
            return v;
        } else {
            self.fetch_sql(table);
            return self.tables.get(table).unwrap().sql.clone().unwrap();
        }
    }

    fn get_tables(&self) -> HashMap<String, MTable> {
        return self.tables.clone();
    }
}

#[test]
fn fetch_tables() {
    println!("{:#?}", MDatabase::open_database("Biblio.mdb"));
}

#[test]
fn get_sql_schema() {
    let mut db = MDatabase {
        file: "Biblio.mdb".to_string(),
        tables: [
            (
                "Title".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Publishers".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Authors".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Author".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Titles".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        sql_schema: None,
    };

    println!("{:#?}", db.get_sql_schema());
    println!("{:#?}", db);
    assert!(db.sql_schema.unwrap().len() >= 100)
}

#[test]
fn get_csv() {
    let mut db = MDatabase {
        file: "Biblio.mdb".to_string(),
        tables: [
            (
                "Title".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Publishers".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Authors".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Author".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Titles".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        sql_schema: None,
    };

    println!("{:#?}", db.get_csv("Titles"));
    println!("{:#?}", db);
    assert!(db.tables.get("Titles").unwrap().csv.clone().unwrap().len() >= 100)
}

#[test]
fn get_sql() {
    let mut db = MDatabase {
        file: "Biblio.mdb".to_string(),
        tables: [
            (
                "Title".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Publishers".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Authors".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Author".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
            (
                "Titles".to_string(),
                MTable {
                    sql: None,
                    csv: None,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        sql_schema: None,
    };

    println!("{:#?}", db.get_sql("Titles"));
    println!("{:#?}", db);
    assert!(db.tables.get("Titles").unwrap().sql.clone().unwrap().len() >= 100)
}