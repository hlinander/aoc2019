use std::env;
use std::cmp;
use std::fs;
use image::{Rgb, RgbImage};
use image::imageops::{resize};
use imageproc::drawing::{
    draw_line_segment_mut, draw_filled_circle_mut
};
use imageproc::rect::Rect;
use std::path::Path;

fn get_dims(path: &Vec<(char, i32)>) -> (i32, i32)
{
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    let mut minx : i32 = 0;
    let mut miny : i32 = 0;
    let mut maxx : i32 = 0;
    let mut maxy : i32 = 0;
    for op in path
    {
        match op.0
        {
            'R' => x += op.1,
            'L' => x -= op.1,
            'U' => y += op.1,
            'D' => y -= op.1,
            _ => {}
        };
        if x > maxx
        {
            maxx = x;
        }
        if x < minx
        {
            minx = x;
        }
        if y > maxy
        {
            maxy = y;
        }
        if y < miny
        {
            miny = y;
        }
    }
    (maxx - minx, maxy - miny)
}

fn line(mut buff : &mut Vec<u8>, delay: &mut u32, line_index: usize, w: i32, sx : i32, sy: i32, mut dx: i32, mut dy: i32) -> Vec<(i32, i32, u32)>
{
    let mut intersections : Vec<(i32, i32, u32)> = Vec::new();
    while(dx != 0 || dy !=0)
    {
        // 
        let idx = (sx + dx + (sy + dy) * w) as usize;
        if(buff[idx] != 0 && buff[idx] != line_index as u8)
        {
            println!("intersection");
            intersections.push((sx + dx, sy + dy, *delay));
        }
        *delay += 1;
        buff[idx] = line_index as u8;
        if(dx > 0) {
            dx -= 1;
        }
        if(dx < 0) {
            dx += 1;
        }
        if(dy > 0) {
            dy -= 1;
        }
        if(dy < 0) {
            dy += 1;
        }
    }    
    buff[(sx + sy * w) as usize] = line_index as u8;
    intersections
}

fn main() {
    let path = Path::new("out.png");
    let data = fs::read_to_string("data/3")
        .expect("Something went wrong reading the file");
    let paths : Vec<Vec<(char, i32)>> = data.split("\n")
                      .map(|path_str| path_str.split(",")
                                              .map(|op| (op.as_bytes()[0] as char, (&op[1..]).parse::<i32>().unwrap())
                                              ).collect())
                      .collect();
    let mut w : i32 = 0;
    let mut h : i32 = 0;
    for path in &paths
    {
        let (cw, ch) = get_dims(path);
        w = cmp::max(w, cw);
        h = cmp::max(h, ch);
    }
    println!("w: {}, h: {}", w as u32, h as u32);
    let mut image = RgbImage::new(((2 * w) as f32 / 10.0) as u32, ((2 * h) as f32 / 10.0) as u32);
    let colors = [Rgb([255u8, 0u8,   0u8]), Rgb([0u8, 255u8,   0u8])];
    let mut buff : Vec<u8> = vec![0; (5 * w * h) as usize];
    let mut intersections : Vec<(i32, i32, u32)> = Vec::new();
    for (i, path) in (&paths).iter().enumerate()
    {
        let mut x = w;
        let mut y = h;
        let mut delay = 0u32;
        for op in path
        {
            let ox = x;
            let oy = y;
            match op.0
            {
                'R' => x += op.1,
                'L' => x -= op.1,
                'U' => y += op.1,
                'D' => y -= op.1,
                _ => {}
            };
            intersections.extend(line(&mut buff, &mut delay, i + 1, 2 * w, ox, oy, x - ox, y - oy));
            draw_line_segment_mut(&mut image, ((ox as f32) / 10.0, (oy as f32) / 10.0), ((x as f32) / 10.0, (y as f32) / 10.0), colors[i]);
            println!("({}, {}) -> ({}, {})", ox, oy, x, y);
        }
    }
    for coord in &intersections
    {
        draw_filled_circle_mut(&mut image, ((coord.0 as f32 / 10.0) as i32, (coord.1 as f32 / 10.0) as i32), 5i32, colors[1]);
    }
    let mh = |(x1, y1, d1) : (i32, i32, u32), (x2, y2, d2): (i32, i32, u32)| (x2 - x1).abs() + (y2 - y1).abs();
    intersections.sort_by(|p1, p2| (mh(*p1, (w, h, 0))).cmp(&(mh(*p2, (w, h, 0)))));
    let mhs : Vec<i32> = intersections.clone().iter().map(|p| mh(*p, (w, h, 0))).collect();
    let cs : Vec<u32> = intersections.clone().iter().map(|(x, y, d)| )
    println!("{}", mhs[0]);
    let coord = intersections[0];
    draw_filled_circle_mut(&mut image, ((coord.0 as f32 / 10.0) as i32, (coord.1 as f32 / 10.0) as i32), 5i32, colors[0]);
    // let out = resize(&image, 512, 512, image::FilterType::Triangle);
    println!("saving...");
    image.save(path).unwrap();

                    //   .map(|mass_str| mass_str.parse::<usize>().unwrap())
                    //   .collect();
}