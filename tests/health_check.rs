// lib.rs 파일 함수 따로 use 없이 사용 가능
// Cargo.toml [lib]에 작성하면 전역적? 사용가능한가봄..

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}

async fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // 아래서 await을 추가적으로 작성할 경우 지정된 주소를 무한히 대기함. 요청 도달시 처리하지만 스스로 종료 또는 완료하지 않음. 따라서 테스트 로직 실행 안됨
    // tokio::spawn은 하나의 Future를 받아서 런타임에 전달해 등록하고 종료될 때까지 기다리지않음. 따라서 이후 Future 및 태스크(테스트 로직)과 동시에 실행 됨
    let _ = tokio::spawn(server);
}
