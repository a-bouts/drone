use cu29::prelude::*;
use cu_msp_lib::structs::{MspRequest, MspResponse};
use cu_msp_sink::MspRequestBatch;
use cu_msp_src::MspResponseBatch;

pub struct DroneControl {
    first_run: bool,
    last_tov: CuTime,
}

impl Freezable for DroneControl {}

impl<'cl> CuTask<'cl> for DroneControl {
    type Input = input_msg!('cl, MspResponseBatch);
    type Output = output_msg!('cl, MspRequestBatch);

    fn new(_config: Option<&ComponentConfig>) -> CuResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            first_run: true,
            last_tov: CuTime::default(),
        })
    }

    fn process(
        &mut self,
        _clock: &RobotClock,
        input: Self::Input,
        output: Self::Output,
    ) -> CuResult<()> {

        let mut batch = MspRequestBatch::new();

        let mut cell_voltage = 0.0;

        let maybe_batch = input;
        if let Some(batch) = maybe_batch.payload() {
            for response in batch.0.iter() {
                match response {
                    // MspResponse::MspBatteryState(state) => {
                    //     cell_voltage = (state.battery_voltage as f32/ state.battery_cell_count as f32) / 10.0;
                    //     println!("Battery: {:3} ---", cell_voltage);
                    // }
                    // MspResponse::MspRawImu(raw) => {
                    //     println!("Raw: {:?} ---", raw);
                    // }
                    // MspResponse::MspRc(rc) => {
                    //     println!("Rc: {:?} ---", rc);
                    // }
                    res => {
                        println!("Res: {:?}", res);
                    }
                }
            }
        }

        batch.push(MspRequest::MspRawImu);
        batch.push(MspRequest::MspRc);
        batch.push(MspRequest::MspBatteryState);
        output.set_payload(batch);
        Ok(()) // outputs another message for downstream
    }
}