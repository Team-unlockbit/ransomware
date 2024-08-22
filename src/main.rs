use mysql::*;
use mysql::prelude::*;
use std::io::{self, Write};

fn main() -> Result<(), mysql::Error> {
    // MySQL 접속 정보
    let user = "root";    // 사용자 이름
    let url = "127.0.0.1";    // MySQL 서버 URL (포트 제외)
    let port = 13306;

    // 사용자로부터 MySQL root 비밀번호를 입력받음
    print!("Enter MySQL root password: ");
    io::stdout().flush()?; // 출력 버퍼를 비워서 메시지를 즉시 출력
    let mut passwd = String::new();
    io::stdin().read_line(&mut passwd)?; // 비밀번호 입력 받기

    // 입력된 비밀번호의 끝에 개행 문자가 포함될 수 있으므로 제거
    let passwd = passwd.trim();

    let database = "example_db";

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

        // 각 컬럼에 대해 업데이트 작업 수행
        for (field, field_type) in &columns {
            if field == "id" {
                continue; // id 컬럼은 건너뛰기
            }

            // 컬럼 타입 변경 쿼리
            let alter_query = format!("ALTER TABLE {} MODIFY {} VARCHAR(255)", table, field);

            // 타입 변경 쿼리 실행
            conn.exec_drop(alter_query, ())?;

            // 컬럼 타입에 따른 업데이트 쿼리 작성 및 실행
            let update_query = format!(
                "UPDATE {} SET {} = CONCAT({}, '_qwer')",
                table, field, field
            );

            // 업데이트 쿼리 실행
            conn.exec_drop(update_query, ())?;
        }

        println!("테이블 {}의 모든 컬럼 데이터가 업데이트되었습니다.", table);
    }

    Ok(())
}

