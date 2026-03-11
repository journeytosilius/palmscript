//! Hilbert-transform cycle indicators and MAMA family state.

use std::f64::consts::PI;

use crate::diagnostic::RuntimeError;
use crate::types::Value;
use crate::vm::SeriesBuffer;

const RAD_TO_DEG: f64 = 180.0 / PI;
const DEG_TO_RAD: f64 = PI / 180.0;
const DEG_TO_RAD_BY_360: f64 = PI / 45.0;
const HILBERT_A: f64 = 0.0962;
const HILBERT_B: f64 = 0.5769;
const HILBERT_SHORT_LOOKBACK: usize = 32;
const HILBERT_LONG_LOOKBACK: usize = 63;
const HILBERT_SHORT_START: usize = 12;
const HILBERT_LONG_START: usize = 37;
const SMOOTH_PRICE_SIZE: usize = 50;

#[derive(Clone, Debug, Default)]
struct HilbertComponent {
    odd: [f64; 3],
    even: [f64; 3],
    prev_odd: f64,
    prev_even: f64,
    prev_input_odd: f64,
    prev_input_even: f64,
}

impl HilbertComponent {
    fn update_odd(&mut self, input: f64, hilbert_idx: usize, adjusted_prev_period: f64) -> f64 {
        let hilbert_temp = HILBERT_A * input;
        let mut value = -self.odd[hilbert_idx];
        self.odd[hilbert_idx] = hilbert_temp;
        value += hilbert_temp;
        value -= self.prev_odd;
        self.prev_odd = HILBERT_B * self.prev_input_odd;
        value += self.prev_odd;
        self.prev_input_odd = input;
        value * adjusted_prev_period
    }

    fn update_even(&mut self, input: f64, hilbert_idx: usize, adjusted_prev_period: f64) -> f64 {
        let hilbert_temp = HILBERT_A * input;
        let mut value = -self.even[hilbert_idx];
        self.even[hilbert_idx] = hilbert_temp;
        value += hilbert_temp;
        value -= self.prev_even;
        self.prev_even = HILBERT_B * self.prev_input_even;
        value += self.prev_even;
        self.prev_input_even = input;
        value * adjusted_prev_period
    }
}

#[derive(Clone, Debug)]
struct SmoothPriceHistory {
    values: [f64; SMOOTH_PRICE_SIZE],
    head: usize,
}

impl Default for SmoothPriceHistory {
    fn default() -> Self {
        Self {
            values: [0.0; SMOOTH_PRICE_SIZE],
            head: 0,
        }
    }
}

impl SmoothPriceHistory {
    fn push(&mut self, value: f64) {
        self.values[self.head] = value;
    }

    fn current(&self) -> f64 {
        self.values[self.head]
    }

    fn advance(&mut self) {
        self.head = (self.head + 1) % SMOOTH_PRICE_SIZE;
    }
}

#[derive(Clone, Debug)]
struct HilbertCycleCore {
    start_bar: usize,
    processed_bars: usize,
    hilbert_idx: usize,
    detrender: HilbertComponent,
    q1: HilbertComponent,
    ji: HilbertComponent,
    jq: HilbertComponent,
    prev_q2: f64,
    prev_i2: f64,
    re: f64,
    im: f64,
    period: f64,
    smooth_period: f64,
    i1_for_odd_prev2: f64,
    i1_for_odd_prev3: f64,
    i1_for_even_prev2: f64,
    i1_for_even_prev3: f64,
    smooth_price: SmoothPriceHistory,
}

impl HilbertCycleCore {
    fn new(start_bar: usize) -> Self {
        Self {
            start_bar,
            processed_bars: 0,
            hilbert_idx: 0,
            detrender: HilbertComponent::default(),
            q1: HilbertComponent::default(),
            ji: HilbertComponent::default(),
            jq: HilbertComponent::default(),
            prev_q2: 0.0,
            prev_i2: 0.0,
            re: 0.0,
            im: 0.0,
            period: 0.0,
            smooth_period: 0.0,
            i1_for_odd_prev2: 0.0,
            i1_for_odd_prev3: 0.0,
            i1_for_even_prev2: 0.0,
            i1_for_even_prev3: 0.0,
            smooth_price: SmoothPriceHistory::default(),
        }
    }

