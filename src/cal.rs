pub(crate) mod config;
pub(crate) mod detect;
pub(crate) mod pnp;

use opencv::calib3d::{rodrigues, solve_pnp, SOLVEPNP_ITERATIVE};
use opencv::core::{Mat, no_array, Point2f};
use opencv::imgproc::point_polygon_test;










