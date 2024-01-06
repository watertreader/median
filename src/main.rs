fn median(mut a: Vec<f32>) -> Option<f32> {
    
    if a.is_empty()
    {
        return None;
    }

    a.sort_by( |x,y| x.partial_cmp(y).unwrap());

    let length = a.len();
    let mid    = length / 2;
   
    let num = if length % 2 == 0
    {
       ( a[mid-1] + a[mid] ) / 2.0
    }
    else 
    {   
       a[mid]
    };
    
    Some(num)
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}