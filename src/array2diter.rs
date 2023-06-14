pub struct Array2DIterator<'a, T> {
    array: &'a [T],
    width: usize,
    index: usize,
}

impl<'a, T> Array2DIterator<'a, T> {
    pub fn new(array: &'a [T], width: usize) -> Array2DIterator<'a, T> {
        Array2DIterator {
            array,
            width,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for Array2DIterator<'a, T> {
    type Item = (&'a T, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.len() {
            return None;
        }

        let x = self.index % self.width;
        let y = self.index / self.width;
        let item = &self.array[self.index];

        self.index += 1;

        Some((item, x, y))
    }
}

pub struct Array2DRangeIterator<T> {
    range: std::ops::Range<usize>,
    width: usize,
    x: usize,
    y: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Array2DRangeIterator<T> {
    pub fn new(range: std::ops::Range<usize>, width: usize) -> Array2DRangeIterator<T> {
        Array2DRangeIterator {
            range,
            width,
            x: 0,
            y: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Iterator for Array2DRangeIterator<T> {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.range.len() / self.width {
            return None;
        }

        let index = self.y * self.width + self.x;

        let current_x = self.x;
        let current_y = self.y;

        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        Some((self.range.start + index, current_x, current_y))
    }
}