    fn step(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Option<CycleStep>, RuntimeError> {
        let bar_index = self.processed_bars;
        self.processed_bars += 1;

        if bar_index < self.start_bar {
            return Ok(None);
        }

        let Some(smoothed_price) = weighted_price_wma(buffer, pc)? else {
            return Ok(None);
        };
        self.smooth_price.push(smoothed_price);

        let adjusted_prev_period = (0.075 * self.period) + 0.54;
        let (quadrature, in_phase) = if bar_index.is_multiple_of(2) {
            let detrender =
                self.detrender
                    .update_even(smoothed_price, self.hilbert_idx, adjusted_prev_period);
            let quadrature = self
                .q1
                .update_even(detrender, self.hilbert_idx, adjusted_prev_period);
            let in_phase = self.i1_for_even_prev3;
            let j_i = self
                .ji
                .update_even(in_phase, self.hilbert_idx, adjusted_prev_period);
            let j_q = self
                .jq
                .update_even(quadrature, self.hilbert_idx, adjusted_prev_period);
            self.hilbert_idx = (self.hilbert_idx + 1) % 3;

            let q2 = (0.2 * (quadrature + j_i)) + (0.8 * self.prev_q2);
            let i2 = (0.2 * (in_phase - j_q)) + (0.8 * self.prev_i2);
            self.update_period(q2, i2);

            self.i1_for_odd_prev3 = self.i1_for_odd_prev2;
            self.i1_for_odd_prev2 = detrender;
            (quadrature, in_phase)
        } else {
            let detrender =
                self.detrender
                    .update_odd(smoothed_price, self.hilbert_idx, adjusted_prev_period);
            let quadrature = self
                .q1
                .update_odd(detrender, self.hilbert_idx, adjusted_prev_period);
            let in_phase = self.i1_for_odd_prev3;
            let j_i = self
                .ji
                .update_odd(in_phase, self.hilbert_idx, adjusted_prev_period);
            let j_q = self
                .jq
                .update_odd(quadrature, self.hilbert_idx, adjusted_prev_period);

            let q2 = (0.2 * (quadrature + j_i)) + (0.8 * self.prev_q2);
            let i2 = (0.2 * (in_phase - j_q)) + (0.8 * self.prev_i2);
            self.update_period(q2, i2);

            self.i1_for_even_prev3 = self.i1_for_even_prev2;
            self.i1_for_even_prev2 = detrender;
            (quadrature, in_phase)
        };

        Ok(Some(CycleStep {
            bar_index,
            in_phase,
            quadrature,
            smooth_period: self.smooth_period,
        }))
    }

    fn update_period(&mut self, q2: f64, i2: f64) {
        self.re = (0.2 * ((i2 * self.prev_i2) + (q2 * self.prev_q2))) + (0.8 * self.re);
        self.im = (0.2 * ((i2 * self.prev_q2) - (q2 * self.prev_i2))) + (0.8 * self.im);
        self.prev_q2 = q2;
        self.prev_i2 = i2;

        let previous_period = self.period;
        if self.im != 0.0 && self.re != 0.0 {
            self.period = 360.0 / ((self.im / self.re).atan() * RAD_TO_DEG);
        }

        let upper = 1.5 * previous_period;
        if self.period > upper {
            self.period = upper;
        }
        let lower = 0.67 * previous_period;
        if self.period < lower {
            self.period = lower;
        }
        self.period = self.period.clamp(6.0, 50.0);
        self.period = (0.2 * self.period) + (0.8 * previous_period);
        self.smooth_period = (0.33 * self.period) + (0.67 * self.smooth_period);
    }

    fn dominant_cycle_phase(&self) -> f64 {
        let dc_period = self.smooth_period + 0.5;
        let dc_period_int = dc_period as usize;
        if dc_period_int == 0 {
            return 0.0;
        }

        let mut real_part = 0.0;
        let mut imag_part = 0.0;
        let mut idx = self.smooth_price.head;
        for sample in 0..dc_period_int {
            let angle = ((sample as f64) * DEG_TO_RAD_BY_360) / dc_period_int as f64;
            let value = self.smooth_price.values[idx];
            real_part += angle.sin() * value;
            imag_part += angle.cos() * value;
            idx = if idx == 0 {
                SMOOTH_PRICE_SIZE - 1
            } else {
                idx - 1
            };
        }

        let mut dc_phase = 0.0;
        let imag_abs = imag_part.abs();
        if imag_abs > 0.0 {
            dc_phase = (real_part / imag_part).atan() * RAD_TO_DEG;
        } else if imag_abs <= 0.01 {
            if real_part < 0.0 {
                dc_phase -= 90.0;
            } else if real_part > 0.0 {
                dc_phase += 90.0;
            }
        }

        dc_phase += 90.0;
        dc_phase += 360.0 / self.smooth_period;
        if imag_part < 0.0 {
            dc_phase += 180.0;
        }
        if dc_phase > 315.0 {
            dc_phase -= 360.0;
        }
        dc_phase
    }

    fn current_smooth_price(&self) -> f64 {
        self.smooth_price.current()
    }

    fn advance_smooth_price(&mut self) {
        self.smooth_price.advance();
    }
}

#[derive(Clone, Copy, Debug)]
struct CycleStep {
    bar_index: usize,
    in_phase: f64,
    quadrature: f64,
    smooth_period: f64,
}

#[derive(Clone, Debug)]
struct MamaCoreState {
    start_bar: usize,
    processed_bars: usize,
    hilbert_idx: usize,
    detrender: HilbertComponent,
    q1: HilbertComponent,
    ji: HilbertComponent,
    jq: HilbertComponent,
    prev_q2: f64,
    prev_i2: f64,
    re: f64,
    im: f64,
    period: f64,
    i1_for_odd_prev2: f64,
    i1_for_odd_prev3: f64,
    i1_for_even_prev2: f64,
    i1_for_even_prev3: f64,
    prev_phase: f64,
    mama: f64,
    fama: f64,
    fast_limit: f64,
    slow_limit: f64,
}

impl MamaCoreState {
    fn new(fast_limit: f64, slow_limit: f64) -> Self {
        Self {
            start_bar: HILBERT_SHORT_START,
            processed_bars: 0,
            hilbert_idx: 0,
            detrender: HilbertComponent::default(),
            q1: HilbertComponent::default(),
            ji: HilbertComponent::default(),
            jq: HilbertComponent::default(),
            prev_q2: 0.0,
            prev_i2: 0.0,
            re: 0.0,
            im: 0.0,
            period: 0.0,
            i1_for_odd_prev2: 0.0,
            i1_for_odd_prev3: 0.0,
            i1_for_even_prev2: 0.0,
            i1_for_even_prev3: 0.0,
            prev_phase: 0.0,
            mama: 0.0,
            fama: 0.0,
            fast_limit,
            slow_limit,
        }
    }

