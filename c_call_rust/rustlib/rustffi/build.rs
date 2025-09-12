use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // 현재 빌드 프로필 확인: "debug" 또는 "release"
    let profile = env::var("PROFILE").unwrap();

    // 프로젝트 경로 및 앱 이름 변수
    let project_home = PathBuf::from("D:\\repo\\X2\\c_call_rust\\");
    let app_name = "capp";

    // 빌드 결과물 경로
    let target_dir = PathBuf::from("target").join(&profile);

    // 복사 대상 파일들
    let dll_src = target_dir.join("rustffi.dll");
    let lib_src = target_dir.join("rustffi.dll.lib");

    // 복사 목적지 설정
    let out_dir = project_home
        .join(app_name)
        .join("x64")
        .join(if profile == "debug" { "Debug" } else { "Release" });

    // let out_dir = match profile.as_str() {
    //     //! PROJECT_HOME and APP_NAME 정도로 구분하는 게 좋겠다.
    //     "debug" => PathBuf::from("D:\\repo\\X2\\c_call_rust\\capp\\x64\\Debug"),
    //     "release" => PathBuf::from("D:\\repo\\X2\\c_call_rust\\capp\\x64\\Release"),
    //     _ => {
    //         println!("cargo:warning=Unknown build profile: {}", profile);
    //         return;
    //     }
    // };

    // 목적지 디렉토리 생성 (없으면)
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // 복사 실행
    fs::copy(&dll_src, out_dir.join("rustffi.dll")).expect("Failed to copy DLL");
    fs::copy(&lib_src, out_dir.join("rustffi.dll.lib")).expect("Failed to copy LIB");

    println!("cargo:warning=Copied files to {:?}", out_dir);
}