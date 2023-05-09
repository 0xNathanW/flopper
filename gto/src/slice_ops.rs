use std::mem::MaybeUninit;

pub fn average(slice: &[f32], weights: &[f32]) -> f32 {
    let mut weight_sum = 0.0;
    let mut value_sum = 0.0;
    for (&v, &w) in slice.iter().zip(weights.iter()) {
        weight_sum += w as f64;
        value_sum += v as f64 * w as f64;
    }
    (value_sum / weight_sum) as f32
}

// Access the idx'th row of a slice.
pub fn row<T>(slice: &[T], idx: usize, row_size: usize) -> &[T] {
    &slice[idx * row_size..(idx + 1) * row_size]
}

pub fn row_mut<T>(slice: &mut [T], idx: usize, row_size: usize) -> &mut[T] {
    &mut slice[idx * row_size..(idx + 1) * row_size]
}

pub fn apply_swap<T>(slice: &mut [T], swap_list: &[(u16, u16)]) {
    for &(i, j) in swap_list {
        unsafe {
            std::ptr::swap(slice.get_unchecked_mut(i as usize), slice.get_unchecked_mut(j as usize));
        }
    }
}

#[inline]
pub(crate) fn sub_slice(lhs: &mut [f32], rhs: &[f32]) {
    lhs.iter_mut().zip(rhs).for_each(|(l, r)| *l -= *r);
}

#[inline]
pub fn mul_slice(lhs: &mut [f32], rhs: &[f32]) {
    lhs.iter_mut().zip(rhs).for_each(|(l, r)| *l *= *r);
}

// Multiply each element in dst by the corresponding element in src and a scaler.
#[inline]
pub fn mul_slice_scalar_uninit(dst: &mut [MaybeUninit<f32>], src: &[f32], scalar: f32) {
    dst.iter_mut().zip(src).for_each(|(d, s)| {
        d.write(*s * scalar);
    });
}

#[inline]
pub fn div_slice(lhs: &mut [f32], rhs: &[f32], default: f32) {
    lhs.iter_mut()
        .zip(rhs)
        .for_each(|(l, r)| *l = if r.to_bits() == 0 { default } else { *l / *r });
}

#[inline]
pub fn div_slice_uninit(dst: &mut [MaybeUninit<f32>], lhs: &[f32], rhs: &[f32], default: f32) {
    dst.iter_mut()
        .zip(lhs.iter().zip(rhs))
        .for_each(|(d, (l, r))| {
            d.write(if r.to_bits() == 0 { default } else { *l / *r });
        });
}

// Sums elements from src into dst, converting dst from MaybeUninit to T.
#[inline]
pub fn sum_slices_uninit<'a>(dst: &'a mut [MaybeUninit<f32>], src: &[f32]) -> &'a mut [f32] {
    let len = dst.len();
    dst.iter_mut().zip(src).for_each(|(d, s)| {
        d.write(*s);
    });
    let dst = unsafe { &mut *(dst as *mut _ as *mut [f32]) };
    src[len..].chunks_exact(len).for_each(|s| {
        dst.iter_mut().zip(s).for_each(|(d, s)| {
            *d += *s;
        });
    });
    dst
}

#[inline]
pub fn sum_slices_uninit_f64<'a>(dst: &'a mut [MaybeUninit<f64>], src: &[f32]) -> &'a mut [f64] {
    let len = dst.len();
    dst.iter_mut().zip(src).for_each(|(d, s)| {
        d.write(*s as f64);
    });
    let dst = unsafe { &mut *(dst as *mut _ as *mut [f64]) };
    src[len..].chunks_exact(len).for_each(|s| {
        dst.iter_mut().zip(s).for_each(|(d, s)| {
            *d += *s as f64;
        });
    });
    dst
}

