use std::{cmp::min, arch::x86_64::_mm256_and_ps};

pub fn getdata(x:usize,y:usize,g:&Vec<&str>)->char
{
    g[x].chars().nth(y).unwrap()
}


pub fn getcount(x:usize,y:usize,g:&Vec<&str>)->u32
{
    let mut count=0;
    let row=g.len();
    let col=g[0].len();
    let xmin = if x<1{0}else{x-1};
    let ymin =if y<1{0}else{y-1};
    for xpos in xmin..x+2
    {
        for ypos in ymin..y+2
        {
            if xpos<row&&ypos<col
            {
                if  getdata(xpos, ypos, g)=='*'
                {
                    count=count+1;
                }
            }
        }
    }
    count
}

pub fn annotate(minefield: &Vec<&str>) -> Vec<String> {
    let mut ans:Vec<String> = Vec::new();
    if minefield.is_empty()
    {
        return vec![];
    }
    if(minefield[0].is_empty())
    {
        return vec![String::from("")];
    }

    for i in 0..minefield.len()
    {
        let mut row=String::new();
        for j in 0..minefield[0].len()
        {
            let ch=minefield[i].chars().nth(j).unwrap();
            if ch=='*'
            {
                row.push('*');
            }
            else {
                let count=getcount(i,j,minefield);
                let ch= if count >0 {count.to_string().chars().nth(0).unwrap()}else{' ' };
                row.push(ch);
            }
        }
        ans.push(row);
    }
    ans
}
