pub fn create_data(size_x: usize, size_y: usize)
-> (Vec<Vec<i32>>, usize){
    let mut data: Vec<Vec<i32>> = Vec::with_capacity(size_x);
    for i in 0..size_x{
        data.push(Vec::with_capacity(size_y));
        for _ in 0..size_y{
            data[i].push(0);
        }
    }
    let size = size_x * 4 * size_y;
    (data, size)
}