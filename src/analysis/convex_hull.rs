use std::ops::Index;
use defs::pos::Pos;

pub struct ConvexHull {
    points: Vec<Pos>
}

impl ConvexHull {
    pub fn new(mut points: Vec<Pos>) -> ConvexHull {
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
            while ccw(points[m-1], points[m], points[i]) <= 0f32 {
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
        ConvexHull { points: points }
    }

    pub fn cell_overlaps(&self, c: Pos) -> bool {
        self.cell_overlaps_flt(c.row as f32, c.col as f32)
            || self.cell_overlaps_flt((c.row+1) as f32, (c.col  ) as f32)
            || self.cell_overlaps_flt((c.row  ) as f32, (c.col+1) as f32)
            || self.cell_overlaps_flt((c.row+1) as f32, (c.col+1) as f32)
    }

    fn cell_overlaps_flt(&self, cx: f32, cy: f32) -> bool {
        let mut it = self.points.iter();
        let mut p = it.next().expect("empty convex hull");
        while let Some(q) = it.next() {
            let (px, py) = (p.row as f32, p.col as f32);
            let (qx, qy) = (q.row as f32, q.col as f32);

            if ccw_flt(px, py, qx, qy, cx, cy) <= ::std::f32::EPSILON {
                return false;
            }

            p = q;
        }

        return true;
    }

    pub fn area(&self) -> f32 {
        let mut area = 0.0f32;
        let n = self.len();
        for i in 0..n {
            let (px, py) = { let p = self.points[i      ]; (p.row as f32, p.col as f32) };
            let (qx, qy) = { let q = self.points[(i+1)%n]; (q.row as f32, q.col as f32) };

            let dx = px-qx;
            let avg_y = (py+qy)/2.0;

            area += dx * avg_y;
        }
        area
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn cells(&self) -> CellIter {
        let (mut rmin, mut cmin) = (0, 0);
        let (mut rmax, mut cmax) = (127, 127);

        // INCOMPLETE
        //for p in self.points.iter() {
        //    if p.row < 
        //}
    }
}

impl Index<usize> for ConvexHull {
    type Output = Pos;
    fn index(&self, i: usize) -> &Pos {
        &self.points[i]
    }
}

fn ccw(p1: Pos, p2: Pos, p3: Pos) -> f32 {
    ccw_flt(p1.row as f32, p1.col as f32, p2.row as f32, p2.col as f32,
            p3.row as f32, p3.col as f32)
}

fn ccw_flt(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
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

        let hull = ConvexHull::new(p);

        assert_eq!(5, hull.len());
        assert_eq!(Pos::new(0,0), hull[0]);
        assert_eq!(Pos::new(3,0), hull[1]);
        assert_eq!(Pos::new(3,2), hull[2]);
        assert_eq!(Pos::new(2,2), hull[3]);
        assert_eq!(Pos::new(0,1), hull[4]);

        assert!( hull.cell_overlaps(Pos::new(0,0)));
        assert!( hull.cell_overlaps(Pos::new(1,0)));
        assert!( hull.cell_overlaps(Pos::new(2,0)));
        assert!( hull.cell_overlaps(Pos::new(2,1)));
        assert!( hull.cell_overlaps(Pos::new(0,1)));
        assert!( hull.cell_overlaps(Pos::new(1,1)));
        assert!(!hull.cell_overlaps(Pos::new(3,1)));

        assert_eq!(5f32, hull.area());
    }

    #[test]
    fn convex_hull2() {
        let mut p = vec![               Pos::new(0,1), Pos::new(0,2),
                         Pos::new(1,0), Pos::new(1,1), Pos::new(1,2),
                         Pos::new(2,0), Pos::new(2,1), Pos::new(2,2)];

        let hull = ConvexHull::new(p);

        assert_eq!(5, hull.len());
        assert_eq!(Pos::new(1,0), hull[0]);
        assert_eq!(Pos::new(2,0), hull[1]);
        assert_eq!(Pos::new(2,2), hull[2]);
        assert_eq!(Pos::new(0,2), hull[3]);
        assert_eq!(Pos::new(0,1), hull[4]);

        assert!( hull.cell_overlaps(Pos::new(1,0)));
        assert!( hull.cell_overlaps(Pos::new(1,0)));
        assert!( hull.cell_overlaps(Pos::new(0,1)));
        assert!( hull.cell_overlaps(Pos::new(1,1)));
        assert!(!hull.cell_overlaps(Pos::new(2,1)));
        assert!(!hull.cell_overlaps(Pos::new(1,2)));
        assert!(!hull.cell_overlaps(Pos::new(2,2)));

        assert_eq!(3.5, hull.area());
    }

    #[test]
    fn convex_hull3() {
        let mut p = vec![Pos::new(0,0), Pos::new(0,1), Pos::new(0,2), Pos::new(0,3),
                         Pos::new(1,0), Pos::new(1,1), Pos::new(1,2), Pos::new(1,3),
                                                       Pos::new(2,2), Pos::new(2,3)];

        let hull = ConvexHull::new(p);

        assert_eq!(5, hull.len());
        assert_eq!(Pos::new(0,0), hull[0]);
        assert_eq!(Pos::new(1,0), hull[1]);
        assert_eq!(Pos::new(2,2), hull[2]);
        assert_eq!(Pos::new(2,3), hull[3]);
        assert_eq!(Pos::new(0,3), hull[4]);

        assert!( hull.cell_overlaps(Pos::new(0,0)));
        assert!( hull.cell_overlaps(Pos::new(0,1)));
        assert!( hull.cell_overlaps(Pos::new(0,2)));
        assert!( hull.cell_overlaps(Pos::new(1,0)));
        assert!( hull.cell_overlaps(Pos::new(1,1)));
        assert!( hull.cell_overlaps(Pos::new(1,2)));
        assert!(!hull.cell_overlaps(Pos::new(2,2)));

        assert_eq!(5f32, hull.area());
    }
}
