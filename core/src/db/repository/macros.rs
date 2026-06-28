macro_rules! fetch_all {
    ($conn:expr, $sql:expr, $params:expr, |$row:ident| $mapping:expr) => {{
        let mut stmt = $conn.prepare($sql).map_err(|e| format!("Prepare error: {}", e))?;
        let rows = stmt
            .query_map($params, |$row| Ok($mapping))
            .map_err(|e| format!("Query error: {}", e))?;
    
        let mut items = Vec::new();
        for row in rows {
            items.push(row.map_err(|e| format!("Mapping error: {}", e))?);
        }
        Ok(items)
    }};
}

macro_rules! fetch_one {
    ($conn:expr, $sql:expr, $params:expr, |$row:ident| $mapping:expr) => {{
        let mut stmt = $conn.prepare($sql).map_err(|e| format!("Prepare error: {}", e))?;
        let row = stmt
            .query_row($params, |$row| Ok($mapping));

        match row {
            Ok(r) => Ok(Some(r)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Database query error: {}", e)),
        }
    }};
}

macro_rules! execute_modify {
    ($conn:expr, $sql:expr, $params:expr, $err_msg:expr) => {{
        let rows_affected = $conn
            .execute($sql, $params)
            .map_err(|e| format!("Database error: {}", e))?;

        if rows_affected == 0 {
            return Err($err_msg.to_string());
        }
        Ok(())
    }};
}

macro_rules! execute_insert {
    ($conn:expr, $sql:expr, $params:expr) => {{
        $conn
            .execute($sql, $params)
            .map_err(|e| format!("Insert error: {}", e))?;

        Ok($conn.last_insert_rowid())
    }};
}

macro_rules! push_field {
    ($update_parts:expr, $params:expr, $column:expr, $option:expr) => {
        if let Some(inner) = &$option {
            $update_parts.push($column);
            $params.push(inner as &dyn rusqlite::ToSql);
        }
    };
}