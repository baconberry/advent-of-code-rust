pub fn vec_intersect<E>(a: &Vec<E>, b: &Vec<E>) -> Vec<E>
where
    E: Clone + PartialEq,
{
    let mut result: Vec<E> = Vec::new();
    for el_a in a {
        for el_b in b {
            if el_a == el_b {
                result.push(el_a.clone());
            }
        }
    }

    result
}
