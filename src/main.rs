mod study1; //그래픽카드가 지원하는 렌더링 종류와 창 생성
mod study2; //컬러풀 삼각형 그리기
mod study3; //점 찍고 연결해보기
mod study4; //삼각형 연결해보기
mod study5; //GPU Buffer를 이용해 색상정보를 저장해 컬러풀 사각형 그리기

use study1::study1_main;
use study2::study2_main;
use study3::study3_main;
use study4::study4_main;
use study5::study5_main;

fn main() {
    study5_main();
}
