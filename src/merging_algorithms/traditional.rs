pub fn merge(list: &mut Vec<i32>, left: usize, right:usize, end: usize){
    if list[right - 1] <= list[right] { return; }
    let mut temp: Vec<i32> = vec![0; end - left];
    let mut cur_left = left;
    let mut cur_right = right;
    let mut write_idx = 0;
    while cur_left < right && cur_right < end {
        if list[cur_right] < list[cur_left] { 
            temp[write_idx] = list[cur_right];
            cur_right += 1;
        } else {
            temp[write_idx] = list[cur_left];
            cur_left += 1;
        }
        write_idx += 1;
    }

    while cur_left < right {
        list[end - (right - cur_left)] = list[cur_left];
        cur_left += 1;
    }

    for i in 0..write_idx{ 
        list[left + i] = temp[i];
    }
}
