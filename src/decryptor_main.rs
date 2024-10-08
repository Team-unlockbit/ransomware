mod decryptor;
mod readme;

use readme::DelReadme;
use mysql::*;
use mysql::prelude::*;
use std::io::{self, Write};

fn main() -> Result<(), mysql::Error> {
	// 사용자로부터 MySQL 접속 정보를 입력받음
    print!("Enter MySQL username: ");
    io::stdout().flush()?; // 출력 버퍼를 비워서 메시지를 즉시 출력
    let mut user = String::new();
    io::stdin().read_line(&mut user)?; // 사용자 이름 입력 받기
    let user = user.trim();

    print!("Enter MySQL server URL: ");
    io::stdout().flush()?;
    let mut url = String::new();
    io::stdin().read_line(&mut url)?; // 서버 URL 입력 받기
    let url = url.trim();

    print!("Enter MySQL server port: ");
    io::stdout().flush()?;
    let mut port_str = String::new();
    io::stdin().read_line(&mut port_str)?; // 포트 입력 받기
    let port: u16 = port_str.trim().parse().expect("Port must be a number");

    print!("Enter MySQL root password: ");
    io::stdout().flush()?;
    let mut passwd = String::new();
    io::stdin().read_line(&mut passwd)?; // 비밀번호 입력 받기
    let passwd = passwd.trim();

    print!("Enter MySQL database name: ");
    io::stdout().flush()?;
    let mut database = String::new();
    io::stdin().read_line(&mut database)?; // 데이터베이스 이름 입력 받기
    let database = database.trim();

    // MySQL 옵션 설정
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(url))
        .tcp_port(port)
        .user(Some(user))
        .pass(Some(passwd))
        .db_name(Some(database));

    // MySQL 서버에 연결
    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;

	let mut db_utils = DelReadme::new(user, passwd, url, port, database)?;

    // README 테이블 삭제
    db_utils.drop_table("README")?;

    // 복호화 모듈 초기화
    let key = b"an example very very secret key."; // 32 bytes
    let decryptor = decryptor::Decryptor::new(key);

    // 모든 테이블 이름 가져오기
    let tables: Vec<String> = conn.query("SHOW TABLES")?;

    // 각 테이블에 대해 작업 수행
    for table in &tables {
        println!("\n테이블: {}", table);

        // 각 테이블의 컬럼 이름과 데이터 타입 가져오기
        let query = format!("SHOW COLUMNS FROM {}", table);
        let columns: Vec<(String, String)> = conn.query_map(
            query,
            |(field, field_type, _, _, _, _): (String, String, String, String, Option<String>, String)| {
                (field, field_type)
            },
        )?;

        // 각 컬럼에 대해 데이터를 복호화하여 업데이트
        for (field, field_type) in &columns {
            if field == "id" {
                continue; // id 컬럼은 건너뛰기
            }

            // 데이터베이스에서 데이터를 읽어와 복호화 후 업데이트
            let select_query = format!("SELECT {} FROM {}", field, table);
            let rows: Vec<String> = conn.query_map(select_query, |data: String| data)?;

            for encrypted_data in rows {
                let decrypted_data = decryptor.decrypt(&encrypted_data);

                let update_query = format!(
                    "UPDATE {} SET {} = '{}' WHERE {} = '{}'",
                    table, field, decrypted_data, field, encrypted_data
                );

                conn.exec_drop(update_query, ())?;
            }
        }

        println!("테이블 {}의 모든 컬럼 데이터가 복호화되었습니다.", table);
    }

    Ok(())
}

