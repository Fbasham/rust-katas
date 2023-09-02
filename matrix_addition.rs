fn matrix_addition(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.iter().zip(b).map(|(x,y)| x.iter().zip(y).map(|(i,j)| i+j).collect()).collect()
}