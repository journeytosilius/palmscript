pub(crate) mod ema;
pub(crate) mod rsi;
pub(crate) mod sma;

pub(crate) use ema::EmaState;
pub(crate) use rsi::RsiState;

#[derive(Clone, Debug)]
pub(crate) enum IndicatorState {
    Ema(EmaState),
    Rsi(RsiState),
}
