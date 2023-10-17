# Rust-And-WGPU-Study

https://www.youtube.com/playlist?list=PL_UrKDEhALdJS0VrLPn7dqC5A4W1vCAUT 강의를 보며 공부하는 자료를 올리는 공간입니다.

알아낸 점
-
run(event_loop, &window, inputs, 9) 안에서의 3번째 매개변수
더 구체적으로는 
rpass.draw(0..num_vertices, 0..1) 의 num_vertices는 정점의 개수를 나타낸다
그런데 왜 0..n과 같은 형태일까?

이유는 셰이더에서 @vertex함수에있는 @builtin(vertex_index)가 0부터 n까지 순회하며 함수를 수행하기 때문이다.
즉 @vertex함수는 한번만 호출되는게 아니라, rpass.draw에서 전달해주는 배열 숫자를 순서대로 매개변수에 넣으며 호출해주기 때문에, 각 정점을 하나하나 찍을 수 있는 것이다.