// return idx is the larger of dst[idx] and src[idx].
#[inline]
pub fn max_slices_uninit<'a>(dst: &'a mut [MaybeUninit<f32>], src: &[f32]) -> &'a mut [f32] {
    let len = dst.len();
    dst.iter_mut().zip(src).for_each(|(d, s)| {
        d.write(*s);
    });
    let dst = unsafe { &mut *(dst as *mut _ as *mut [f32]) };
    src[len..].chunks_exact(len).for_each(|s| {
        dst.iter_mut().zip(s).for_each(|(d, s)| {
            *d = d.max(*s);
        });
    });
    dst
}

#[inline]
pub fn max_fma_slices_uninit<'a>(dst: &'a mut [MaybeUninit<f32>], src1: &[f32], src2: &[f32]) -> &'a mut [f32] {
    let len = dst.len();
    dst.iter_mut()
        .zip(src1.iter().zip(src2))
        .for_each(|(d, (s1, s2))| {
            d.write(if s2.is_sign_positive() {
                *s1 * *s2
            } else {
                *s1
            });
        });
    let dst = unsafe { &mut *(dst as *mut _ as *mut [f32]) };
    src1[len..]
        .chunks_exact(len)
        .zip(src2[len..].chunks_exact(len))
        .for_each(|(s1, s2)| {
            dst.iter_mut()
                .zip(s1.iter().zip(s2))
                .for_each(|(d, (s1, s2))| {
                    if s2.is_sign_positive() {
                        *d += *s1 * *s2;
                    } else {
                        *d = d.max(*s1);
                    }
                });
        });
    dst
}

#[inline]
pub fn fma_slices_uninit<'a>(dst: &'a mut [MaybeUninit<f32>], src1: &[f32], src2: &[f32]) -> &'a mut [f32] {
    let len = dst.len();
    dst.iter_mut()
        .zip(src1.iter().zip(src2))
        .for_each(|(d, (s1, s2))| {
            d.write(*s1 * *s2);
        });
    let dst = unsafe { &mut *(dst as *mut _ as *mut [f32]) };
    src1[len..]
        .chunks_exact(len)
        .zip(src2[len..].chunks_exact(len))
        .for_each(|(s1, s2)| {
            dst.iter_mut()
                .zip(s1.iter().zip(s2))
                .for_each(|(d, (s1, s2))| {
                    *d += *s1 * *s2;
                });
        });
    dst
}

/// Encodes the `f32` slice to the `i16` slice, and returns the scale.
#[inline]
pub fn encode_signed_slice(dst: &mut [i16], slice: &[f32]) -> f32 {
    let scale = slice_absolute_max(slice);
    let scale_nonzero = if scale == 0.0 { 1.0 } else { scale };
    let encoder = i16::MAX as f32 / scale_nonzero;
    dst.iter_mut()
        .zip(slice)
        .for_each(|(d, s)| *d = unsafe { (s * encoder).round().to_int_unchecked::<i32>() as i16 });
    scale
}

#[inline]
pub fn encode_unsigned_slice(dst: &mut [u16], slice: &[f32]) -> f32 {
    let scale = slice_nonnegative_max(slice);
    let scale_nonzero = if scale == 0.0 { 1.0 } else { scale };
    let encoder = u16::MAX as f32 / scale_nonzero;

    dst.iter_mut().zip(slice).for_each(|(d, s)| {
        *d = unsafe { (s * encoder + 0.49999997).to_int_unchecked::<i32>() as u16 }
    });
    scale
}

// Decodes the encoded `i16` slice to the `f32` slice.
#[inline]
pub fn decode_signed_slice(slice: &[i16], scale: f32) -> Vec<f32> {
    let decoder = scale / i16::MAX as f32;
    let mut result = Vec::<f32>::with_capacity(slice.len());
    let ptr = result.as_mut_ptr();
    unsafe {
        for i in 0..slice.len() {
            *ptr.add(i) = (*slice.get_unchecked(i)) as f32 * decoder;
        }
        result.set_len(slice.len());
    }
    result
}

