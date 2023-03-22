/*
 * Copyright 2023 One Law LLC. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use core::ffi::c_char;
use sqlite::ManagedConnection;
use sqlite::{Connection, ResultCode};
use sqlite_nostd as sqlite;

fn opendb() -> Result<ManagedConnection, ResultCode> {
    let connection = sqlite::open(sqlite::strlit!(":memory:"))?;
    connection.enable_load_extension(true)?;
    connection.load_extension("../../dbg/crsqlite", None)?;
    Ok(connection)
}

fn closedb(db: ManagedConnection) -> Result<(), ResultCode> {
    db.exec_safe("SELECT crsql_finalize()")?;
    Ok(())
}

#[test]
fn tear_down() {
    tear_down_impl().unwrap();
}

fn tear_down_impl() -> Result<(), ResultCode> {
    let db = opendb()?;
    db.exec_safe("CREATE TABLE foo (a primary key, b);")?;
    db.exec_safe("SELECT crsql_as_crr('foo');")?;
    db.exec_safe("SELECT crsql_as_table('foo');")?;
    let stmt = db.prepare_v2("SELECT count(*) FROM sqlite_master WHERE name LIKE 'foo__%'")?;
    stmt.step()?;
    let count = stmt.column_int(0)?;
    assert!(count == 0);
    closedb(db)
}