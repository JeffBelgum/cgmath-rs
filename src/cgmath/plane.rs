// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cast::transmute;

use intersect::Intersect;
use point::{Point, Point3};
use ray::Ray3;
use vector::{Vec3, Vec4};
use vector::{Vector, EuclideanVector};

use std::fmt;

/// A 3-dimendional plane formed from the equation: `a*x + b*y + c*z - d = 0`.
///
/// # Fields
///
/// - `n`: a unit vector representing the normal of the plane where:
///   - `n.x`: corresponds to `A` in the plane equation
///   - `n.y`: corresponds to `B` in the plane equation
///   - `n.z`: corresponds to `C` in the plane equation
/// - `d`: the distance value, corresponding to `D` in the plane equation
///
/// # Notes
///
/// The `a*x + b*y + c*z - d = 0` form is preferred over the other common
/// alternative, `a*x + b*y + c*z + d = 0`, because it tends to avoid
/// superfluous negations (see _Real Time Collision Detection_, p. 55).
#[deriving(Clone, Eq)]
pub struct Plane<S> {
    n: Vec3<S>,
    d: S,
}

impl<S: Float> Plane<S> {
    /// Construct a plane from a normal vector and a scalar distance
    pub fn new(n: Vec3<S>, d: S) -> Plane<S> {
        Plane { n: n, d: d }
    }

    /// # Arguments
    ///
    /// - `a`: the `x` component of the normal
    /// - `b`: the `y` component of the normal
    /// - `c`: the `z` component of the normal
    /// - `d`: the plane's distance value
    pub fn from_abcd(a: S, b: S, c: S, d: S) -> Plane<S> {
        Plane { n: Vec3::new(a, b, c), d: d }
    }

    /// Construct a plane from the components of a four-dimensional vector
    pub fn from_vec4(v: Vec4<S>) -> Plane<S> {
        unsafe { transmute(v) }
    }

    /// Constructs a plane that passes through the the three points `a`, `b` and `c`
    pub fn from_points(a: Point3<S>, b: Point3<S>, c: Point3<S>) -> Option<Plane<S>> {
        // create two vectors that run parallel to the plane
        let v0 = b.sub_p(&a);
        let v1 = c.sub_p(&a);

        // find the normal vector that is perpendicular to v1 and v2
        let mut n = v0.cross(&v1);

        if n.approx_eq(&Vec3::zero()) { None }
        else {
            // compute the normal and the distance to the plane
            n.normalize_self();
            let d = -a.dot(&n);

            Some(Plane::new(n, d))
        }
    }
}

impl<S: Float> Intersect<Option<Point3<S>>> for (Plane<S>, Ray3<S>) {
    fn intersection(&self) -> Option<Point3<S>> {
        fail!("Not yet implemented");
    }
}

impl<S: Float> Intersect<Option<Ray3<S>>> for (Plane<S>, Plane<S>) {
    fn intersection(&self) -> Option<Ray3<S>> {
        fail!("Not yet implemented");
    }
}

impl<S: Float> Intersect<Option<Point3<S>>> for (Plane<S>, Plane<S>, Plane<S>) {
    fn intersection(&self) -> Option<Point3<S>> {
        fail!("Not yet implemented");
    }
}

impl<S: Float> ApproxEq<S> for Plane<S> {
    #[inline]
    fn approx_epsilon() -> S {
        // TODO: fix this after static methods are fixed in rustc
        fail!(~"Doesn't work!");
    }

    #[inline]
    fn approx_eq(&self, other: &Plane<S>) -> bool {
        self.n.approx_eq(&other.n) &&
        self.d.approx_eq(&other.d)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Plane<S>, approx_epsilon: &S) -> bool {
        self.n.approx_eq_eps(&other.n, approx_epsilon) &&
        self.d.approx_eq_eps(&other.d, approx_epsilon)
    }
}

impl<S: Clone + fmt::Float> ToStr for Plane<S> {
    fn to_str(&self) -> ~str {
        format!("{:f}x + {:f}y + {:f}z - {:f} = 0",
                self.n.x,
                self.n.y,
                self.n.z,
                self.d)
    }
}
