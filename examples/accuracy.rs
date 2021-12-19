use std::f64::consts::PI;
use crlibm as C;

fn main() {
    println!("cos({:.17}) ≈ {} (rounded to the nearest)", PI, C::cos_rn(PI));
    let cos_rd = C::cos_rd(PI);
    let cos_ru = C::cos_ru(PI);
    println!("cos({:.17}) ∈ [{}, {}]", PI, cos_rd, cos_ru);
    println!("                           (width: {:.5e})", cos_ru - cos_rd);
    println!("cos(π) =: cospi(1.) ∈ [{}, {}]",
             C::cospi_rd(1.), C::cospi_ru(1.));
    println!("acos(-1)/π =: acospi(-1.) = {}", C::acospi_rn(-1.));

    let exp_rd = C::exp_rd(-1.);  let exp_ru = C::exp_ru(-1.);
    println!("exp(-1.) ∈ [{:.17}, {:.17}]", exp_rd, exp_ru);
    println!("           (width: {:.5e})", exp_ru - exp_rd);
}
