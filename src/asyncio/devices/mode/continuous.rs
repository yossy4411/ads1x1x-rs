//! Continuous measurement mode.

use crate::{
    conversion, mode, Ads1x1x, ChannelId, Error, ModeChangeError, Register,
};
use crate::asyncio::devices::OperatingMode;
use core::marker::PhantomData;

impl<I2C, IC, CONV, E> Ads1x1x<I2C, IC, CONV, mode::Continuous>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
    CONV: conversion::ConvertMeasurement,
{
    /// Changes to one-shot operating mode.
    pub async fn into_one_shot(
        mut self,
    ) -> Result<Ads1x1x<I2C, IC, CONV, mode::OneShot>, ModeChangeError<E, Self>> {
        if let Err(Error::I2C(e)) = self.set_operating_mode(OperatingMode::OneShot).await {
            return Err(ModeChangeError::I2C(e, self));
        }
        Ok(Ads1x1x {
            i2c: self.i2c,
            address: self.address,
            config: self.config,
            fsr: self.fsr,
            a_conversion_was_started: false,
            _conv: PhantomData,
            _ic: PhantomData,
            _mode: PhantomData,
        })
    }

    /// Reads the most recent measurement.
    pub async fn read(&mut self) -> Result<i16, Error<E>> {
        let value = self.read_register(Register::CONVERSION).await?;
        Ok(CONV::convert_measurement(value))
    }

    /// Selects the channel used for measurements.
    ///
    /// Note that when changing the channel in continuous conversion mode, the
    /// ongoing conversion will be completed.
    /// The following conversions will use the new channel configuration.
    #[allow(unused_variables)]
    pub async fn select_channel<CH: ChannelId<Self>>(&mut self, channel: CH) -> Result<(), Error<E>> {
        let config = self.config.with_mux_bits(CH::channel_id());
        self.write_register(Register::CONFIG, config.bits).await?;
        self.config = config;
        Ok(())
    }
}
