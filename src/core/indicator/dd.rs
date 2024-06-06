use std::fmt::Debug;

use super::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::core::{Error, OHLCV};


//trait CloneIndicatorInstanceDyn<T: Clone + Sized + OHLCV>: Clone + IndicatorInstanceDyn<T>
//{
//}

/// Dynamically dispatchable [`IndicatorConfig`](crate::core::IndicatorConfig)
pub trait IndicatorConfigDyn<T>
where
 T: OHLCV + Clone + Debug,
{

	/// Dynamically initializes the **State** based on the current **Configuration**
	fn init(&self, initial_value: &T) -> Result<Box<dyn  IndicatorInstanceDyn<T> + Send + Sync >, Error>;

	/// Evaluates dynamically dispatched [`IndicatorConfig`](crate::core::IndicatorConfig)  over series of OHLC and returns series of `IndicatorResult`s
	/// ```
	/// use yata::prelude::dd::*;
	/// use yata::helpers::{RandomCandles};
	/// use yata::indicators::Trix;
	///
	/// let candles: Vec<_> = RandomCandles::new().take(10).collect();
	/// let static_config = Trix::default();
	/// let dyn_config: Box<dyn IndicatorConfigDyn<_>> = Box::new(static_config); // here we are loosing information about `IndicatorConfig`s type.
	/// let results = dyn_config.over(&candles).unwrap();
	/// println!("{:?}", results);
	/// ```
	fn over(&self, inputs: &dyn AsRef<[T]>) -> Result<Vec<IndicatorResult>, Error>;

	/// Returns a name of the indicator
	fn name(&self) -> &'static str;

	/// Validates if **Configuration** is OK
	fn validate(&self) -> bool;

	/// Dynamically sets **Configuration** parameters
	fn set(&mut self, name: &str, value: String) -> Result<(), Error>;

	/// Returns an [`IndicatorResult`](crate::core::IndicatorResult) size processing by the indicator `(count of raw values, count of signals)`
	fn size(&self) -> (u8, u8);
}

impl<T, I, C> IndicatorConfigDyn<T> for C
where
	T: OHLCV + Clone + Debug,
	I: IndicatorInstanceDyn<T> + IndicatorInstance<Config = Self> + Sync + Send + Clone + 'static,
	C: IndicatorConfig<Instance = I> + Clone + 'static,
{
	fn init(&self, initial_value: &T) -> Result<Box<dyn IndicatorInstanceDyn<T> + Send + Sync>, Error> {
		let instance = IndicatorConfig::init(self.clone(), initial_value)?;
		Ok(Box::new(instance))
	}

	fn over(&self, inputs: &dyn AsRef<[T]>) -> Result<Vec<IndicatorResult>, Error> {
		IndicatorConfig::over(self.clone(), inputs)
	}

	fn name(&self) -> &'static str {
		<Self as IndicatorConfig>::NAME
	}

	fn validate(&self) -> bool {
		IndicatorConfig::validate(self)
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		IndicatorConfig::set(self, name, value)
	}

	fn size(&self) -> (u8, u8) {
		IndicatorConfig::size(self)
	}
}

/// Dynamically dispatchable [`IndicatorInstance`](crate::core::IndicatorInstance)
pub trait IndicatorInstanceDyn<T: OHLCV + Sized + Clone>
{
	/// Evaluates given candle and returns [`IndicatorResult`](crate::core::IndicatorResult)


	fn next(&mut self, candle: &T) -> IndicatorResult;

	/// Evaluates the **State** over the given sequence of candles and returns sequence of `IndicatorResult`s.
	/// ```
	/// use yata::prelude::dd::*;
	/// use yata::helpers::{RandomCandles};
	/// use yata::indicators::Trix;
	///
	/// let candles: Vec<_> = RandomCandles::new().take(10).collect();
	/// let static_config = Trix::default();
	/// let dyn_config: Box<dyn IndicatorConfigDyn<_>> = Box::new(static_config); // here we are loosing information about `IndicatorConfig`s type.
	/// let mut state = dyn_config.init(&candles[0]).unwrap();
	///
	/// let results = state.over(&candles);
	/// let foo = results.clone();
	/// println!("{:?}", foo);
	/// println!("{:?}", results);
	/// ```
	fn over(&mut self, inputs: &dyn AsRef<[T]>) -> Vec<IndicatorResult>;

	/// Returns a reference to dynamically dispatched **Configuration**, associated with the current **State**
	/*
	fn config(&self) -> Box<Self::Config>;
	*/
	/// Returns count of indicator's raw values and count of indicator's signals.
	///
	/// See more at [`IndicatorConfig`](crate::core::IndicatorConfig::size)
	fn size(&self) -> (u8, u8);

	/// Returns a name of the indicator
	fn name(&self) -> &'static str;
}

impl<T, I> IndicatorInstanceDyn<T> for I
where
	T: OHLCV + Sized + Clone,
	I: IndicatorInstance + Send + Sync + Clone + 'static,
{

	fn next(&mut self, candle: &T) -> IndicatorResult {
		IndicatorInstance::next(self, candle)
	}

	fn over(&mut self, inputs: &dyn AsRef<[T]>) -> Vec<IndicatorResult> {
		IndicatorInstance::over(self, inputs)
	}

	/*
	fn config(&self) -> Box<dyn IndicatorConfigDyn<T>> {
		self.config()
	}*/

	fn size(&self) -> (u8, u8) {
		IndicatorInstance::size(self)
	}

	fn name(&self) -> &'static str {
		IndicatorInstance::name(self)
	}
}
