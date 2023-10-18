mod study1; //그래픽카드가 지원하는 렌더링 종류와 창 생성
mod study2; //컬러풀 삼각형 그리기
mod study3; //점 찍고 연결해보기
mod study4; //삼각형 연결해보기
mod study5; //GPU Buffer를 이용해 색상정보를 저장해 컬러풀 사각형 그리기
mod study6; //정육면체 만들기
mod study7; //정육면체 애니메이션 (울렁울렁 버텍스)
mod study8; //간단한 광원

use study1::study1_main;
use study2::study2_main;
use study3::study3_main;
use study4::study4_main;
use study5::study5_main;
use study6::study6_main;
use study7::study7_main;
use study8::study8_main;

fn main() {
    study8_main();
}
