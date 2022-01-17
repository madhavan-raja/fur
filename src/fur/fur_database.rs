use std::fs;
use std::path::PathBuf;

pub struct FurDB {
    dir: PathBuf,
}

impl FurDB {
    pub fn new(dir: PathBuf) -> Result<FurDB, &'static str> {
        // TODO: Handle the errors
        if false {
            return Err("Error occoured");
        }

        Ok(FurDB { dir: dir })
    }

    pub fn get_name(db: &FurDB) -> String {
        Self::get_info(db, "name")
    }

    pub fn get_description(db: &FurDB) -> String {
        Self::get_info(db, "description")
    }

    pub fn get_tables(db: &FurDB) -> Vec<String> {
        let mut tables = Vec::new();

        for file in fs::read_dir(&db.dir).unwrap() {
            let file_name = file.unwrap();
            let metadata = fs::metadata(&file_name.path());

            if metadata.unwrap().is_dir() {
                tables.push(file_name.file_name().to_string_lossy().to_string());
            }
        }

        tables
    }

    fn get_info_file_path(db: &FurDB) -> String {
        db.dir.as_path().display().to_string() + "\\fur.json"
    }

    fn get_info(db: &FurDB, property: &str) -> String {
        let db_info_path = Self::get_info_file_path(db);

        let db_info_contents_raw =
            fs::read_to_string(&db_info_path).expect("Something went wrong while reading the file");

        let db_info_contents: serde_json::Value = serde_json::from_str(&db_info_contents_raw)
            .expect("Something went wrong while parsing JSON");

        let property = db_info_contents.get(property).unwrap().to_string();

        property
    }
}
