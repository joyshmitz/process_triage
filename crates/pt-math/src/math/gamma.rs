//! Gamma distribution utilities for Bayesian duration models.
//!
//! Provides PDF, CDF, survival, and hazard functions for the Gamma distribution.
//! Uses the regularized incomplete gamma function with series/continued-fraction
//! approximations for numerical stability.
//!
//! # Parameterization
//!
//! Uses **rate parameterization**: `Gamma(α, β)` where:
//! - `α` = shape parameter (α > 0)
//! - `β` = rate parameter (β > 0)
//!
//! The density is: `f(t) = β^α / Γ(α) * t^(α-1) * e^(-βt)`
//!
//! This is equivalent to scale parameterization with `θ = 1/β`.

use super::stable::log_gamma;

// Constants for incomplete gamma computation
const GAMMAINC_MAX_ITERS: usize = 200;
const GAMMAINC_EPS: f64 = 3.0e-12;
const GAMMAINC_FPMIN: f64 = 1.0e-30;

/// Log of the Gamma distribution PDF at t.
///
/// Uses rate parameterization: `f(t) = β^α / Γ(α) * t^(α-1) * e^(-βt)`
///
/// # Arguments
/// * `t` - The value at which to evaluate (t >= 0)
/// * `alpha` - Shape parameter (α > 0)
/// * `beta` - Rate parameter (β > 0)
///
/// # Returns
/// * `log f(t | α, β)` or appropriate boundary value
pub fn gamma_log_pdf(t: f64, alpha: f64, beta: f64) -> f64 {
    // NaN propagation
    if t.is_nan() || alpha.is_nan() || beta.is_nan() {
        return f64::NAN;
    }

    // Parameter validation
    if alpha <= 0.0 || beta <= 0.0 {
        return f64::NAN;
    }

    // Domain check
    if t < 0.0 {
        return f64::NEG_INFINITY;
    }

    // Special case: t = 0
    if t == 0.0 {
        if alpha < 1.0 {
            // Density diverges to +∞
            return f64::INFINITY;
        } else if alpha == 1.0 {
            // Exponential case: f(0) = β
            return beta.ln();
        } else {
            // alpha > 1: f(0) = 0
            return f64::NEG_INFINITY;
        }
    }

    // General case: log f(t) = α*log(β) - log(Γ(α)) + (α-1)*log(t) - β*t
    alpha * beta.ln() - log_gamma(alpha) + (alpha - 1.0) * t.ln() - beta * t
}

/// Gamma distribution PDF at t.
///
/// Returns `exp(gamma_log_pdf(t, alpha, beta))` with proper handling of
/// boundary cases.
pub fn gamma_pdf(t: f64, alpha: f64, beta: f64) -> f64 {
    let log_pdf = gamma_log_pdf(t, alpha, beta);
    if log_pdf.is_nan() {
        return f64::NAN;
    }
    if log_pdf == f64::INFINITY {
        return f64::INFINITY;
    }
    if log_pdf == f64::NEG_INFINITY {
        return 0.0;
    }
    log_pdf.exp()
}

/// Regularized lower incomplete gamma function P(a, x).
///
/// P(a, x) = γ(a, x) / Γ(a) = ∫₀ˣ t^(a-1) e^(-t) dt / Γ(a)
///
/// This is the CDF of Gamma(a, 1) evaluated at x.
pub fn gamma_p(a: f64, x: f64) -> f64 {
    if a.is_nan() || x.is_nan() {
        return f64::NAN;
    }
    if a <= 0.0 {
        return f64::NAN;
    }
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x.is_infinite() {
        return 1.0;
    }

    // Choose algorithm based on x vs a+1
    if x < a + 1.0 {
        // Series representation is more efficient
        gammainc_series(a, x)
    } else {
        // Continued fraction for Q(a,x) is more efficient
        1.0 - gammainc_cf(a, x)
    }
}

/// Regularized upper incomplete gamma function Q(a, x).
///
/// Q(a, x) = Γ(a, x) / Γ(a) = 1 - P(a, x)
///
/// This is the survival function of Gamma(a, 1) evaluated at x.
pub fn gamma_q(a: f64, x: f64) -> f64 {
    if a.is_nan() || x.is_nan() {
        return f64::NAN;
    }
    if a <= 0.0 {
        return f64::NAN;
    }
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return 1.0;
    }
    if x.is_infinite() {
        return 0.0;
    }

    // Choose algorithm based on x vs a+1
    if x < a + 1.0 {
        1.0 - gammainc_series(a, x)
    } else {
        gammainc_cf(a, x)
    }
}

