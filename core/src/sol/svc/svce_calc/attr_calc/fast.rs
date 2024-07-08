use crate::{
    defs::{AggrKey, AttrVal, EItemCatId},
    sol::svc::svce_calc::{SolAggrMode, SolOp},
    util::StMap,
};

use super::{is_penal, PENALTY_BASE};

pub(in crate::sol::svc::svce_calc) struct SolAttrCalcFast {
    pre_assign: SolAttrAggr,
    pre_mul: SolAttrStack,
    pre_div: SolAttrStack,
    add: SolAttrAggr,
    sub: SolAttrAggr,
    post_mul: SolAttrStack,
    post_div: SolAttrStack,
    post_perc: SolAttrStack,
    post_assign: SolAttrAggr,
    extra_mul: SolAttrAggr,
}
impl SolAttrCalcFast {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            pre_assign: SolAttrAggr::new(),
            pre_mul: SolAttrStack::new(),
            pre_div: SolAttrStack::new(),
            add: SolAttrAggr::new(),
            sub: SolAttrAggr::new(),
            post_mul: SolAttrStack::new(),
            post_div: SolAttrStack::new(),
            post_perc: SolAttrStack::new(),
            post_assign: SolAttrAggr::new(),
            extra_mul: SolAttrAggr::new(),
        }
    }
    pub(in crate::sol::svc::svce_calc) fn add_val(
        &mut self,
        val: AttrVal,
        res_mult: Option<AttrVal>,
        proj_mult: Option<AttrVal>,
        op: &SolOp,
        attr_pen: bool,
        item_cat: &EItemCatId,
        aggr_mode: &SolAggrMode,
    ) {
        match op {
            SolOp::PreAssign => {
                self.pre_assign
                    .add_val(val, res_mult, proj_mult, normalize_noop, diminish_noop, aggr_mode)
            }
            SolOp::PreMul => self.pre_mul.add_val(
                val,
                res_mult,
                proj_mult,
                normalize_noop,
                diminish_mul,
                is_penal(attr_pen, item_cat),
                aggr_mode,
            ),
            SolOp::PreDiv => self.pre_div.add_val(
                val,
                res_mult,
                proj_mult,
                normalize_div,
                diminish_mul,
                is_penal(attr_pen, item_cat),
                aggr_mode,
            ),
            SolOp::Add => self
                .add
                .add_val(val, res_mult, proj_mult, normalize_noop, diminish_basic, aggr_mode),
            SolOp::Sub => self
                .sub
                .add_val(val, res_mult, proj_mult, normalize_sub, diminish_basic, aggr_mode),
            SolOp::PostMul => self.post_mul.add_val(
                val,
                res_mult,
                proj_mult,
                normalize_noop,
                diminish_mul,
                is_penal(attr_pen, item_cat),
                aggr_mode,
            ),
            SolOp::PostMulImmune => {
                self.post_mul
                    .add_val(val, res_mult, proj_mult, normalize_noop, diminish_mul, false, aggr_mode)
            }
            SolOp::PostDiv => self.post_div.add_val(
                val,
                res_mult,
                proj_mult,
                normalize_div,
                diminish_mul,
                is_penal(attr_pen, item_cat),
                aggr_mode,
            ),
            SolOp::PostPerc => self.post_perc.add_val(
                val,
                res_mult,
                proj_mult,
                normalize_perc,
                diminish_mul,
                is_penal(attr_pen, item_cat),
                aggr_mode,
            ),
            SolOp::PostAssign => {
                self.post_assign
                    .add_val(val, res_mult, proj_mult, normalize_noop, diminish_noop, aggr_mode)
            }
            SolOp::ExtraMul => {
                self.extra_mul
                    .add_val(val, res_mult, proj_mult, normalize_noop, diminish_mul, aggr_mode)
            }
        };
    }
    pub(in crate::sol::svc::svce_calc) fn apply_dogma_mods(&mut self, base_val: AttrVal, hig: bool) -> AttrVal {
        let val = apply_assign(base_val, self.pre_assign.get_comb_val(combine_assigns, hig));
        let val = apply_mul(val, self.pre_mul.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_mul(val, self.pre_div.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_add(val, self.add.get_comb_val(combine_adds, hig));
        let val = apply_add(val, self.sub.get_comb_val(combine_adds, hig));
        let val = apply_mul(val, self.post_mul.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_mul(val, self.post_div.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_mul(val, self.post_perc.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_assign(val, self.post_assign.get_comb_val(combine_assigns, hig));
        val
    }
    pub(in crate::sol::svc::svce_calc) fn apply_extra_mods(&mut self, dogma_val: AttrVal, hig: bool) -> AttrVal {
        let val = apply_mul(dogma_val, self.extra_mul.get_comb_val(combine_muls, hig));
        val
    }
}

struct SolAttrStack {
    stacked: SolAttrAggr,
    penalized: SolAttrAggr,
}
impl SolAttrStack {
    fn new() -> Self {
        Self {
            stacked: SolAttrAggr::new(),
            penalized: SolAttrAggr::new(),
        }
    }
    fn add_val<N, D>(
        &mut self,
        val: AttrVal,
        res_mult: Option<AttrVal>,
        proj_mult: Option<AttrVal>,
        normalize_func: N,
        diminish_func: D,
        penalizable: bool,
        aggr_mode: &SolAggrMode,
    ) where
        N: Fn(AttrVal) -> Option<AttrVal>,
        D: Fn(AttrVal, Option<AttrVal>, Option<AttrVal>) -> AttrVal,
    {
        if penalizable {
            self.penalized
                .add_val(val, res_mult, proj_mult, normalize_func, diminish_func, aggr_mode)
        } else {
            self.stacked
                .add_val(val, res_mult, proj_mult, normalize_func, diminish_func, aggr_mode)
        }
    }
    fn get_comb_val<F1, F2>(&mut self, comb_func: F1, pen_func: F2, hig: bool) -> Option<AttrVal>
    where
        F1: Fn(&Vec<AttrVal>, bool) -> Option<AttrVal>,
        F2: Fn(&Vec<AttrVal>, bool) -> Option<AttrVal>,
    {
        if let Some(val) = self.penalized.get_comb_val(pen_func, hig) {
            self.stacked
                .add_val(val, None, None, normalize_noop, diminish_noop, &SolAggrMode::Stack);
        }
        self.stacked.get_comb_val(comb_func, hig)
    }
}

struct SolAttrAggr {
    stack: Vec<AttrVal>,
    aggr_min: StMap<AggrKey, Vec<AttrVal>>,
    aggr_max: StMap<AggrKey, Vec<AttrVal>>,
}
impl SolAttrAggr {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: StMap::new(),
            aggr_max: StMap::new(),
        }
    }
    fn add_val<N, D>(
        &mut self,
        val: AttrVal,
        res_mult: Option<AttrVal>,
        proj_mult: Option<AttrVal>,
        normalize_func: N,
        diminish_func: D,
        aggr_mode: &SolAggrMode,
    ) where
        N: Fn(AttrVal) -> Option<AttrVal>,
        D: Fn(AttrVal, Option<AttrVal>, Option<AttrVal>) -> AttrVal,
    {
        let val = match normalize_func(val) {
            Some(val) => val,
            None => return,
        };
        let val = diminish_func(val, res_mult, proj_mult);
        match aggr_mode {
            SolAggrMode::Stack => self.stack.push(val),
            SolAggrMode::Min(key) => self.aggr_min.entry(*key).or_insert_with(|| Vec::new()).push(val),
            SolAggrMode::Max(key) => self.aggr_max.entry(*key).or_insert_with(|| Vec::new()).push(val),
        }
    }
    fn get_comb_val<F>(&mut self, comb_func: F, high_is_good: bool) -> Option<AttrVal>
    where
        F: Fn(&Vec<AttrVal>, bool) -> Option<AttrVal>,
    {
        // Resolve aggregations
        for vals in self.aggr_min.values() {
            if let Some(val) = get_min(vals) {
                self.stack.push(val);
            }
        }
        for vals in self.aggr_max.values() {
            if let Some(val) = get_max(vals) {
                self.stack.push(val);
            }
        }
        comb_func(&self.stack, high_is_good)
    }
}

// Normalization functions
fn normalize_noop(val: AttrVal) -> Option<AttrVal> {
    Some(val)
}
fn normalize_sub(val: AttrVal) -> Option<AttrVal> {
    Some(-val)
}
fn normalize_div(val: AttrVal) -> Option<AttrVal> {
    if val == 0.0 {
        return None;
    }
    Some(1.0 / val)
}
fn normalize_perc(val: AttrVal) -> Option<AttrVal> {
    Some(1.0 + val / 100.0)
}

// Apply diminishing factors (resistance- and projection-related reductions)
fn diminish_noop(val: AttrVal, _: Option<AttrVal>, _: Option<AttrVal>) -> AttrVal {
    val
}
fn diminish_basic(mut val: AttrVal, res_mult: Option<AttrVal>, proj_mult: Option<AttrVal>) -> AttrVal {
    if let Some(res_mult) = res_mult {
        val *= res_mult;
    }
    if let Some(proj_mult) = proj_mult {
        val *= proj_mult;
    }
    val
}
fn diminish_mul(val: AttrVal, res_mult: Option<AttrVal>, proj_mult: Option<AttrVal>) -> AttrVal {
    if res_mult.is_none() && proj_mult.is_none() {
        return val;
    }
    diminish_basic(val - 1.0, res_mult, proj_mult) + 1.0
}

// Application functions
fn apply_assign(base_val: AttrVal, other_val: Option<AttrVal>) -> AttrVal {
    other_val.unwrap_or_else(|| base_val)
}
fn apply_add(base_val: AttrVal, other_val: Option<AttrVal>) -> AttrVal {
    match other_val {
        Some(other_val) => base_val + other_val,
        None => base_val,
    }
}
fn apply_mul(base_val: AttrVal, other_val: Option<AttrVal>) -> AttrVal {
    match other_val {
        Some(other_val) => base_val * other_val,
        None => base_val,
    }
}

// Regular combination functions
fn combine_assigns(vals: &Vec<AttrVal>, high_is_good: bool) -> Option<AttrVal> {
    match high_is_good {
        true => get_max(vals),
        false => get_min(vals),
    }
}
fn combine_adds(vals: &Vec<AttrVal>, _: bool) -> Option<AttrVal> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().sum())
}
fn combine_muls(vals: &Vec<AttrVal>, _: bool) -> Option<AttrVal> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().product())
}

