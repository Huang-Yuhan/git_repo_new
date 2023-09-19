pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len()==0 {return None;}
    let mut l:i32=0;
    let mut r=array.len() as i32 -1;
    let mut mid=0;
    let mut flag=false;
    loop
    {
        if l>r {break;}
        mid=((l+r)/2)as usize;
        if array[mid]<key 
        {
            l=mid as i32 +1;
        }
        else if array[mid] >key
        {
            r=mid as i32 -1;
        }
        else if array[mid]==key {flag=true;break;}
    }
    match flag
    {
        true=> Some(mid),
        false=> None,
    }
}
