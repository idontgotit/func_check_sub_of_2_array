fn is_sub_array(origin_array: &[i32],sub_array: &[i32]) -> bool{
	let n = origin_array.len();
	let m = sub_array.len();
	if n < m{
		return false;
	}
    let mut i = 0; let mut j = 0;
    while i < n && j < m{
        if origin_array[i] == sub_array[j]{
            i += 1;
            j += 1;
            if j == m{
                return true;
            }
        }
        else{
            i = i - j + 1;
            j = 0;
        }
    }      
    return false;
}

fn main() {
	let org_arr = [1, 2,3,5,6,8, 10, 11];
	let sub_arr = [6,8,10];
    let is_sub = is_sub_array(&org_arr, &sub_arr);
    if is_sub == true{
    	println!("sub_arr is sub of org_arr");
    }else{
    	println!("sub_arr NOT is sub of org_arr");
    }

}