// Penalized combination functions
fn combine_muls_pen(vals: &Vec<AttrVal>, _: bool) -> Option<AttrVal> {
    penalize_vals(vals.iter().map(|v| *v))
}

// Misc functions
fn get_min(vals: &Vec<AttrVal>) -> Option<AttrVal> {
    vals.iter().min_by(|a, b| a.total_cmp(b)).copied()
}
fn get_max(vals: &Vec<AttrVal>) -> Option<AttrVal> {
    vals.iter().max_by(|a, b| a.total_cmp(b)).copied()
}
fn penalize_vals(vals: impl Iterator<Item = AttrVal>) -> Option<AttrVal> {
    // Gather positive multipliers into one chain, negative into another, with stronger
    // modifications being first
    let mut positive = Vec::new();
    let mut negative = Vec::new();
    for val in vals {
        if val >= 1.0 {
            positive.push(val);
        } else {
            negative.push(val);
        }
    }
    if positive.is_empty() && negative.is_empty() {
        return None;
    }
    positive.sort_by(|a, b| b.partial_cmp(a).unwrap());
    negative.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Some(get_chain_val(positive) * get_chain_val(negative))
}
fn get_chain_val(vals: Vec<AttrVal>) -> AttrVal {
    let mut val = 1.0;
    for (i, mod_val) in vals.iter().enumerate() {
        // Ignore 12th modification and further as non-significant
        if i > 10 {
            break;
        }
        val *= 1.0 + (mod_val - 1.0) * PENALTY_BASE.powi((i as i32).pow(2));
    }
    val
}
