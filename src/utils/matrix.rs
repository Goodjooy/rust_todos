use std::fmt::Display;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

struct Matrix<T: Default, const W: usize, const H: usize> {
    d: [[T; W]; H],
}

impl<T: Default, const H: usize, const W: usize> Matrix<T, W, H> {
    fn from_iter<I: Iterator<Item = T>>(iter: &mut I) -> Option<Self>
    where
        [[T; W]; H]: Default,
    {
        let mut res = Self::default();
        for y in 0..H {
            for x in 0..W {
                let next_data = iter.next()?;
                let t = &mut res[(x, y)];
                *t = next_data;
            }
        }

        Some(res)
    }

    fn from_iter_default<I: Iterator<Item = T>>(iter: &mut I) -> Self
    where
        [[T; W]; H]: Default,
    {
        let mut res = Self::default();
        for y in 0..H {
            for x in 0..W {
                let next_data = iter.next().unwrap_or_default();
                let t = &mut res[(x, y)];
                *t = next_data;
            }
        }
        res
    }
}

impl<T: Default, const H: usize, const W: usize> Matrix<T, W, H> {
    fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        match ((W, H), (x, y)) {
            ((w, h), (x, y)) if x > w || y > h => None,
            ((_w, _h), (x, y)) => Some(&self.d[y][x]),
        }
    }
    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
        match ((W, H), (x, y)) {
            ((w, h), (x, y)) if x > w || y > h => None,
            ((_w, _h), (x, y)) => Some(&mut self.d[y][x]),
        }
    }

    fn get_size(&self) -> (usize, usize) {
        (W, H)
    }
}

impl<T: Default, const H: usize, const W: usize> Matrix<T, W, H> {
    fn dot_mul<const P: usize>(left: Matrix<T, P, H>, right: Matrix<T, W, P>) -> Matrix<T, W, H>
    where
        T: Mul<Output = T> + Add<Output = T> + Clone,
        [[T; W]; H]: Default,
    {
        let mut res: [[T; W]; H] = Default::default();

        for x in 0..W {
            for y in 0..H {
                let data = &mut res[y][x];
                let res = (0..P)
                    .into_iter()
                    .map(|i| {
                        let lt = left.get((i, y)).unwrap().clone();
                        let rt = right.get((x, i)).unwrap().clone();
                        lt * rt
                    })
                    .reduce(|ld, rd| ld + rd)
                    .unwrap_or(T::default());
                *data = res;
            }
        }

        Matrix { d: res }
    }
    fn transpose(self) -> Matrix<T, H, W>
    where
        [[T; H]; W]: Default,
    {
        let mut res: Matrix<T, H, W> = Default::default();
        for (y, xds) in self.d.into_iter().enumerate() {
            for (x, d) in xds.into_iter().enumerate() {
                let t = res.get_mut((y, x)).unwrap();
                *t = d;
            }
        }
        res
    }

    fn do_ops<E, F>(self, func: F) -> Matrix<E, W, H>
    where
        F: Fn(T) -> E,
        [[E; W]; H]: Default,
        E: Default,
    {
        let data = self.d;
        let mut res = Matrix::<E, W, H>::default();

        for (y, yds) in data.into_iter().enumerate() {
            for (x, sd) in yds.into_iter().enumerate() {
                let d = res.get_mut((x, y)).unwrap();
                *d = func(sd)
            }
        }

        res
    }
}

impl<T, const H: usize, const W: usize> Add for Matrix<T, W, H>
where
    T: Default + Add<Output = T>,
    [[T; W]; H]: Default,
{
    type Output = Matrix<T, W, H>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res: Self = Default::default();
        for (y, (lxd, rxd)) in self.d.into_iter().zip(rhs.d.into_iter()).enumerate() {
            for (x, (l, r)) in lxd.into_iter().zip(rxd.into_iter()).enumerate() {
                let t = res.get_mut((x, y)).unwrap();
                *t = l + r
            }
        }
        res
    }
}

impl<T: Default, const H: usize, const W: usize> Default for Matrix<T, W, H>
where
    [[T; W]; H]: Default,
{
    fn default() -> Self {
        Self {
            d: Default::default(),
        }
    }
}
impl<T: Default, const H: usize, const W: usize> Index<(usize, usize)> for Matrix<T, W, H> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).expect("Index Out Of range")
    }
}