    fn step(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Option<(usize, f64, f64)>, RuntimeError> {
        let bar_index = self.processed_bars;
        self.processed_bars += 1;

        if bar_index < self.start_bar {
            return Ok(None);
        }

        let Some(smoothed_price) = weighted_price_wma(buffer, pc)? else {
            return Ok(None);
        };
        let today_value = expect_buffer_f64(buffer, 0, pc)?;
        let adjusted_prev_period = (0.075 * self.period) + 0.54;

        let alpha_angle = if bar_index.is_multiple_of(2) {
            let detrender =
                self.detrender
                    .update_even(smoothed_price, self.hilbert_idx, adjusted_prev_period);
            let quadrature = self
                .q1
                .update_even(detrender, self.hilbert_idx, adjusted_prev_period);
            let in_phase = self.i1_for_even_prev3;
            let j_i = self
                .ji
                .update_even(in_phase, self.hilbert_idx, adjusted_prev_period);
            let j_q = self
                .jq
                .update_even(quadrature, self.hilbert_idx, adjusted_prev_period);
            self.hilbert_idx = (self.hilbert_idx + 1) % 3;

            let q2 = (0.2 * (quadrature + j_i)) + (0.8 * self.prev_q2);
            let i2 = (0.2 * (in_phase - j_q)) + (0.8 * self.prev_i2);

            self.i1_for_odd_prev3 = self.i1_for_odd_prev2;
            self.i1_for_odd_prev2 = detrender;
            self.update_mama(today_value, in_phase, quadrature);
            self.update_period(q2, i2);
            bar_index
        } else {
            let detrender =
                self.detrender
                    .update_odd(smoothed_price, self.hilbert_idx, adjusted_prev_period);
            let quadrature = self
                .q1
                .update_odd(detrender, self.hilbert_idx, adjusted_prev_period);
            let in_phase = self.i1_for_odd_prev3;
            let j_i = self
                .ji
                .update_odd(in_phase, self.hilbert_idx, adjusted_prev_period);
            let j_q = self
                .jq
                .update_odd(quadrature, self.hilbert_idx, adjusted_prev_period);

            let q2 = (0.2 * (quadrature + j_i)) + (0.8 * self.prev_q2);
            let i2 = (0.2 * (in_phase - j_q)) + (0.8 * self.prev_i2);

            self.i1_for_even_prev3 = self.i1_for_even_prev2;
            self.i1_for_even_prev2 = detrender;
            self.update_mama(today_value, in_phase, quadrature);
            self.update_period(q2, i2);
            bar_index
        };

        Ok(Some((alpha_angle, self.mama, self.fama)))
    }

    fn update_mama(&mut self, today_value: f64, in_phase: f64, quadrature: f64) {
        let phase = if in_phase != 0.0 {
            (quadrature / in_phase).atan() * RAD_TO_DEG
        } else {
            0.0
        };

        let mut delta_phase = self.prev_phase - phase;
        self.prev_phase = phase;
        if delta_phase < 1.0 {
            delta_phase = 1.0;
        }

        let mut alpha = if delta_phase > 1.0 {
            let candidate = self.fast_limit / delta_phase;
            if candidate < self.slow_limit {
                self.slow_limit
            } else {
                candidate
            }
        } else {
            self.fast_limit
        };

        self.mama = (alpha * today_value) + ((1.0 - alpha) * self.mama);
        alpha *= 0.5;
        self.fama = (alpha * self.mama) + ((1.0 - alpha) * self.fama);
    }

    fn update_period(&mut self, q2: f64, i2: f64) {
        self.re = (0.2 * ((i2 * self.prev_i2) + (q2 * self.prev_q2))) + (0.8 * self.re);
        self.im = (0.2 * ((i2 * self.prev_q2) - (q2 * self.prev_i2))) + (0.8 * self.im);
        self.prev_q2 = q2;
        self.prev_i2 = i2;

        let previous_period = self.period;
        if self.im != 0.0 && self.re != 0.0 {
            self.period = 360.0 / ((self.im / self.re).atan() * RAD_TO_DEG);
        }

        let upper = 1.5 * previous_period;
        if self.period > upper {
            self.period = upper;
        }
        let lower = 0.67 * previous_period;
        if self.period < lower {
            self.period = lower;
        }
        self.period = self.period.clamp(6.0, 50.0);
        self.period = (0.2 * self.period) + (0.8 * previous_period);
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HtDcPeriodState {
    core: HilbertCycleCore,
    last_seen_version: u64,
    cached_output: Value,
}

impl HtDcPeriodState {
    pub(crate) fn new() -> Self {
        Self {
            core: HilbertCycleCore::new(HILBERT_SHORT_START),
            last_seen_version: 0,
            cached_output: Value::NA,
        }
    }

    pub(crate) fn update(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if buffer.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = buffer.version();

        let result = match self.core.step(buffer, pc)? {
            Some(step) if step.bar_index >= HILBERT_SHORT_LOOKBACK => {
                Value::F64(step.smooth_period)
            }
            _ => Value::NA,
        };
        self.cached_output = result.clone();
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HtDcPhaseState {
    core: HilbertCycleCore,
    last_seen_version: u64,
    cached_output: Value,
}

impl HtDcPhaseState {
    pub(crate) fn new() -> Self {
        Self {
            core: HilbertCycleCore::new(HILBERT_LONG_START),
            last_seen_version: 0,
            cached_output: Value::NA,
        }
    }

    pub(crate) fn update(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if buffer.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = buffer.version();

        let result = match self.core.step(buffer, pc)? {
            Some(step) if step.bar_index >= HILBERT_LONG_LOOKBACK => {
                Value::F64(self.core.dominant_cycle_phase())
            }
            _ => Value::NA,
        };
        self.core.advance_smooth_price();
        self.cached_output = result.clone();
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HtPhasorState {
    core: HilbertCycleCore,
    last_seen_version: u64,
    cached_output: Value,
}

impl HtPhasorState {
    pub(crate) fn new() -> Self {
        Self {
            core: HilbertCycleCore::new(HILBERT_SHORT_START),
            last_seen_version: 0,
            cached_output: na_tuple2(),
        }
    }

    pub(crate) fn update(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if buffer.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = buffer.version();

        let result = match self.core.step(buffer, pc)? {
            Some(step) if step.bar_index >= HILBERT_SHORT_LOOKBACK => {
                tuple2(step.in_phase, step.quadrature)
            }
            _ => na_tuple2(),
        };
        self.cached_output = result.clone();
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HtSineState {
    core: HilbertCycleCore,
    last_seen_version: u64,
    cached_output: Value,
}

impl HtSineState {
    pub(crate) fn new() -> Self {
        Self {
            core: HilbertCycleCore::new(HILBERT_LONG_START),
            last_seen_version: 0,
            cached_output: na_tuple2(),
        }
    }

    pub(crate) fn update(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if buffer.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = buffer.version();

        let result = match self.core.step(buffer, pc)? {
            Some(step) if step.bar_index >= HILBERT_LONG_LOOKBACK => {
                let phase = self.core.dominant_cycle_phase();
                tuple2(
                    (phase * DEG_TO_RAD).sin(),
                    ((phase + 45.0) * DEG_TO_RAD).sin(),
                )
            }
            _ => na_tuple2(),
        };
        self.core.advance_smooth_price();
        self.cached_output = result.clone();
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HtTrendlineState {
    core: HilbertCycleCore,
    itrend1: f64,
    itrend2: f64,
    itrend3: f64,
    last_seen_version: u64,
    cached_output: Value,
}

impl HtTrendlineState {
    pub(crate) fn new() -> Self {
        Self {
            core: HilbertCycleCore::new(HILBERT_LONG_START),
            itrend1: 0.0,
            itrend2: 0.0,
            itrend3: 0.0,
            last_seen_version: 0,
            cached_output: Value::NA,
        }
    }

    pub(crate) fn update(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if buffer.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = buffer.version();

        let result = match self.core.step(buffer, pc)? {
            Some(step) if step.bar_index >= HILBERT_LONG_LOOKBACK => {
                let trendline = self.compute_trendline(buffer, pc)?;
                Value::F64(trendline)
            }
            _ => Value::NA,
        };
        self.core.advance_smooth_price();
        self.cached_output = result.clone();
        Ok(result)
    }

    fn compute_trendline(&mut self, buffer: &SeriesBuffer, pc: usize) -> Result<f64, RuntimeError> {
        let dc_period = self.core.smooth_period + 0.5;
        let dc_period_int = dc_period as usize;
        let mut average = 0.0;
        for offset in 0..dc_period_int {
            average += expect_buffer_f64(buffer, offset, pc)?;
        }
        if dc_period_int > 0 {
            average /= dc_period_int as f64;
        }
        let trendline =
            (4.0 * average + 3.0 * self.itrend1 + 2.0 * self.itrend2 + self.itrend3) / 10.0;
        self.itrend3 = self.itrend2;
        self.itrend2 = self.itrend1;
        self.itrend1 = average;
        Ok(trendline)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HtTrendModeState {
    core: HilbertCycleCore,
    itrend1: f64,
    itrend2: f64,
    itrend3: f64,
    days_in_trend: usize,
    prev_dc_phase: f64,
    prev_sine: f64,
    prev_lead_sine: f64,
    last_seen_version: u64,
    cached_output: Value,
}

impl HtTrendModeState {
    pub(crate) fn new() -> Self {
        Self {
            core: HilbertCycleCore::new(HILBERT_LONG_START),
            itrend1: 0.0,
            itrend2: 0.0,
            itrend3: 0.0,
            days_in_trend: 0,
            prev_dc_phase: 0.0,
            prev_sine: 0.0,
            prev_lead_sine: 0.0,
            last_seen_version: 0,
            cached_output: Value::NA,
        }
    }

    pub(crate) fn update(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if buffer.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = buffer.version();

        let result = match self.core.step(buffer, pc)? {
            Some(step) if step.bar_index >= HILBERT_LONG_LOOKBACK => {
                let phase = self.core.dominant_cycle_phase();
                let sine = (phase * DEG_TO_RAD).sin();
                let lead_sine = ((phase + 45.0) * DEG_TO_RAD).sin();
                let trendline = self.compute_trendline(buffer, pc)?;
                let trend = self.compute_trend(step, phase, sine, lead_sine, trendline);
                Value::F64(if trend { 1.0 } else { 0.0 })
            }
            _ => Value::NA,
        };
        self.core.advance_smooth_price();
        self.cached_output = result.clone();
        Ok(result)
    }

    fn compute_trendline(&mut self, buffer: &SeriesBuffer, pc: usize) -> Result<f64, RuntimeError> {
        let dc_period = self.core.smooth_period + 0.5;
        let dc_period_int = dc_period as usize;
        let mut average = 0.0;
        for offset in 0..dc_period_int {
            average += expect_buffer_f64(buffer, offset, pc)?;
        }
        if dc_period_int > 0 {
            average /= dc_period_int as f64;
        }
        let trendline =
            (4.0 * average + 3.0 * self.itrend1 + 2.0 * self.itrend2 + self.itrend3) / 10.0;
        self.itrend3 = self.itrend2;
        self.itrend2 = self.itrend1;
        self.itrend1 = average;
        Ok(trendline)
    }

    fn compute_trend(
        &mut self,
        step: CycleStep,
        dc_phase: f64,
        sine: f64,
        lead_sine: f64,
        trendline: f64,
    ) -> bool {
        let mut trend = true;
        if ((sine > lead_sine) && (self.prev_sine <= self.prev_lead_sine))
            || ((sine < lead_sine) && (self.prev_sine >= self.prev_lead_sine))
        {
            self.days_in_trend = 0;
            trend = false;
        }

        self.days_in_trend += 1;
        if (self.days_in_trend as f64) < (0.5 * step.smooth_period) {
            trend = false;
        }

        let delta_phase = dc_phase - self.prev_dc_phase;
        if step.smooth_period != 0.0
            && delta_phase > (0.67 * 360.0 / step.smooth_period)
            && delta_phase < (1.5 * 360.0 / step.smooth_period)
        {
            trend = false;
        }

        let smooth_price = self.core.current_smooth_price();
        if trendline != 0.0 && ((smooth_price - trendline) / trendline).abs() >= 0.015 {
            trend = true;
        }

        self.prev_dc_phase = dc_phase;
        self.prev_sine = sine;
        self.prev_lead_sine = lead_sine;
        trend
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MamaState {
    core: MamaCoreState,
    last_seen_version: u64,
    cached_output: Value,
}

impl MamaState {
    pub(crate) fn new(fast_limit: f64, slow_limit: f64) -> Self {
        Self {
            core: MamaCoreState::new(fast_limit, slow_limit),
            last_seen_version: 0,
            cached_output: na_tuple2(),
        }
    }

    pub(crate) fn update(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if buffer.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = buffer.version();

        let result = match self.core.step(buffer, pc)? {
            Some((bar_index, mama, fama)) if bar_index >= HILBERT_SHORT_LOOKBACK => {
                tuple2(mama, fama)
            }
            _ => na_tuple2(),
        };
        self.cached_output = result.clone();
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MamaAverageState {
    inner: MamaState,
    last_seen_version: u64,
    cached_output: Value,
}

impl MamaAverageState {
    pub(crate) fn new() -> Self {
        Self {
            inner: MamaState::new(0.5, 0.05),
            last_seen_version: 0,
            cached_output: Value::NA,
        }
    }

    pub(crate) fn update(
        &mut self,
        buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if buffer.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = buffer.version();
        self.cached_output = match self.inner.update(buffer, pc)? {
            Value::Tuple2(values) => *values[0].clone(),
            Value::NA => Value::NA,
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "tuple2",
                    found: other.type_name(),
                })
            }
        };
        Ok(self.cached_output.clone())
    }
}

fn weighted_price_wma(buffer: &SeriesBuffer, pc: usize) -> Result<Option<f64>, RuntimeError> {
    let mut weighted_sum = 0.0;
    let weights = [4.0, 3.0, 2.0, 1.0];
    for (offset, weight) in weights.into_iter().enumerate() {
        let value = match buffer.get(offset) {
            Value::F64(value) => value,
            Value::NA => return Ok(None),
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "f64",
                    found: other.type_name(),
                })
            }
        };
        weighted_sum += value * weight;
    }
    Ok(Some(weighted_sum * 0.1))
}

fn expect_buffer_f64(buffer: &SeriesBuffer, offset: usize, pc: usize) -> Result<f64, RuntimeError> {
    match buffer.get(offset) {
        Value::F64(value) => Ok(value),
        Value::NA => Ok(0.0),
        other => Err(RuntimeError::TypeMismatch {
            pc,
            expected: "f64",
            found: other.type_name(),
        }),
    }
}

fn tuple2(first: f64, second: f64) -> Value {
    Value::Tuple2([Box::new(Value::F64(first)), Box::new(Value::F64(second))])
}

fn na_tuple2() -> Value {
    Value::Tuple2([Box::new(Value::NA), Box::new(Value::NA)])
}
