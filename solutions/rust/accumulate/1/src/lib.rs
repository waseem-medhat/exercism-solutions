pub fn map<A, B, F>(input: Vec<A>, mut function: F) -> Vec<B>
where
    F: FnMut(A) -> B,
{
    let mut res = Vec::with_capacity(input.len());
    for item in input {
        res.push(function(item));
    }
    res
}

