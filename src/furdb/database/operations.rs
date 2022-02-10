use crate::furdb::{FurDB, FurTable, FurTableInfo};

impl FurDB {
    pub fn get_all_table_ids(&self) -> std::io::Result<Vec<String>> {
        let mut tables = Vec::new();

        for file in std::fs::read_dir(&self.dir)? {
            let file_name = file?;
            let metadata = std::fs::metadata(&file_name.path());

            if metadata?.is_dir() {
                tables.push(file_name.file_name().to_string_lossy().to_string());
            }
        }

        Ok(tables)
    }

    pub fn get_table(
        &self,
        table_id: &str,
        table_info: Option<FurTableInfo>,
    ) -> std::io::Result<FurTable> {
        let mut table_dir_path = self.dir.clone();
        table_dir_path.push(table_id);
        let tb = FurTable::new(table_dir_path, table_info)?;

        Ok(tb)
    }
}