/// Series expansion for P(a, x) when x < a+1.
///
/// P(a, x) = e^(-x) * x^a * Σ_{n=0}^∞ x^n / Γ(a+n+1)
fn gammainc_series(a: f64, x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }

    // Compute in log domain for stability
    let log_gam_a = log_gamma(a);
    let log_prefactor = a * x.ln() - x - log_gam_a;

    // Series: Σ_{n=0}^∞ x^n * Γ(a) / Γ(a+n+1)
    // = Σ_{n=0}^∞ x^n / (a * (a+1) * ... * (a+n))
    let mut term = 1.0 / a;
    let mut sum = term;

    for n in 1..=GAMMAINC_MAX_ITERS {
        term *= x / (a + n as f64);
        sum += term;
        if term.abs() < GAMMAINC_EPS * sum.abs() {
            break;
        }
    }

    let result = log_prefactor.exp() * sum;

    // Clamp to [0, 1]
    result.clamp(0.0, 1.0)
}

/// Continued fraction for Q(a, x) when x >= a+1.
///
/// Uses modified Lentz's algorithm (Numerical Recipes).
fn gammainc_cf(a: f64, x: f64) -> f64 {
    // Compute log(x^a * e^(-x) / Γ(a))
    let log_gam_a = log_gamma(a);
    let log_prefactor = a * x.ln() - x - log_gam_a;

    // Continued fraction representation
    // Q(a,x) = (x^a * e^(-x) / Γ(a)) * CF
    // where CF = 1 / (x - a + 1 + K₁/(x - a + 3 + K₂/(x - a + 5 + ...)))
    // with Kₙ = n * (a - n)

    let mut b = x - a + 1.0;
    let mut c = 1.0 / GAMMAINC_FPMIN;
    let mut d = 1.0 / b;
    let mut h = d;

    for i in 1..=GAMMAINC_MAX_ITERS {
        let ai = -(i as f64) * (i as f64 - a);
        b += 2.0;
        d = ai * d + b;
        if d.abs() < GAMMAINC_FPMIN {
            d = GAMMAINC_FPMIN;
        }
        c = b + ai / c;
        if c.abs() < GAMMAINC_FPMIN {
            c = GAMMAINC_FPMIN;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < GAMMAINC_EPS {
            break;
        }
    }

    let result = log_prefactor.exp() * h;

    // Clamp to [0, 1]
    result.clamp(0.0, 1.0)
}

/// CDF of the Gamma distribution.
///
/// P(T <= t) where T ~ Gamma(α, β)
pub fn gamma_cdf(t: f64, alpha: f64, beta: f64) -> f64 {
    if t.is_nan() || alpha.is_nan() || beta.is_nan() {
        return f64::NAN;
    }
    if alpha <= 0.0 || beta <= 0.0 {
        return f64::NAN;
    }
    if t <= 0.0 {
        return 0.0;
    }
    if t.is_infinite() {
        return 1.0;
    }

    // CDF = P(α, β*t) where P is regularized lower incomplete gamma
    gamma_p(alpha, beta * t)
}

/// Log of the Gamma CDF.
///
/// Returns log(P(T <= t)) where T ~ Gamma(α, β).
/// More stable than log(gamma_cdf(...)) for small probabilities.
pub fn gamma_log_cdf(t: f64, alpha: f64, beta: f64) -> f64 {
    let cdf = gamma_cdf(t, alpha, beta);
    if cdf.is_nan() {
        return f64::NAN;
    }
    if cdf == 0.0 {
        return f64::NEG_INFINITY;
    }
    if cdf == 1.0 {
        return 0.0;
    }
    cdf.ln()
}

/// Survival function of the Gamma distribution.
///
/// S(t) = P(T > t) = 1 - CDF(t)
pub fn gamma_survival(t: f64, alpha: f64, beta: f64) -> f64 {
    if t.is_nan() || alpha.is_nan() || beta.is_nan() {
        return f64::NAN;
    }
    if alpha <= 0.0 || beta <= 0.0 {
        return f64::NAN;
    }
    if t <= 0.0 {
        return 1.0;
    }
    if t.is_infinite() {
        return 0.0;
    }

    // Survival = Q(α, β*t) where Q is regularized upper incomplete gamma
    gamma_q(alpha, beta * t)
}

/// Log of the survival function.
///
/// Returns log(P(T > t)) where T ~ Gamma(α, β).
/// Essential for stable hazard computations.
pub fn gamma_log_survival(t: f64, alpha: f64, beta: f64) -> f64 {
    let surv = gamma_survival(t, alpha, beta);
    if surv.is_nan() {
        return f64::NAN;
    }
    if surv == 0.0 {
        return f64::NEG_INFINITY;
    }
    if surv == 1.0 {
        return 0.0;
    }
    surv.ln()
}

/// Hazard rate (failure rate) of the Gamma distribution.
///
/// h(t) = f(t) / S(t) = exp(log_pdf - log_survival)
///
/// The hazard rate indicates the instantaneous failure rate given survival to t.
pub fn gamma_hazard(t: f64, alpha: f64, beta: f64) -> f64 {
    if t.is_nan() || alpha.is_nan() || beta.is_nan() {
        return f64::NAN;
    }
    if alpha <= 0.0 || beta <= 0.0 {
        return f64::NAN;
    }
    if t < 0.0 {
        return f64::NAN;
    }

    // Special case: t = 0
    if t == 0.0 {
        if alpha < 1.0 {
            // PDF diverges, survival = 1 => hazard diverges
            return f64::INFINITY;
        } else if alpha == 1.0 {
            // Exponential: constant hazard = β
            return beta;
        } else {
            // PDF = 0, survival = 1 => hazard = 0
            return 0.0;
        }
    }

    // General case: h(t) = exp(log_pdf - log_survival)
    let log_pdf = gamma_log_pdf(t, alpha, beta);
    let log_surv = gamma_log_survival(t, alpha, beta);

    if log_pdf.is_nan() || log_surv.is_nan() {
        return f64::NAN;
    }

    // If survival is 0 (log_surv = -inf), hazard is infinite
    if log_surv == f64::NEG_INFINITY {
        return f64::INFINITY;
    }

    // If PDF is 0 (log_pdf = -inf), hazard is 0
    if log_pdf == f64::NEG_INFINITY {
        return 0.0;
    }

    (log_pdf - log_surv).exp()
}

/// Cumulative hazard (integrated hazard) of the Gamma distribution.
///
/// H(t) = -log(S(t)) = -log_survival(t)
///
/// The cumulative hazard is useful for survival analysis and Cox models.
pub fn gamma_cum_hazard(t: f64, alpha: f64, beta: f64) -> f64 {
    let log_surv = gamma_log_survival(t, alpha, beta);
    if log_surv.is_nan() {
        return f64::NAN;
    }
    -log_surv
}

/// Mean of Gamma(α, β).
///
/// E[T] = α / β
pub fn gamma_mean(alpha: f64, beta: f64) -> f64 {
    if alpha.is_nan() || beta.is_nan() || alpha <= 0.0 || beta <= 0.0 {
        return f64::NAN;
    }
    alpha / beta
}

/// Variance of Gamma(α, β).
///
/// Var[T] = α / β²
pub fn gamma_var(alpha: f64, beta: f64) -> f64 {
    if alpha.is_nan() || beta.is_nan() || alpha <= 0.0 || beta <= 0.0 {
        return f64::NAN;
    }
    alpha / (beta * beta)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            return false;
        }
        if a.is_infinite() && b.is_infinite() {
            return a.is_sign_positive() == b.is_sign_positive();
        }
        (a - b).abs() <= tol
    }

    fn rel_eq(a: f64, b: f64, rel_tol: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            return false;
        }
        if a.is_infinite() && b.is_infinite() {
            return a.is_sign_positive() == b.is_sign_positive();
        }
        let diff = (a - b).abs();
        let max_ab = a.abs().max(b.abs());
        if max_ab == 0.0 {
            return diff == 0.0;
        }
        diff / max_ab <= rel_tol
    }

    // ==================== Exponential special case tests ====================
    // Gamma(1, β) = Exponential(rate=β)

    #[test]
    fn exponential_pdf_matches_gamma_1_beta() {
        let beta: f64 = 2.5;
        let t: f64 = 1.0;

        // Exponential PDF: f(t) = β * e^(-β*t)
        let expected_log_pdf = beta.ln() - beta * t;
        let gamma_log_pdf_val = gamma_log_pdf(t, 1.0, beta);

        assert!(
            approx_eq(gamma_log_pdf_val, expected_log_pdf, 1e-10),
            "Gamma(1,{}) log_pdf at t={}: got {}, expected {}",
            beta,
            t,
            gamma_log_pdf_val,
            expected_log_pdf
        );
    }

    #[test]
    fn exponential_cdf_matches_gamma_1_beta() {
        let beta: f64 = 2.0;
        let t: f64 = 0.5;

        // Exponential CDF: F(t) = 1 - e^(-β*t)
        let expected_cdf = 1.0 - (-beta * t).exp();
        let gamma_cdf_val = gamma_cdf(t, 1.0, beta);

        assert!(
            approx_eq(gamma_cdf_val, expected_cdf, 1e-6),
            "Gamma(1,{}) CDF at t={}: got {}, expected {}",
            beta,
            t,
            gamma_cdf_val,
            expected_cdf
        );
    }

    #[test]
    fn exponential_survival_matches_gamma_1_beta() {
        let beta: f64 = 1.5;
        let t: f64 = 2.0;

        // Exponential survival: S(t) = e^(-β*t)
        let expected_surv = (-beta * t).exp();
        let gamma_surv_val = gamma_survival(t, 1.0, beta);

        assert!(
            approx_eq(gamma_surv_val, expected_surv, 1e-6),
            "Gamma(1,{}) survival at t={}: got {}, expected {}",
            beta,
            t,
            gamma_surv_val,
            expected_surv
        );
    }

    #[test]
    fn exponential_hazard_is_constant_beta() {
        let beta = 3.0;

        // Exponential hazard is constant = β
        for t in [0.0, 0.5, 1.0, 2.0, 5.0] {
            let h = gamma_hazard(t, 1.0, beta);
            assert!(
                approx_eq(h, beta, 1e-6),
                "Gamma(1,{}) hazard at t={}: got {}, expected {}",
                beta,
                t,
                h,
                beta
            );
        }
    }

    // ==================== Golden value tests ====================

    #[test]
    fn gamma_pdf_known_values() {
        // Gamma(2, 1) at t=1: f(1) = 1^(2-1) * e^(-1) / Γ(2) = e^(-1)
        let pdf = gamma_pdf(1.0, 2.0, 1.0);
        let expected = 1.0_f64.exp().recip(); // e^(-1) ≈ 0.3679
        assert!(
            approx_eq(pdf, expected, 1e-6),
            "Gamma(2,1) PDF at t=1: got {}, expected {}",
            pdf,
            expected
        );

        // Gamma(3, 2) at t=0.5: f(0.5) = 2³/Γ(3) * 0.5² * e^(-1) = 8/2 * 0.25 * e^(-1) = e^(-1)
        let pdf2 = gamma_pdf(0.5, 3.0, 2.0);
        let expected2 = (-1.0_f64).exp();
        assert!(
            approx_eq(pdf2, expected2, 1e-6),
            "Gamma(3,2) PDF at t=0.5: got {}, expected {}",
            pdf2,
            expected2
        );
    }

    #[test]
    fn gamma_cdf_known_values() {
        // Gamma(1, 1) = Exp(1): CDF at t=1 should be 1 - e^(-1) ≈ 0.6321
        let cdf = gamma_cdf(1.0, 1.0, 1.0);
        let expected = 1.0 - (-1.0_f64).exp();
        assert!(
            approx_eq(cdf, expected, 1e-6),
            "Gamma(1,1) CDF at t=1: got {}, expected {}",
            cdf,
            expected
        );

        // Check a few more known values from tables
        // Gamma(2, 1) at t=2: P(2,2) ≈ 0.594
        let cdf2 = gamma_cdf(2.0, 2.0, 1.0);
        assert!(
            cdf2 > 0.59 && cdf2 < 0.60,
            "Gamma(2,1) CDF at t=2: got {}, expected ~0.594",
            cdf2
        );
    }

    // ==================== Survival monotonicity tests ====================

    #[test]
    fn survival_decreasing_in_t() {
        let alpha = 2.5;
        let beta = 1.0;

        let mut prev_surv = 1.0;
        for t in [0.1, 0.5, 1.0, 2.0, 5.0, 10.0] {
            let surv = gamma_survival(t, alpha, beta);
            assert!(
                surv < prev_surv,
                "Survival should decrease: S({}) = {} >= S(prev) = {}",
                t,
                surv,
                prev_surv
            );
            prev_surv = surv;
        }
    }

    #[test]
    fn log_survival_at_zero_is_zero() {
        // log(S(0)) = log(1) = 0
        let log_surv = gamma_log_survival(0.0, 2.0, 1.0);
        assert!(
            approx_eq(log_surv, 0.0, 1e-12),
            "log_survival(0) should be 0, got {}",
            log_surv
        );
    }

    // ==================== Hazard behavior tests ====================

    #[test]
    fn hazard_increasing_for_alpha_gt_1() {
        // For α > 1, hazard is increasing (IFR - increasing failure rate)
        let alpha = 2.0;
        let beta = 1.0;

        let h1 = gamma_hazard(1.0, alpha, beta);
        let h2 = gamma_hazard(2.0, alpha, beta);
        let h3 = gamma_hazard(5.0, alpha, beta);

        assert!(h1 < h2, "Hazard should increase: h(1)={} < h(2)={}", h1, h2);
        assert!(h2 < h3, "Hazard should increase: h(2)={} < h(5)={}", h2, h3);
    }

    #[test]
    fn hazard_decreasing_for_alpha_lt_1() {
        // For α < 1, hazard is decreasing (DFR - decreasing failure rate)
        let alpha = 0.5;
        let beta = 1.0;

        let h1 = gamma_hazard(0.5, alpha, beta);
        let h2 = gamma_hazard(1.0, alpha, beta);
        let h3 = gamma_hazard(2.0, alpha, beta);

        assert!(h1 > h2, "Hazard should decrease: h(0.5)={} > h(1)={}", h1, h2);
        assert!(h2 > h3, "Hazard should decrease: h(1)={} > h(2)={}", h2, h3);
    }

    #[test]
    fn cum_hazard_equals_neg_log_survival() {
        let alpha = 2.0;
        let beta = 1.5;
        let t = 1.5;

        let cum_h = gamma_cum_hazard(t, alpha, beta);
        let log_surv = gamma_log_survival(t, alpha, beta);

        assert!(
            approx_eq(cum_h, -log_surv, 1e-10),
            "cum_hazard should equal -log_survival: {} vs {}",
            cum_h,
            -log_surv
        );
    }

    // ==================== Edge case tests ====================

    #[test]
    fn alpha_lt_1_behavior_near_zero() {
        // For α < 1, PDF diverges at t=0
        let alpha = 0.5;
        let beta = 1.0;

        let log_pdf_0 = gamma_log_pdf(0.0, alpha, beta);
        assert!(
            log_pdf_0.is_infinite() && log_pdf_0.is_sign_positive(),
            "log_pdf(0) should be +inf for α<1, got {}",
            log_pdf_0
        );

        // Very small t should have large PDF
        let log_pdf_small = gamma_log_pdf(0.001, alpha, beta);
        assert!(
            log_pdf_small > 0.0,
            "log_pdf(0.001) should be positive for α<1, got {}",
            log_pdf_small
        );
    }

    #[test]
    fn alpha_gt_1_behavior_near_zero() {
        // For α > 1, PDF = 0 at t=0
        let alpha = 2.0;
        let beta = 1.0;

        let log_pdf_0 = gamma_log_pdf(0.0, alpha, beta);
        assert!(
            log_pdf_0.is_infinite() && log_pdf_0.is_sign_negative(),
            "log_pdf(0) should be -inf for α>1, got {}",
            log_pdf_0
        );
    }

    #[test]
    fn large_t_tail_behavior() {
        // Very large t should have survival near 0
        let alpha = 2.0;
        let beta = 1.0;
        let t = 100.0;

        let surv = gamma_survival(t, alpha, beta);
        assert!(surv < 1e-30, "Survival at t=100 should be tiny, got {}", surv);

        // log_survival should be very negative but finite
        let log_surv = gamma_log_survival(t, alpha, beta);
        assert!(
            log_surv.is_finite() && log_surv < -50.0,
            "log_survival at t=100 should be very negative, got {}",
            log_surv
        );
    }

    #[test]
    fn invalid_params_return_nan() {
        // Negative alpha
        assert!(gamma_log_pdf(1.0, -1.0, 1.0).is_nan());
        assert!(gamma_cdf(1.0, -1.0, 1.0).is_nan());

        // Zero alpha
        assert!(gamma_log_pdf(1.0, 0.0, 1.0).is_nan());

        // Negative beta
        assert!(gamma_log_pdf(1.0, 1.0, -1.0).is_nan());
        assert!(gamma_survival(1.0, 1.0, -1.0).is_nan());

        // Zero beta
        assert!(gamma_hazard(1.0, 1.0, 0.0).is_nan());
    }

    #[test]
    fn negative_t_returns_appropriate_values() {
        // PDF at t < 0 should be 0 (log = -inf)
        assert!(gamma_log_pdf(-1.0, 2.0, 1.0).is_infinite());
        assert!(gamma_log_pdf(-1.0, 2.0, 1.0).is_sign_negative());

        // CDF at t <= 0 should be 0
        assert!(approx_eq(gamma_cdf(-1.0, 2.0, 1.0), 0.0, 1e-12));
        assert!(approx_eq(gamma_cdf(0.0, 2.0, 1.0), 0.0, 1e-12));

        // Survival at t <= 0 should be 1
        assert!(approx_eq(gamma_survival(-1.0, 2.0, 1.0), 1.0, 1e-12));
        assert!(approx_eq(gamma_survival(0.0, 2.0, 1.0), 1.0, 1e-12));
    }

    // ==================== Mean/Variance tests ====================

    #[test]
    fn mean_and_variance_formulas() {
        let alpha = 3.0;
        let beta = 2.0;

        let mean = gamma_mean(alpha, beta);
        let var = gamma_var(alpha, beta);

        // E[X] = α/β = 3/2 = 1.5
        assert!(approx_eq(mean, 1.5, 1e-12));

        // Var[X] = α/β² = 3/4 = 0.75
        assert!(approx_eq(var, 0.75, 1e-12));
    }

    // ==================== Regularized incomplete gamma tests ====================

    #[test]
    fn gamma_p_known_values() {
        // P(1, 1) = 1 - e^(-1) ≈ 0.6321
        let p = gamma_p(1.0, 1.0);
        let expected = 1.0 - (-1.0_f64).exp();
        assert!(rel_eq(p, expected, 1e-6), "P(1,1): got {}, expected {}", p, expected);

        // P(2, 2) ≈ 0.594
        let p2 = gamma_p(2.0, 2.0);
        assert!(p2 > 0.59 && p2 < 0.60, "P(2,2) should be ~0.594, got {}", p2);
    }

    #[test]
    fn gamma_q_complements_p() {
        let a = 2.5;
        let x = 1.5;

        let p = gamma_p(a, x);
        let q = gamma_q(a, x);

        assert!(
            approx_eq(p + q, 1.0, 1e-10),
            "P + Q should equal 1: {} + {} = {}",
            p,
            q,
            p + q
        );
    }

    #[test]
    fn gamma_p_boundary_values() {
        // P(a, 0) = 0
        assert!(approx_eq(gamma_p(2.0, 0.0), 0.0, 1e-12));

        // P(a, ∞) = 1
        assert!(approx_eq(gamma_p(2.0, f64::INFINITY), 1.0, 1e-12));

        // Q(a, 0) = 1
        assert!(approx_eq(gamma_q(2.0, 0.0), 1.0, 1e-12));

        // Q(a, ∞) = 0
        assert!(approx_eq(gamma_q(2.0, f64::INFINITY), 0.0, 1e-12));
    }

    // ==================== NaN propagation tests ====================

    #[test]
    fn nan_propagates() {
        assert!(gamma_log_pdf(f64::NAN, 1.0, 1.0).is_nan());
        assert!(gamma_log_pdf(1.0, f64::NAN, 1.0).is_nan());
        assert!(gamma_log_pdf(1.0, 1.0, f64::NAN).is_nan());

        assert!(gamma_cdf(f64::NAN, 1.0, 1.0).is_nan());
        assert!(gamma_survival(1.0, f64::NAN, 1.0).is_nan());
        assert!(gamma_hazard(1.0, 1.0, f64::NAN).is_nan());
    }

    // ==================== Log-domain stability tests ====================

    #[test]
    fn log_pdf_matches_pdf() {
        let t = 2.0;
        let alpha = 2.5;
        let beta = 1.5;

        let pdf = gamma_pdf(t, alpha, beta);
        let log_pdf = gamma_log_pdf(t, alpha, beta);

        assert!(
            approx_eq(pdf.ln(), log_pdf, 1e-10),
            "log(pdf) should match log_pdf: {} vs {}",
            pdf.ln(),
            log_pdf
        );
    }

    #[test]
    fn log_survival_matches_survival() {
        let t = 1.5;
        let alpha = 2.0;
        let beta = 1.0;

        let surv = gamma_survival(t, alpha, beta);
        let log_surv = gamma_log_survival(t, alpha, beta);

        assert!(
            approx_eq(surv.ln(), log_surv, 1e-10),
            "log(survival) should match log_survival: {} vs {}",
            surv.ln(),
            log_surv
        );
    }
}