impl<T: Default, const H: usize, const W: usize> IndexMut<(usize, usize)> for Matrix<T, W, H> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).expect("Index Out Of range")
    }
}
impl<T: Default + Display, const H: usize, const W: usize> Display for Matrix<T, W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[\n")?;
        for dl in self.d.iter() {
            for d in dl.iter() {
                write!(f, "{}, ", d)?;
            }
            writeln!(f, "")?;
        }
        write!(f, " ]\n")?;
        writeln!(f, "({}, {})", W, H)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn test_generate_from_iter() {
        let mut data = vec![1, 2, 3, 4].into_iter();
        let left = Matrix::<i32, 2, 2>::from_iter(&mut data).unwrap();

        assert_eq!((2, 2), left.get_size());
        assert_eq!(Some(&1), left.get((0, 0)));
        assert_eq!(Some(&2), left.get((1, 0)));
        assert_eq!(Some(&3), left.get((0, 1)));
        assert_eq!(Some(&4), left.get((1, 1)));
    }
    #[test]
    fn test_generate_from_iter_default() {
        let mut data = vec![1, 8].into_iter();
        let left = Matrix::<i32, 2, 2>::from_iter_default(&mut data);

        assert_eq!((2, 2), left.get_size());
        assert_eq!(Some(&1), left.get((0, 0)));
        assert_eq!(Some(&8), left.get((1, 0)));
        assert_eq!(Some(&0), left.get((0, 1)));
        assert_eq!(Some(&0), left.get((1, 1)));
    }

    #[test]
    fn test_dot_mul() {
        let mut data = vec![1, 2, 3, 4].into_iter();
        let left = Matrix::<i32, 2, 2>::from_iter(&mut data).unwrap();
        let mut data = vec![1, 2, 3, 4].into_iter();
        let right = Matrix::<i32, 2, 2>::from_iter_default(&mut data);

        assert_eq!((2, 2), left.get_size());
        assert_eq!(Some(&1), left.get((0, 0)));
        assert_eq!(Some(&2), left.get((1, 0)));
        assert_eq!(Some(&3), left.get((0, 1)));
        assert_eq!(Some(&4), left.get((1, 1)));

        assert_eq!((2, 2), right.get_size());
        assert_eq!(Some(&1), right.get((0, 0)));
        assert_eq!(Some(&2), right.get((1, 0)));
        assert_eq!(Some(&3), right.get((0, 1)));
        assert_eq!(Some(&4), right.get((1, 1)));

        let res = Matrix::dot_mul(left, right);

        assert_eq!((2, 2), res.get_size());
        assert_eq!(Some(&7), res.get((0, 0)));
        assert_eq!(Some(&10), res.get((1, 0)));
        assert_eq!(Some(&15), res.get((0, 1)));
        assert_eq!(Some(&22), res.get((1, 1)));
    }

    #[test]
    fn test_transpose() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let matrix = Matrix::<i32, 2, 3>::from_iter(&mut data.into_iter()).unwrap();

        assert_eq!((2, 3), matrix.get_size());
        assert_eq!(Some(&1), matrix.get((0, 0)));
        assert_eq!(Some(&2), matrix.get((1, 0)));
        assert_eq!(Some(&3), matrix.get((0, 1)));
        assert_eq!(Some(&4), matrix.get((1, 1)));
        assert_eq!(Some(&5), matrix.get((0, 2)));
        assert_eq!(Some(&6), matrix.get((1, 2)));

        let matrix = matrix.transpose();

        assert_eq!((3, 2), matrix.get_size());
        assert_eq!(Some(&1), matrix.get((0, 0)));
        assert_eq!(Some(&2), matrix.get((0, 1)));
        assert_eq!(Some(&3), matrix.get((1, 0)));
        assert_eq!(Some(&4), matrix.get((1, 1)));
        assert_eq!(Some(&5), matrix.get((2, 0)));
        assert_eq!(Some(&6), matrix.get((2, 1)));
    }

    #[test]
    fn test_add() {
        let mut data = vec![1, 2, 3, 4].into_iter();
        let left = Matrix::<i32, 2, 2>::from_iter(&mut data).unwrap();
        let mut data = vec![1, 2, 3, 4].into_iter();
        let right = Matrix::<i32, 2, 2>::from_iter_default(&mut data);

        assert_eq!((2, 2), left.get_size());
        assert_eq!(Some(&1), left.get((0, 0)));
        assert_eq!(Some(&2), left.get((1, 0)));
        assert_eq!(Some(&3), left.get((0, 1)));
        assert_eq!(Some(&4), left.get((1, 1)));

        assert_eq!((2, 2), right.get_size());
        assert_eq!(Some(&1), right.get((0, 0)));
        assert_eq!(Some(&2), right.get((1, 0)));
        assert_eq!(Some(&3), right.get((0, 1)));
        assert_eq!(Some(&4), right.get((1, 1)));

        let res = left + right;

        assert_eq!((2, 2), res.get_size());
        assert_eq!(Some(&2), res.get((0, 0)));
        assert_eq!(Some(&4), res.get((1, 0)));
        assert_eq!(Some(&6), res.get((0, 1)));
        assert_eq!(Some(&8), res.get((1, 1)));
    }

    #[test]
    fn test_do_ops() {
        let mut data = vec![1, 2, 3, 4].into_iter();
        let left = Matrix::<i32, 2, 2>::from_iter(&mut data).unwrap();

        assert_eq!((2, 2), left.get_size());
        assert_eq!(Some(&1), left.get((0, 0)));
        assert_eq!(Some(&2), left.get((1, 0)));
        assert_eq!(Some(&3), left.get((0, 1)));
        assert_eq!(Some(&4), left.get((1, 1)));

        let res = left.do_ops(|d| d * 10);

        assert_eq!((2, 2), res.get_size());
        assert_eq!(Some(&10), res.get((0, 0)));
        assert_eq!(Some(&20), res.get((1, 0)));
        assert_eq!(Some(&30), res.get((0, 1)));
        assert_eq!(Some(&40), res.get((1, 1)));
    }
}
