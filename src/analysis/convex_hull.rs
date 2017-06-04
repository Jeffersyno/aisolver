use defs::pos::Pos;

pub fn convex_hull(points: &mut Vec<Pos>) {
    // find corner with lowest col-coord & swap with index 0
    let (min_i, _) = points.iter().enumerate() .min_by(|&(_,p), &(_,q)| {
       p.col.cmp(&q.col)
    }).unwrap();
    points.swap(0, min_i);
    let o = points[0];

    // filter out points with same col as reference point
    let mut k = 1;
    for i in 1..points.len() {
        if points[i].col == o.col {
            points.swap(i, k);
            k += 1;
        }
    }

    // sort the first k by row
    points[0..k].sort_by(|p, q| p.row.cmp(&q.row));

    // sort by polar angle (by cotan, monotonic over quadrants 1,2)
    // we sort anti-clockwise
    points[k..].sort_by(|&p, &q| {
        let (a, b) = (p-o, q-o);
        let e = (a.row as f32) / (a.col as f32);
        let f = (b.row as f32) / (b.col as f32);
        f.partial_cmp(&e).expect("unexpected NaN or Inf")
    });

    // make points 'circular': first point at 0 and end
    points.push(o);

    // graham's scan
    let (mut i, mut m) = (2, 1);
    while i < points.len() {
        while area(points[m-1], points[m], points[i]) <= 0f32 {
            if m > 1 { m -= 1; }
            else if i < points.len() - 1 {
                points.swap(i, m);
                i += 1;
            } else { break; }
        }
        m += 1;
        points.swap(i, m);
        i += 1;
    }

    points.truncate(m);
}

pub fn in_complex_hull(hull: &[Pos], c: Pos) -> bool {
    //in_complex_hull_flt(hull, (c.row as f32)+0.5, (c.col as f32)+0.5)
    in_complex_hull_flt(hull, c.row as f32, c.col as f32)
        || in_complex_hull_flt(hull, (c.row+1) as f32, c.col as f32)
        || in_complex_hull_flt(hull, c.row as f32, (c.col+1) as f32)
        || in_complex_hull_flt(hull, (c.row+1) as f32, (c.col+1) as f32)
}

fn in_complex_hull_flt(hull: &[Pos], cx: f32, cy: f32) -> bool {
    let mut it = hull.iter();
    let mut p = it.next().expect("empty convex hull");
    while let Some(q) = it.next() {
        let (px, py) = (p.row as f32, p.col as f32);
        let (qx, qy) = (q.row as f32, q.col as f32);

        if area_flt(px, py, qx, qy, cx, cy) <= ::std::f32::EPSILON {
            return false;
        }

        p = q;
    }

    return true;
}

pub fn complex_hull_area(hull: &[Pos]) -> f32 {
    let mut it = hull.iter();
    let mut p = it.next().expect("empty convex hull");
    let mut area = 0.0f32;
    while let Some(q) = it.next() {
        let (px, py) = (p.row as f32, p.col as f32);
        let (qx, qy) = (q.row as f32, q.col as f32);

        let dx = px-qx;
        let avg_y = (py+qy)/2.0;

        area += dx * avg_y;
        p = q;
    }
    area
}

fn area(p1: Pos, p2: Pos, p3: Pos) -> f32 {
    area_flt(p1.row as f32, p1.col as f32, p2.row as f32, p2.col as f32,
              p3.row as f32, p3.col as f32)
}

fn area_flt(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
    0.5 * ((x2-x1)*(y3-y2) - (y2-y1)*(x3-x2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convex_hull1() {
        let mut p = vec![Pos::new(0,0), Pos::new(0,1),
                         Pos::new(1,0), Pos::new(1,1),
                         Pos::new(2,0), Pos::new(2,1), Pos::new(2,2),
                         Pos::new(3,0), Pos::new(3,1), Pos::new(3,2)];

        convex_hull(&mut p);

        assert_eq!(5, p.len());
        assert_eq!(Pos::new(0,0), p[0]);
        assert_eq!(Pos::new(3,0), p[1]);
        assert_eq!(Pos::new(3,2), p[2]);
        assert_eq!(Pos::new(2,2), p[3]);
        assert_eq!(Pos::new(0,1), p[4]);

        assert!( in_complex_hull(&p, Pos::new(0,0)));
        assert!( in_complex_hull(&p, Pos::new(1,0)));
        assert!( in_complex_hull(&p, Pos::new(2,0)));
        assert!( in_complex_hull(&p, Pos::new(2,1)));
        assert!( in_complex_hull(&p, Pos::new(0,1)));
        assert!( in_complex_hull(&p, Pos::new(1,1)));
        assert!(!in_complex_hull(&p, Pos::new(3,1)));

        assert!(5.0-complex_hull_area(&p) == 0.0);
    }

    #[test]
    fn convex_hull2() {
        let mut p = vec![               Pos::new(0,1), Pos::new(0,2),
                         Pos::new(1,0), Pos::new(1,1), Pos::new(1,2),
                         Pos::new(2,0), Pos::new(2,1), Pos::new(2,2)];

        convex_hull(&mut p);

        println!("{:?}", p);

        assert_eq!(5, p.len());
        assert_eq!(Pos::new(1,0), p[0]);
        assert_eq!(Pos::new(2,0), p[1]);
        assert_eq!(Pos::new(2,2), p[2]);
        assert_eq!(Pos::new(0,2), p[3]);
        assert_eq!(Pos::new(0,1), p[4]);

        assert!( in_complex_hull(&p, Pos::new(1,0)));
        assert!( in_complex_hull(&p, Pos::new(1,0)));
        assert!( in_complex_hull(&p, Pos::new(0,1)));
        assert!( in_complex_hull(&p, Pos::new(1,1)));
        assert!(!in_complex_hull(&p, Pos::new(2,1)));
        assert!(!in_complex_hull(&p, Pos::new(1,2)));
        assert!(!in_complex_hull(&p, Pos::new(2,2)));
    }

    #[test]
    fn convex_hull3() {
        let mut p = vec![Pos::new(0,0), Pos::new(0,1), Pos::new(0,2), Pos::new(0,3),
                         Pos::new(1,0), Pos::new(1,1), Pos::new(1,2), Pos::new(1,3),
                                                       Pos::new(2,2), Pos::new(2,3)];

        convex_hull(&mut p);

        assert_eq!(5, p.len());
        assert_eq!(Pos::new(0,0), p[0]);
        assert_eq!(Pos::new(1,0), p[1]);
        assert_eq!(Pos::new(2,2), p[2]);
        assert_eq!(Pos::new(2,3), p[3]);
        assert_eq!(Pos::new(0,3), p[4]);

        assert!( in_complex_hull(&p, Pos::new(0,0)));
        assert!( in_complex_hull(&p, Pos::new(0,1)));
        assert!( in_complex_hull(&p, Pos::new(0,2)));
        assert!( in_complex_hull(&p, Pos::new(1,0)));
        assert!( in_complex_hull(&p, Pos::new(1,1)));
        assert!( in_complex_hull(&p, Pos::new(1,2)));
        assert!(!in_complex_hull(&p, Pos::new(2,2)));
    }
}