fn slice_absolute_max(slice: &[f32]) -> f32 {
    if slice.len() < 16 {
        slice.iter().fold(0.0, |a, x| a.max(x.abs()))
    } else {
        let mut tmp: [f32; 8] = slice[..8].try_into().unwrap();
        tmp.iter_mut().for_each(|x| *x = x.abs());
        let mut iter = slice[8..].chunks_exact(8);
        for chunk in iter.by_ref() {
            for i in 0..8 {
                tmp[i] = tmp[i].max(chunk[i].abs());
            }
        }
        let tmpmax = tmp.iter().fold(0.0f32, |a, &x| a.max(x));
        iter.remainder().iter().fold(tmpmax, |a, x| a.max(x.abs()))
    }
}

fn slice_nonnegative_max(slice: &[f32]) -> f32 {
    if slice.len() < 16 {
        slice.iter().fold(0.0, |a, &x| a.max(x))
    } else {
        let mut temp: [f32; 8] = slice[..8].try_into().unwrap();
        let mut iter = slice[8..].chunks_exact(8);
        for chunk in iter.by_ref() {
            for i in 0..8 {
                temp[i] = temp[i].max(chunk[i]);
            }
        }
        let temp_max = temp.iter().fold(0.0_f32, |a, &x| a.max(x));
        iter.remainder().iter().fold(temp_max, |a, &x| a.max(x))
    }
}

pub fn inner_product(src1: &[f32], src2: &[f32]) -> f32 {
    const CHUNK_SIZE: usize = 8;

    let len = src1.len();
    let len_chunk = len / CHUNK_SIZE * CHUNK_SIZE;
    let mut acc = [0.0; CHUNK_SIZE];

    for i in (0..len_chunk).step_by(CHUNK_SIZE) {
        for j in 0..CHUNK_SIZE {
            unsafe {
                let x = *src1.get_unchecked(i + j);
                let y = *src2.get_unchecked(i + j);
                *acc.get_unchecked_mut(j) += (x * y) as f64;
            }
        }
    }

    for i in len_chunk..len {
        unsafe {
            let x = *src1.get_unchecked(i);
            let y = *src2.get_unchecked(i);
            *acc.get_unchecked_mut(0) += (x * y) as f64;
        }
    }

    acc.iter().sum::<f64>() as f32
}

pub fn inner_product_cond(
    src1: &[f32],
    src2: &[f32],
    cond: &[u16],
    threshold: u16,
    less: f32,
    greater: f32,
    equal: f32,
) -> f32 {
    const CHUNK_SIZE: usize = 8;

    let len = src1.len();
    let len_chunk = len / CHUNK_SIZE * CHUNK_SIZE;
    let mut acc = [0.0; CHUNK_SIZE];

    for i in (0..len_chunk).step_by(CHUNK_SIZE) {
        for j in 0..CHUNK_SIZE {
            unsafe {
                let x = *src1.get_unchecked(i + j);
                let y = *src2.get_unchecked(i + j);
                let c = *cond.get_unchecked(i + j);

                // `match` prevents vectorization
                #[allow(clippy::comparison_chain)]
                let z = if c < threshold {
                    less
                } else if c > threshold {
                    greater
                } else {
                    equal
                };

                *acc.get_unchecked_mut(j) += (x * y * z) as f64;
            }
        }
    }

    for i in len_chunk..len {
        unsafe {
            let x = *src1.get_unchecked(i);
            let y = *src2.get_unchecked(i);
            let c = *cond.get_unchecked(i);

            #[allow(clippy::comparison_chain)]
            let z = if c < threshold {
                less
            } else if c > threshold {
                greater
            } else {
                equal
            };

            *acc.get_unchecked_mut(0) += (x * y * z) as f64;
        }
    }

    acc.iter().sum::<f64>() as f32
}

pub fn apply_locking_strategy(dst: &mut [f32], locking: &[f32]) {
    if !locking.is_empty() {
        dst.iter_mut().zip(locking).for_each(|(d, s)| {
            *d *= *s;
        });
    }    
}